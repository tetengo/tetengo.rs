/*!
 * A timetable vocabulary.
 *
 * Copyright (C) 2023-2024 kaoru  <https://www.tetengo.org/>
 */

use std::collections::HashMap;
use std::io::{BufRead, Lines};
use std::rc::Rc;

use anyhow::Result;
use tetengo_lattice::entry::Entry;
use tetengo_lattice::string_input::StringInput;
use tetengo_lattice::vocabulary::Vocabulary;

/**
 * A timetable error.
 */
#[derive(Clone, Copy, Debug, thiserror::Error)]
pub(crate) enum TimetableError {
    /**
     * Unexpected end of file.
     */
    #[error("unexpected end of file")]
    UnexpectedEndOfFile,

    /**
     * Station names and telegram codes unmatch.
     */
    #[error("station names and telegram codes unmatch")]
    StationNamesAndTelegramCodesUnmatch,

    /**
     * Invalid train line found.
     */
    #[error("invalid train line found")]
    InvalidTrainLineFound,

    /**
     * Invalid arrival/departure time found.
     */
    #[error("invalid arrival/departure time found")]
    InvalidArrivalOrDepartureTimeFound,

    /**
     * Invalid time found.
     */
    #[error("invalid time found")]
    InvalidTimeFound,

    /**
     * Both arrival and departure time not found.
     */
    #[error("both arrival and departure time not found")]
    BothArrivalAndDepartureTimeNotFound,
}

/**
 * A station.
 */
pub(crate) struct Station {
    name: String,
    telegram_code: String,
}

impl Station {
    /**
     * Creates a station.
     *
     * # Arguments
     * * `name`          - A name.
     * * `telegram_code` - A telegram code.
     */
    pub(crate) const fn new(name: String, telegram_code: String) -> Self {
        Self {
            name,
            telegram_code,
        }
    }

    /**
     * Returns the name.
     *
     * # Returns
     * The name.
     */
    pub(crate) fn name(&self) -> &str {
        self.name.as_str()
    }

    /**
     * Returns the telegram code.
     *
     * # Returns
     * The telegram code.
     */
    pub(crate) fn telegram_code(&self) -> &str {
        self.telegram_code.as_str()
    }
}

/**
 * A stop.
 */
#[derive(Clone)]
pub(crate) struct Stop {
    arrival_time: Option<usize>,
    departure_time: Option<usize>,
}

impl Stop {
    /**
     * Creates a stop.
     *
     * # Arguments
     * * `arrival_time`   - An arrival time.
     * * `departure_time` - A departure time.
     */
    pub(crate) const fn new(arrival_time: Option<usize>, departure_time: Option<usize>) -> Self {
        Self {
            arrival_time,
            departure_time,
        }
    }

    /**
     * Returns the arrival time.
     *
     * # Returns
     * The arrival time.
     */
    pub(crate) const fn arrival_time(&self) -> Option<usize> {
        self.arrival_time
    }

    /**
     * Sets an arrival time.
     *
     * # Arguments
     * * `time` - An arrival time.
     */
    pub(crate) fn set_arrival_time(&mut self, time: usize) {
        self.arrival_time = Some(time);
    }

    /**
     * Returns the departure time.
     *
     * # Returns
     * The departure time.
     */
    pub(crate) const fn departure_time(&self) -> Option<usize> {
        self.departure_time
    }

    /**
     * Sets a departure time.
     *
     * # Arguments
     * * `time` - A departure time.
     */
    pub(crate) fn set_departure_time(&mut self, time: usize) {
        self.departure_time = Some(time);
    }
}

/**
 * A train.
 */
#[derive(Clone)]
pub(crate) struct Train {
    _number: String,
    _name: String,
    stops: Vec<Stop>,
}

impl Train {
    /**
     * Creates a train.
     *
     * # Arguments
     * * `number` - A number.
     * * `name`   - A name.
     * * `stops`  - Stops.
     */
    pub(crate) const fn new(number: String, name: String, stops: Vec<Stop>) -> Self {
        Self {
            _number: number,
            _name: name,
            stops,
        }
    }

    /**
     * Returns the number.
     *
     * # Returns
     * The number.
     */
    pub(crate) fn _number(&self) -> &str {
        self._number.as_str()
    }

    /**
     * Returns the name.
     *
     * # Returns
     * The name.
     */
    pub(crate) fn _name(&self) -> &str {
        self._name.as_str()
    }

    /**
     * Returns the stops.
     *
     * # Returns
     * The stops.
     */
    pub(crate) fn stops(&self) -> &[Stop] {
        self.stops.as_slice()
    }

    /**
     * Returns the stops.
     *
     * # Returns
     * The stops.
     */
    pub(crate) fn stops_mut(&mut self) -> &mut Vec<Stop> {
        &mut self.stops
    }
}

/**
 * A section.
 */
#[derive(Clone)]
pub(crate) struct Section {
    _train: Rc<Train>,
    _from: usize,
    _to: usize,
}

impl Section {
    /**
     * Creates a section.
     *
     * # Arguments
     * * `train` - A train.
     * * `from`  - A departure station index.
     * * `to`    - An arrival station index.
     */
    pub(crate) const fn new(train: Rc<Train>, from: usize, to: usize) -> Self {
        Self {
            _train: train,
            _from: from,
            _to: to,
        }
    }

    /**
     * Returns the train.
     *
     * # Returns
     * The train.
     */
    pub(crate) fn _train(&self) -> &Train {
        self._train.as_ref()
    }

    /**
     * Returns the departure station index.
     *
     * # Returns
     * The departure station index.
     */
    pub(crate) const fn _from(&self) -> usize {
        self._from
    }

    /**
     * Returns the arrival station index.
     *
     * # Returns
     * The arrival station index.
     */
    pub(crate) const fn _to(&self) -> usize {
        self._to
    }
}

struct TimetableValue {
    stations: Vec<Station>,
    trains: Vec<Train>,
}

impl TimetableValue {
    fn new(stations: Vec<Station>, trains: Vec<Train>) -> Self {
        Self { stations, trains }
    }
}

/**
 * A timetable vocabulary.
 */
pub(crate) struct Timetable {
    value: TimetableValue,
}

impl Timetable {
    /**
     * Creates a timetable vocabulary.
     *
     * # Arguments
     * * `reader` - A reader.
     */
    pub(crate) fn new(reader: Box<dyn BufRead>) -> Result<Self> {
        Ok(Self {
            value: Self::build_timetable(reader)?,
        })
    }

    fn build_timetable(mut reader: Box<dyn BufRead>) -> Result<TimetableValue> {
        let mut value = Self::parse_input(reader.as_mut())?;
        Self::guess_arrival_times(&mut value)?;
        Ok(value)
    }

    fn parse_input(reader: &mut dyn BufRead) -> Result<TimetableValue> {
        let mut lines = reader.lines();

        let stations = {
            let Some(line1) = Self::read_line(&mut lines)? else {
                return Err(TimetableError::UnexpectedEndOfFile.into());
            };
            let Some(line2) = Self::read_line(&mut lines)? else {
                return Err(TimetableError::UnexpectedEndOfFile.into());
            };
            Self::parse_stations(line1, line2)?
        };

        let trains = {
            let mut trains = Vec::new();
            while let Some(line) = Self::read_line(&mut lines)? {
                if line.is_empty() || (line.len() == 1 && line[0].is_empty()) {
                    continue;
                }
                trains.push(Self::parse_train(line, stations.len())?);
            }
            trains
        };

        Ok(TimetableValue::new(stations, trains))
    }

    fn read_line(lines: &mut Lines<&mut dyn BufRead>) -> Result<Option<Vec<String>>> {
        let Some(line) = lines.next() else {
            return Ok(None);
        };
        let line = line?;
        let elements = line
            .split(',')
            .map(|e| e.trim().to_string())
            .collect::<Vec<_>>();
        Ok(Some(elements))
    }

    fn parse_stations(line1: Vec<String>, line2: Vec<String>) -> Result<Vec<Station>> {
        if line1.len() != line2.len() {
            return Err(TimetableError::StationNamesAndTelegramCodesUnmatch.into());
        }
        let stations = line1
            .into_iter()
            .skip(2)
            .zip(line2.into_iter().skip(2))
            .map(|(name, telegram_code)| Station::new(name, telegram_code))
            .collect::<Vec<_>>();
        Ok(stations)
    }

    fn parse_train(mut line: Vec<String>, station_count: usize) -> Result<Train> {
        if line.len() > station_count + 2 {
            return Err(TimetableError::InvalidTrainLineFound.into());
        }
        line.resize(station_count + 2, String::new());
        let number = line[0].clone();
        let name = line[1].clone();
        let stops = line
            .into_iter()
            .skip(2)
            .map(Self::to_stop)
            .collect::<Result<Vec<_>>>()?;
        Ok(Train::new(number, name, stops))
    }

    fn to_stop(element: String) -> Result<Stop> {
        let string_times = element
            .split('/')
            .map(|e| e.trim().to_string())
            .collect::<Vec<_>>();
        if string_times.is_empty() || string_times.len() > 2 {
            Err(TimetableError::InvalidArrivalOrDepartureTimeFound.into())
        } else if string_times.len() == 1 {
            Ok(Stop::new(None, Self::to_minutes(string_times[0].as_str())?))
        } else {
            Ok(Stop::new(
                Self::to_minutes(string_times[0].as_str())?,
                Self::to_minutes(string_times[1].as_str())?,
            ))
        }
    }

    fn to_minutes(string_time: &str) -> Result<Option<usize>> {
        if string_time.is_empty() || string_time == "-" {
            return Ok(None);
        }
        let int_time = string_time.parse::<usize>()?;
        let hour = int_time / 100;
        let minute = int_time - hour * 100;
        if hour >= 24 || minute >= 60 {
            return Err(TimetableError::InvalidTimeFound.into());
        }
        Ok(Some(hour * 60 + minute))
    }

    fn guess_arrival_times(value: &mut TimetableValue) -> Result<()> {
        for from in 0..value.stations.len() - 1 {
            for to in from + 1..value.stations.len() {
                let minimum_duration = Self::minimum_duration(value.trains.as_ref(), from, to)?;
                for train in &mut value.trains {
                    if !Self::all_passing(train.stops(), from, to) {
                        continue;
                    }
                    if train.stops()[to].arrival_time().is_none() {
                        let Some(from_departure_time) = train.stops()[from].departure_time() else {
                            return Err(TimetableError::BothArrivalAndDepartureTimeNotFound.into());
                        };
                        train.stops_mut()[to].set_arrival_time(Self::add_time(
                            from_departure_time,
                            minimum_duration,
                        ));
                    } else if train.stops()[from].departure_time().is_none() {
                        let Some(to_arrival_time) = train.stops()[to].arrival_time() else {
                            return Err(TimetableError::BothArrivalAndDepartureTimeNotFound.into());
                        };
                        train.stops_mut()[from]
                            .set_departure_time(Self::add_time(to_arrival_time, -minimum_duration));
                    }
                }
            }
        }
        Ok(())
    }

    fn minimum_duration(trains: &[Train], from: usize, to: usize) -> Result<isize> {
        let mut minimum = isize::MAX;
        for train in trains {
            if !Self::all_passing(train.stops(), from, to) {
                continue;
            }
            let from_time = if let Some(departure_time) = train.stops()[from].arrival_time() {
                departure_time
            } else if let Some(departure_time) = train.stops()[from].departure_time() {
                departure_time
            } else {
                return Err(TimetableError::BothArrivalAndDepartureTimeNotFound.into());
            };
            let to_time = if let Some(arrival_time) = train.stops()[to].arrival_time() {
                arrival_time
            } else if let Some(arrival_time) = train.stops()[to].departure_time() {
                arrival_time
            } else {
                return Err(TimetableError::BothArrivalAndDepartureTimeNotFound.into());
            };
            if Self::diff_time(to_time, from_time) < minimum {
                minimum = Self::diff_time(to_time, from_time);
            }
        }
        Ok(minimum)
    }

    /**
     * Returns the stations.
     *
     * # Returns
     * The stations.
     */
    pub(crate) fn stations(&self) -> &[Station] {
        self.value.stations.as_slice()
    }

    /**
     * Returns the station index.
     *
     * # Arguments
     * * `name_or_telegram_code` - A name or telegram code.
     *
     * # Returns
     * The index. Or `stations().len()` if no station is found.
     */
    pub(crate) fn station_index(&self, name_or_telegram_code: &str) -> usize {
        for (i, station) in self.value.stations.iter().enumerate() {
            if station.name().to_lowercase() == name_or_telegram_code.to_lowercase()
                || station.telegram_code().to_uppercase() == name_or_telegram_code.to_uppercase()
            {
                return i;
            }
        }
        self.value.stations.len()
    }

    /**
     * Creates a vocabulary.
     *
     * # Arguments
     * * `departure_time` - A departure time.
     *
     * # Returns
     * A vocabulary.
     */
    pub(crate) fn create_vocabulary(&self, _departure_time: usize) -> Box<dyn Vocabulary> {
        let _entries = Self::build_entries(&self.value);
        todo!()
    }

    /*
        /*!
            \brief Creates a vocabulary.

            \param departure_time A departure time.

            \return A vocabulary.
        */
        [[nodiscard]] std::unique_ptr<tetengo::lattice::vocabulary> create_vocabulary(std::size_t departure_time) const;
    */
    /*
        std::unique_ptr<tetengo::lattice::vocabulary> create_vocabulary(const std::size_t departure_time) const
        {
            auto entries = build_entries(m_timetable);
            auto connections = build_connections(entries, departure_time);
            return std::make_unique<tetengo::lattice::unordered_map_vocabulary>(
                std::move(entries), std::move(connections), entry_hash, entry_equal_to);
        }
    */

    fn build_entries(timetable: &TimetableValue) -> Vec<(String, Vec<Entry>)> {
        let mut map = HashMap::<String, Vec<Entry>>::new();
        for train in &timetable.trains {
            for from in 0..timetable.stations.len() - 1 {
                for to in from + 1..timetable.stations.len() {
                    if !Self::all_passing(train.stops(), from, to) {
                        continue;
                    }

                    let section_name = Self::make_section_name(&timetable.stations, from, to);
                    let found = map.entry(section_name.clone()).or_default();
                    let section = Section::new(Rc::new(train.clone()), from, to);
                    found.push(Entry::new(
                        Box::new(StringInput::new(section_name)),
                        Box::new(section),
                        Self::make_section_duration(train.stops(), from, to) as i32,
                    ));
                }
            }
        }
        todo!()
    }
    /*
        static std::vector<std::pair<std::string, std::vector<tetengo::lattice::entry>>>
        build_entries(const timetable_value& timetable_)
        {
            std::unordered_map<std::string, std::vector<tetengo::lattice::entry>> map{};
            for (const auto& train_: timetable_.trains)
            {
                for (auto from = static_cast<std::size_t>(0); from + 1 < std::size(timetable_.stations); ++from)
                {
                    for (auto to = from + 1; to < std::size(timetable_.stations); ++to)
                    {
                        if (!all_passing(train_.stops(), from, to))
                        {
                            continue;
                        }

                        auto section_name = make_section_name(timetable_.stations, from, to);
                        auto found = map.find(section_name);
                        if (found == std::end(map))
                        {
                            auto inserted =
                                map.insert(std::make_pair(section_name, std::vector<tetengo::lattice::entry>{}));
                            found = inserted.first;
                        }
                        section    section_{ &train_, from, to };
                        const auto section_duration = make_section_duration(train_.stops(), from, to);
                        found->second.emplace_back(
                            std::make_unique<tetengo::lattice::string_input>(std::move(section_name)),
                            std::move(section_),
                            static_cast<int>(section_duration));
                    }
                }
            }

            std::vector<std::pair<std::string, std::vector<tetengo::lattice::entry>>> entries{};
            entries.reserve(std::size(map));
            for (auto& map_entry: map)
            {
                entries.emplace_back(map_entry.first, std::move(map_entry.second));
            }
            return entries;
        }
    */

    fn all_passing(stops: &[Stop], from: usize, to: usize) -> bool {
        if stops[from].arrival_time().is_none() && stops[from].departure_time().is_none() {
            return false;
        }
        if stops[to].arrival_time().is_none() && stops[to].departure_time().is_none() {
            return false;
        }
        for stop in stops.iter().take(to).skip(from + 1) {
            if stop.arrival_time().is_some() || stop.departure_time().is_some() {
                return false;
            }
        }
        true
    }

    fn make_section_name(stations: &[Station], from: usize, to: usize) -> String {
        let mut name = String::new();
        for i in from..to {
            name += &format!(
                "{}-{}/",
                stations[i].telegram_code(),
                stations[i + 1].telegram_code()
            );
        }
        name
    }
    /*
        static std::string
        make_section_name(const std::vector<station>& stations, const std::size_t from, const std::size_t to)
        {
            std::string name;
            for (auto i = from; i + 1 <= to; ++i)
            {
                name += stations[i].telegram_code() + "-" + stations[i + 1].telegram_code() + "/";
            }
            return name;
        }
    */

    fn make_section_duration(stops: &[Stop], from: usize, to: usize) -> usize {
        let departure_time = stops[from].departure_time().unwrap_or_else(|| {
            unreachable!("departure_time must be set.");
        });
        let arrival_time = stops[to].arrival_time().unwrap_or_else(|| {
            unreachable!("arrival_time must be set.");
        });
        Self::diff_time(arrival_time, departure_time) as usize
    }
    /*
        static std::size_t
        make_section_duration(const std::vector<stop>& stops, const std::size_t from, const std::size_t to)
        {
            assert(stops[from].departure_time());
            assert(stops[to].arrival_time());
            return diff_time(*stops[to].arrival_time(), *stops[from].departure_time());
        }
    */

    /*
        static std::vector<std::pair<std::pair<tetengo::lattice::entry, tetengo::lattice::entry>, int>> build_connections(
            const std::vector<std::pair<std::string, std::vector<tetengo::lattice::entry>>>& entries,
            const std::size_t                                                                departure_time)
        {
            std::vector<std::pair<std::pair<tetengo::lattice::entry, tetengo::lattice::entry>, int>> connections{};

            for (const auto& from: entries)
            {
                for (const auto& to: entries)
                {
                    for (const auto& from_entry: from.second)
                    {
                        for (const auto& to_entry: to.second)
                        {
                            const auto* const p_from_value = std::any_cast<section>(&from_entry.value());
                            const auto* const p_to_value = std::any_cast<section>(&to_entry.value());
                            if (p_from_value->to() != p_to_value->from())
                            {
                                continue;
                            }

                            const auto from_arrival_time =
                                p_from_value->p_train()->stops()[p_from_value->to()].arrival_time();
                            const auto to_departure_time =
                                p_to_value->p_train()->stops()[p_to_value->from()].departure_time();
                            assert(from_arrival_time);
                            assert(to_departure_time);
                            auto cost = static_cast<int>(diff_time(*to_departure_time, *from_arrival_time));
                            if (cost > 60)
                            {
                                continue;
                            }
                            if (p_from_value->p_train()->number() != p_to_value->p_train()->number())
                            {
                                cost += 1;
                            }

                            connections.emplace_back(std::make_pair(from_entry, to_entry), cost);
                        }
                    }
                }
            }

            for (const auto& key_and_entries: entries)
            {
                for (const auto& entry: key_and_entries.second)
                {
                    const auto* const p_section = std::any_cast<section>(&entry.value());
                    const auto        section_departure_time =
                        p_section ? *p_section->p_train()->stops()[p_section->from()].departure_time() : 0;
                    if (const auto bos_cost = static_cast<int>(diff_time(section_departure_time, departure_time));
                        bos_cost <= 240)
                    {
                        connections.emplace_back(std::make_pair(tetengo::lattice::entry::bos_eos(), entry), bos_cost);
                    }
                    connections.emplace_back(std::make_pair(entry, tetengo::lattice::entry::bos_eos()), 0);
                }
            }

            return connections;
        }
    */

    fn add_time(time: usize, duration: isize) -> usize {
        assert!(time < 1440);
        assert!(-1440 < duration && duration < 1440);
        (time as isize + 1440 + duration) as usize % 1440
    }

    fn diff_time(time1: usize, time2: usize) -> isize {
        assert!(time1 < 1440);
        assert!(time2 < 1440);
        (time1 as isize + 1440 - time2 as isize) % 1440
    }
}

/*
    std::size_t entry_hash(const tetengo::lattice::entry_view& entry)
    {
        const std::size_t key_hash = entry.p_key() ? entry.p_key()->hash_value() : 0;
        std::size_t       entry_train_number_hash = std::hash<std::string_view>{}(std::string_view{});
        std::size_t       entry_train_name_hash = std::hash<std::string_view>{}(std::string_view{});
        std::size_t       entry_from_hash = std::hash<std::size_t>{}(0);
        std::size_t       entry_to_hash = std::hash<std::size_t>{}(0);
        if (entry.value()->has_value())
        {
            if (const auto* const p_section = std::any_cast<section>(entry.value()); p_section)
            {
                entry_train_number_hash = std::hash<std::string>{}(p_section->p_train()->number());
                entry_train_name_hash = std::hash<std::string>{}(p_section->p_train()->name());
                entry_from_hash = std::hash<std::size_t>{}(p_section->from());
                entry_to_hash = std::hash<std::size_t>{}(p_section->to());
            }
        }
        return key_hash ^ entry_train_number_hash ^ entry_train_name_hash ^ entry_from_hash ^ entry_to_hash;
    }
*/
/*
    bool entry_equal_to(const tetengo::lattice::entry_view& one, const tetengo::lattice::entry_view& another)
    {
        if (one.value()->has_value() && another.value()->has_value())
        {
            const auto* const p_one_section = std::any_cast<section>(one.value());
            const auto* const p_another_section = std::any_cast<section>(another.value());
            if (p_one_section && p_another_section)
            {
                return ((!one.p_key() && !another.p_key()) ||
                        (one.p_key() && another.p_key() && *one.p_key() == *another.p_key())) &&
                       p_one_section->p_train()->number() == p_another_section->p_train()->number() &&
                       p_one_section->p_train()->name() == p_another_section->p_train()->name() &&
                       p_one_section->from() == p_another_section->from() &&
                       p_one_section->to() == p_another_section->to();
            }
            else
            {
                assert(false);
                throw std::logic_error{ "Unexpected entry value." };
            }
        }
        else if (one.value()->has_value() || another.value()->has_value())
        {
            return false;
        }
        else
        {
            return (!one.p_key() && !another.p_key()) ||
                   (one.p_key() && another.p_key() && *one.p_key() == *another.p_key());
        }
    }


}
*/
