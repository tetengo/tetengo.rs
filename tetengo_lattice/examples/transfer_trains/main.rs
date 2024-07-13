/*!
 * A train transfer guide.
 *
 * Copyright (C) 2023-2024 kaoru  <https://www.tetengo.org/>
 */

mod timetable;

use std::env;
use std::fs::File;
use std::io::{stdin, stdout, BufRead, BufReader, Lines, StdinLock, Write};
use std::path::Path;
use std::process::exit;

use anyhow::Result;
use timetable::Timetable;

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
        let _departure_and_arrival = match get_departure_and_arrival(&mut lines, &timetable)? {
            Input::DepartureAndArrival(Some(value)) => value,
            Input::DepartureAndArrival(None) => continue,
            Input::Eof => break,
        };
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

/*
    void build_lattice(
        const std::pair<std::pair<std::size_t, std::size_t>, std::size_t>& departure_and_arrival,
        const timetable&                                                   timetable_,
        tetengo::lattice::lattice&                                         lattice_)
    {
        for (auto i = departure_and_arrival.first.first; i < departure_and_arrival.second; ++i)
        {
            auto key =
                timetable_.stations()[i].telegram_code() + "-" + timetable_.stations()[i + 1].telegram_code() + "/";
            auto p_input = std::make_unique<tetengo::lattice::string_input>(std::move(key));
            lattice_.push_back(std::move(p_input));
        }
    }
*/
/*
    struct trip_section
    {
        std::string_view train_number;

        std::string_view train_name;

        std::size_t departure_time;

        std::size_t departure_station;

        std::size_t arrival_time;

        std::size_t arrival_station;
    };
*/
/*
    struct trip
    {
        std::vector<trip_section> sections;

        int cost;
    };
*/
/*
    std::vector<trip> enumerate_trips(
        const tetengo::lattice::lattice&                                            lattice_,
        const std::pair<tetengo::lattice::node, std::unique_ptr<std::vector<int>>>& eos_and_precedings,
        const std::size_t                                                           trip_capacity)
    {
        tetengo::lattice::n_best_iterator       iter{ lattice_,
                                                eos_and_precedings.first,
                                                std::make_unique<tetengo::lattice::constraint>() };
        const tetengo::lattice::n_best_iterator last{};
        std::vector<trip>                       trips{};
        trips.reserve(trip_capacity);
        std::unordered_set<std::string_view> duplication_checker{};
        for (; std::size(trips) < trip_capacity && iter != last; ++iter)
        {
            const auto& path = *iter;
            if (path.cost() >= 1440)
            {
                break;
            }

            trip trip_{};
            for (const auto& node: path.nodes())
            {
                const auto* const p_section = std::any_cast<section>(&node.value());
                if (!p_section)
                {
                    continue;
                }

                if (std::empty(trip_.sections) || trip_.sections.back().train_number != p_section->p_train()->number())
                {
                    trip_.sections.push_back({ p_section->p_train()->number(),
                                               p_section->p_train()->name(),
                                               *p_section->p_train()->stops()[p_section->from()].departure_time(),
                                               p_section->from(),
                                               *p_section->p_train()->stops()[p_section->to()].arrival_time(),
                                               p_section->to() });
                }
                else
                {
                    trip_.sections.back().arrival_time = *p_section->p_train()->stops()[p_section->to()].arrival_time();
                    trip_.sections.back().arrival_station = p_section->to();
                }
            }
            trip_.cost = path.cost();

            if (duplication_checker.find(trip_.sections.front().train_number) != std::end(duplication_checker) ||
                duplication_checker.find(trip_.sections.back().train_number) != std::end(duplication_checker))
            {
                continue;
            }

            duplication_checker.insert(trip_.sections.front().train_number);
            duplication_checker.insert(trip_.sections.back().train_number);
            trips.push_back(std::move(trip_));
        }

        return trips;
    }
*/
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
