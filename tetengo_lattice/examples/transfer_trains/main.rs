/*!
 * A train transfer guide.
 *
 * Copyright (C) 2023-2024 kaoru  <https://www.tetengo.org/>
 */

mod timetable;

use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{stdin, stdout, BufRead, BufReader, Lines, StdinLock, Write};
use std::path::Path;
use std::process::exit;

use anyhow::Result;
use tetengo_lattice::{Constraint, Lattice, NBestIterator, Node, StringInput};
use timetable::{Section, Timetable};

fn main() {
    if let Err(e) = main_core() {
        eprintln!("Error: {}", e);
        exit(1);
    }
}

fn main_core() -> Result<()> {
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
        let (eos_node, _) = lattice.settle()?;

        let _trips = enumerate_trips(&lattice, eos_node, 5);
    }

    Ok(())
}
/*
int main(const int argc, char** const argv)
{
    try
    {
        std::locale::global(std::locale{ "" });

        if (argc <= 1)
        {
            std::cerr << "Usage: transfer_trains timetable.txt" << std::endl;
            return 0;
        }

        const timetable timetable_{ create_input_stream(argv[1]) };

        while (std::cin)
        {
            const auto departure_and_arrival = get_departure_and_arrival(timetable_);
            if (!departure_and_arrival)
            {
                continue;
            }

            const auto                p_vocabulary = timetable_.create_vocabulary(departure_and_arrival->first.second);
            tetengo::lattice::lattice lattice_{ *p_vocabulary };
            build_lattice(*departure_and_arrival, timetable_, lattice_);
            const auto eos_and_precedings = lattice_.settle();

            const auto trips = enumerate_trips(lattice_, eos_and_precedings, 5);

            print_trips(trips, timetable_);
        }
        return 0;
    }
    catch (const std::exception& e)
    {
        std::cerr << "Error: " << e.what() << std::endl;
        return 1;
    }
    catch (...)
    {
        std::cerr << "Error: unknown error." << std::endl;
        return 1;
    }
}
 */

fn create_reader(path: &Path) -> Result<Box<dyn BufRead>> {
    let reader = BufReader::new(File::open(path)?);
    Ok(Box::new(reader))
}

enum Input {
    DepartureAndArrival(Option<((usize, usize), usize)>),
    Eof,
}

fn get_departure_and_arrival(
    lines: &mut Lines<StdinLock<'_>>,
    timetable: &Timetable,
) -> Result<Input> {
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
) -> Result<Option<usize>> {
    print!("{}: ", prompt);
    stdout().flush()?;
    let Some(input) = lines.next() else {
        return Ok(None);
    };
    let input = input?;
    Ok(Some(timetable.station_index(input.trim())))
}

fn get_time(prompt: &str, lines: &mut Lines<StdinLock<'_>>) -> Result<Option<usize>> {
    print!("{}: ", prompt);
    stdout().flush()?;
    let Some(input) = lines.next() else {
        return Ok(None);
    };
    let input = input?;

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
) -> Result<()> {
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

struct TripSection {
    pub(crate) train_number: String,
    pub(crate) _train_name: String,
    pub(crate) _departure_time: usize,
    pub(crate) _departure_station: usize,
    pub(crate) arrival_time: usize,
    pub(crate) arrival_station: usize,
}

struct Trip {
    pub(crate) sections: Vec<TripSection>,
    pub(crate) cost: i32,
}

fn enumerate_trips(lattice: &Lattice<'_>, eos_node: Node<'_>, trip_capacity: usize) -> Vec<Trip> {
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
                if let Some(section) = node_value.as_any().downcast_ref::<Section>() {
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
                    _train_name: section.train().name().to_string(),
                    _departure_time: section.train().stops()[section.from()]
                        .departure_time()
                        .unwrap_or_else(|| unreachable!("departure_time must not None.")),
                    _departure_station: section.from(),
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

/*
    std::string to_fixed_width_train_name(const std::string_view& train_name, const std::size_t width)
    {
        const tetengo::text::grapheme_splitter grapheme_splitter_{};
        const auto                             graphemes = grapheme_splitter_.split(train_name);
        const auto                             train_name_width = std::accumulate(
            std::begin(graphemes),
            std::end(graphemes),
            static_cast<std::size_t>(0),
            [](const std::size_t subtotal, const tetengo::text::grapheme& grapheme) {
                return subtotal + grapheme.width();
            });
        if (train_name_width >= width)
        {
            return std::string{ train_name };
        }
        else
        {
            return std::string{ train_name } + std::string(width - train_name_width, ' ');
        }
    }
*/
/*
    std::string to_time_string(const int time_value)
    {
        assert(0 <= time_value && time_value < 1440);
        return (boost::format{ "%02d:%02d" } % (time_value / 60) % (time_value % 60)).str();
    }
*/
/*
    void print_trips(const std::vector<trip>& trips, const timetable& timetable_)
    {
        for (auto i = static_cast<std::size_t>(0); i < std::size(trips); ++i)
        {
            const auto& trip_ = trips[i];

            std::cout << boost::format("[%d] Cost: %d") % (i + 1) % trip_.cost << std::endl;

            for (const auto& section: trip_.sections)
            {
                const auto train = boost::format("    %5s %s %5s->%5s %s->%s") % section.train_number %
                                   to_fixed_width_train_name(section.train_name, 40) %
                                   to_time_string(static_cast<int>(section.departure_time)) %
                                   to_time_string(static_cast<int>(section.arrival_time)) %
                                   timetable_.stations()[section.departure_station].name() %
                                   timetable_.stations()[section.arrival_station].name();
                std::cout << encode_for_print(train.str()) << std::endl;
            }
        }

        std::cout << "--------------------------------" << std::endl;
    }


}
*/
