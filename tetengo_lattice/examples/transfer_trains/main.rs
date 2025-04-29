/*!
 * A train transfer guide.
 *
 * Copyright (C) 2023-2025 kaoru  <https://www.tetengo.org/>
 */

mod timetable;

use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines, StdinLock, Write, stdin, stdout};
use std::path::Path;
use std::process::exit;

use timetable::TimetableError;
use unicode_width::UnicodeWidthStr;

use tetengo_lattice::{Constraint, Error, Lattice, NBestIterator, Node, StringInput};

use crate::timetable::{Section, Timetable};

fn main() {
    if let Err(e) = main_core() {
        eprintln!("Error: {}", e);
        exit(1);
    }
}

fn main_core() -> Result<(), TimetableError> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() <= 1 {
        eprintln!("Usage: transfer_trains timetable.txt");
        return Ok(());
    }

    let timetable = Timetable::new(create_reader(Path::new(&args[1]))?)?;

    let mut lines = stdin().lines();
    loop {
        let departure_and_arrival = match get_departure_and_arrival(&mut lines, &timetable)? {
            Input::DepartureAndArrival(Some(value)) => value,
            Input::DepartureAndArrival(None) => continue,
            Input::Eof => break,
        };

        let ((_, departure_time), _) = departure_and_arrival;
        let vocabulary = timetable.create_vocabulary(departure_time);
        let mut lattice = Lattice::new(vocabulary.as_ref());
        build_lattice(departure_and_arrival, &timetable, &mut lattice)?;
        let eos_node = lattice.settle()?;

        let trips = enumerate_trips(&lattice, eos_node, 5);

        print_trips(&trips, &timetable);
    }

    Ok(())
}

fn create_reader(path: &Path) -> Result<Box<dyn BufRead>, Error> {
    let reader = BufReader::new(File::open(path).map_err(|e| Error::InternalError(Box::new(e)))?);
    Ok(Box::new(reader))
}

enum Input {
    DepartureAndArrival(Option<((usize, usize), usize)>),
    Eof,
}

fn get_departure_and_arrival(
    lines: &mut Lines<StdinLock<'_>>,
    timetable: &Timetable,
) -> Result<Input, Error> {
    let Some(departure_station_index) = get_station_index("Departure Station", lines, timetable)?
    else {
        return Ok(Input::Eof);
    };
    if departure_station_index >= timetable.stations().len() {
        println!("No departure station is found.");
        return Ok(Input::DepartureAndArrival(None));
    }

    let Some(departure_time) = get_time("Departure Time", lines)? else {
        return Ok(Input::Eof);
    };
    if departure_time >= 1440 {
        println!("Wrong time format.");
        return Ok(Input::DepartureAndArrival(None));
    }

    let Some(arrival_station_index) = get_station_index("Arrival Station", lines, timetable)?
    else {
        return Ok(Input::Eof);
    };
    if arrival_station_index >= timetable.stations().len() {
        println!("No arrival station is found.");
        return Ok(Input::DepartureAndArrival(None));
    };

    Ok(Input::DepartureAndArrival(Some((
        (departure_station_index, departure_time),
        arrival_station_index,
    ))))
}

fn get_station_index(
    prompt: &str,
    lines: &mut Lines<StdinLock<'_>>,
    timetable: &Timetable,
) -> Result<Option<usize>, Error> {
    print!("{}: ", prompt);
    stdout()
        .flush()
        .map_err(|e| Error::InternalError(Box::new(e)))?;
    let Some(input) = lines.next() else {
        return Ok(None);
    };
    let input = input.map_err(|e| Error::InternalError(Box::new(e)))?;
    Ok(Some(timetable.station_index(input.trim())))
}

fn get_time(prompt: &str, lines: &mut Lines<StdinLock<'_>>) -> Result<Option<usize>, Error> {
    print!("{}: ", prompt);
    stdout()
        .flush()
        .map_err(|e| Error::InternalError(Box::new(e)))?;
    let Some(input) = lines.next() else {
        return Ok(None);
    };
    let input = input.map_err(|e| Error::InternalError(Box::new(e)))?;

    let elements = input.split(':').collect::<Vec<_>>();
    if elements.len() != 2 {
        return Ok(Some(1440));
    }
    let Ok(hour) = elements[0].parse::<usize>() else {
        return Ok(Some(1440));
    };
    let Ok(minute) = elements[1].parse::<usize>() else {
        return Ok(Some(1440));
    };
    if hour >= 24 || minute >= 60 {
        return Ok(Some(1440));
    }
    Ok(Some(hour * 60 + minute))
}

fn build_lattice(
    ((departure_station_index, _), arrival_station_index): ((usize, usize), usize),
    timetable: &Timetable,
    lattice: &mut Lattice<'_>,
) -> Result<(), Error> {
    for i in departure_station_index..arrival_station_index {
        let key = format!(
            "{}-{}/",
            timetable.stations()[i].telegram_code(),
            timetable.stations()[i + 1].telegram_code()
        );
        let input = Box::new(StringInput::new(key));
        lattice.push_back(input)?;
    }
    Ok(())
}

#[derive(Debug)]
struct TripSection {
    pub(crate) train_number: String,
    pub(crate) train_name: String,
    pub(crate) departure_time: usize,
    pub(crate) departure_station: usize,
    pub(crate) arrival_time: usize,
    pub(crate) arrival_station: usize,
}

#[derive(Debug)]
struct Trip {
    pub(crate) sections: Vec<TripSection>,
    pub(crate) cost: i32,
}

fn enumerate_trips(lattice: &Lattice<'_>, eos_node: Node, trip_capacity: usize) -> Vec<Trip> {
    let iter = NBestIterator::new(lattice, eos_node, Box::new(Constraint::new()));
    let mut trips = Vec::with_capacity(trip_capacity);
    let mut duplication_checker = HashSet::<String>::new();
    for path in iter {
        if trips.len() >= trip_capacity || path.cost() >= 1440 {
            break;
        }

        let mut trip = Trip {
            sections: Vec::new(),
            cost: 0,
        };
        for node in path.nodes() {
            let section = if let Some(node_value) = node.value() {
                if let Some(section) = node_value.downcast_ref::<Section>() {
                    section
                } else {
                    continue;
                }
            } else {
                continue;
            };

            if trip.sections.is_empty()
                || trip
                    .sections
                    .last()
                    .unwrap_or_else(|| unreachable!("trip.sections must not empty."))
                    .train_number
                    != section.train().number()
            {
                trip.sections.push(TripSection {
                    train_number: section.train().number().to_string(),
                    train_name: section.train().name().to_string(),
                    departure_time: section.train().stops()[section.from()]
                        .departure_time()
                        .unwrap_or_else(|| unreachable!("departure_time must not None.")),
                    departure_station: section.from(),
                    arrival_time: section.train().stops()[section.to()]
                        .arrival_time()
                        .unwrap_or_else(|| unreachable!("arrival_time must not None.")),
                    arrival_station: section.to(),
                });
            } else {
                let last_section = trip
                    .sections
                    .last_mut()
                    .unwrap_or_else(|| unreachable!("trip.sections must not empty."));
                last_section.arrival_time = section.train().stops()[section.to()]
                    .arrival_time()
                    .unwrap_or_else(|| unreachable!("arrival_time must not None."));
                last_section.arrival_station = section.to();
            }
        }
        trip.cost = path.cost();

        let first_section = trip
            .sections
            .first()
            .unwrap_or_else(|| unreachable!("trip.sections must not empty."));
        let last_section = trip
            .sections
            .last()
            .unwrap_or_else(|| unreachable!("trip.sections must not empty."));
        if duplication_checker.contains(&first_section.train_number)
            || duplication_checker.contains(&last_section.train_number)
        {
            continue;
        }
        let _ = duplication_checker.insert(first_section.train_number.clone());
        let _ = duplication_checker.insert(last_section.train_number.clone());
        trips.push(trip);
    }
    trips
}

fn print_trips(trips: &[Trip], timetable: &Timetable) {
    for (i, trip) in trips.iter().enumerate() {
        println!("[{}] Cost: {}", i + 1, trip.cost);

        for section in &trip.sections {
            let train = format!(
                "    {:5} {} {:5}->{:5} {}->{}",
                section.train_number,
                to_fixed_width_train_name(&section.train_name, 40),
                to_time_string(section.departure_time),
                to_time_string(section.arrival_time),
                timetable.stations()[section.departure_station].name(),
                timetable.stations()[section.arrival_station].name()
            );
            println!("{}", train);
        }
    }
    println!("--------------------------------");
}

fn to_fixed_width_train_name(train_name: &str, width: usize) -> String {
    let train_name_with = train_name.width_cjk();
    if train_name_with >= width {
        train_name.to_string()
    } else {
        format!("{}{}", train_name, " ".repeat(width - train_name_with))
    }
}

fn to_time_string(time_value: usize) -> String {
    assert!(time_value < 1440);
    format!("{:02}:{:02}", time_value / 60, time_value % 60)
}
