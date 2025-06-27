/*!
 * A timetable vocabulary.
 *
 * Copyright (C) 2023-2025 kaoru  <https://www.tetengo.org/>
 */

use std::collections::HashMap;
use std::error;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::{BufRead, Lines};
use std::rc::Rc;

use tetengo_lattice::{Entry, HashMapVocabulary, StringInput, Vocabulary};

/**
 * A timetable error.
 */
#[derive(Debug, thiserror::Error)]
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

    /**
     * An error returned from an internal crate.
     */
    #[error("internal error: {0}")]
    InternalError(#[from] Box<dyn error::Error>),
}

/**
 * A station.
 */
#[derive(Debug)]
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
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
pub(crate) struct Train {
    number: String,
    name: String,
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
            number,
            name,
            stops,
        }
    }

    /**
     * Returns the number.
     *
     * # Returns
     * The number.
     */
    pub(crate) fn number(&self) -> &str {
        self.number.as_str()
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
#[derive(Clone, Debug)]
pub(crate) struct Section {
    train: Rc<Train>,
    from: usize,
    to: usize,
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
        Self { train, from, to }
    }

    /**
     * Returns the train.
     *
     * # Returns
     * The train.
     */
    pub(crate) fn train(&self) -> &Train {
        self.train.as_ref()
    }

    /**
     * Returns the departure station index.
     *
     * # Returns
     * The departure station index.
     */
    pub(crate) const fn from(&self) -> usize {
        self.from
    }

    /**
     * Returns the arrival station index.
     *
     * # Returns
     * The arrival station index.
     */
    pub(crate) const fn to(&self) -> usize {
        self.to
    }
}

#[derive(Debug)]
struct TimetableValue {
    stations: Vec<Station>,
    trains: Vec<Train>,
}

impl TimetableValue {
    const fn new(stations: Vec<Station>, trains: Vec<Train>) -> Self {
        Self { stations, trains }
    }
}

/**
 * A timetable vocabulary.
 */
#[derive(Debug)]
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
    pub(crate) fn new(reader: Box<dyn BufRead>) -> Result<Self, TimetableError> {
        Ok(Self {
            value: Self::build_timetable(reader)?,
        })
    }

    fn build_timetable(mut reader: Box<dyn BufRead>) -> Result<TimetableValue, TimetableError> {
        let mut value = Self::parse_input(reader.as_mut())?;
        Self::guess_arrival_times(&mut value)?;
        Ok(value)
    }

    fn parse_input(reader: &mut dyn BufRead) -> Result<TimetableValue, TimetableError> {
        let mut lines = reader.lines();

        let stations = {
            let Some(line1) = Self::read_line(&mut lines)? else {
                return Err(TimetableError::UnexpectedEndOfFile);
            };
            let Some(line2) = Self::read_line(&mut lines)? else {
                return Err(TimetableError::UnexpectedEndOfFile);
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

    fn read_line(
        lines: &mut Lines<&mut dyn BufRead>,
    ) -> Result<Option<Vec<String>>, TimetableError> {
        let Some(line) = lines.next() else {
            return Ok(None);
        };
        let line = line.map_err(|e| TimetableError::InternalError(e.into()))?;
        let elements = line
            .split(',')
            .map(|e| e.trim().to_string())
            .collect::<Vec<_>>();
        Ok(Some(elements))
    }

    fn parse_stations(
        line1: Vec<String>,
        line2: Vec<String>,
    ) -> Result<Vec<Station>, TimetableError> {
        if line1.len() != line2.len() {
            return Err(TimetableError::StationNamesAndTelegramCodesUnmatch);
        }
        let stations = line1
            .into_iter()
            .skip(2)
            .zip(line2.into_iter().skip(2))
            .map(|(name, telegram_code)| Station::new(name, telegram_code))
            .collect::<Vec<_>>();
        Ok(stations)
    }

    fn parse_train(mut line: Vec<String>, station_count: usize) -> Result<Train, TimetableError> {
        if line.len() > station_count + 2 {
            return Err(TimetableError::InvalidTrainLineFound);
        }
        line.resize(station_count + 2, String::new());
        let number = line[0].clone();
        let name = line[1].clone();
        let stops = line
            .into_iter()
            .skip(2)
            .map(Self::to_stop)
            .collect::<Result<Vec<_>, TimetableError>>()?;
        Ok(Train::new(number, name, stops))
    }

    fn to_stop(element: String) -> Result<Stop, TimetableError> {
        let string_times = element
            .split('/')
            .map(|e| e.trim().to_string())
            .collect::<Vec<_>>();
        if string_times.is_empty() || string_times.len() > 2 {
            Err(TimetableError::InvalidArrivalOrDepartureTimeFound)
        } else if string_times.len() == 1 {
            Ok(Stop::new(None, Self::to_minutes(string_times[0].as_str())?))
        } else {
            Ok(Stop::new(
                Self::to_minutes(string_times[0].as_str())?,
                Self::to_minutes(string_times[1].as_str())?,
            ))
        }
    }

    fn to_minutes(string_time: &str) -> Result<Option<usize>, TimetableError> {
        if string_time.is_empty() || string_time == "-" {
            return Ok(None);
        }
        let int_time = string_time
            .parse::<usize>()
            .map_err(|e| TimetableError::InternalError(e.into()))?;
        let hour = int_time / 100;
        let minute = int_time - hour * 100;
        if hour >= 24 || minute >= 60 {
            return Err(TimetableError::InvalidTimeFound);
        }
        Ok(Some(hour * 60 + minute))
    }

    fn guess_arrival_times(value: &mut TimetableValue) -> Result<(), TimetableError> {
        for from in 0..value.stations.len() - 1 {
            for to in from + 1..value.stations.len() {
                let minimum_duration = Self::minimum_duration(value.trains.as_ref(), from, to)?;
                for train in &mut value.trains {
                    if !Self::all_passing(train.stops(), from, to) {
                        continue;
                    }
                    if train.stops()[to].arrival_time().is_none() {
                        let Some(from_departure_time) = train.stops()[from].departure_time() else {
                            return Err(TimetableError::BothArrivalAndDepartureTimeNotFound);
                        };
                        train.stops_mut()[to].set_arrival_time(Self::add_time(
                            from_departure_time,
                            minimum_duration,
                        ));
                    } else if train.stops()[from].departure_time().is_none() {
                        let Some(to_arrival_time) = train.stops()[to].arrival_time() else {
                            return Err(TimetableError::BothArrivalAndDepartureTimeNotFound);
                        };
                        train.stops_mut()[from]
                            .set_departure_time(Self::add_time(to_arrival_time, -minimum_duration));
                    }
                }
            }
        }
        Ok(())
    }

    fn minimum_duration(trains: &[Train], from: usize, to: usize) -> Result<isize, TimetableError> {
        let mut minimum = isize::MAX;
        for train in trains {
            if !Self::all_passing(train.stops(), from, to) {
                continue;
            }
            let from_time = if let Some(departure_time) = train.stops()[from].departure_time() {
                departure_time
            } else if let Some(arrival_time) = train.stops()[from].arrival_time() {
                arrival_time
            } else {
                return Err(TimetableError::BothArrivalAndDepartureTimeNotFound);
            };
            let to_time = if let Some(arrival_time) = train.stops()[to].arrival_time() {
                arrival_time
            } else if let Some(departure_time) = train.stops()[to].departure_time() {
                departure_time
            } else {
                return Err(TimetableError::BothArrivalAndDepartureTimeNotFound);
            };
            let duration = Self::diff_time(to_time, from_time);
            if duration < minimum {
                minimum = duration;
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
    pub(crate) fn create_vocabulary(&self, departure_time: usize) -> Rc<dyn Vocabulary> {
        let entries = Self::build_entries(&self.value);
        let connections = Self::build_connections(&entries, departure_time);
        Rc::new(HashMapVocabulary::new(
            entries,
            connections,
            &Self::entry_hash_value,
            &Self::entry_equal_to,
        ))
    }

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
                        i32::try_from(Self::make_section_duration(train.stops(), from, to))
                            .expect("Section duration should fit in i32"),
                    ));
                }
            }
        }
        map.into_iter().collect::<Vec<_>>()
    }

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

    fn make_section_duration(stops: &[Stop], from: usize, to: usize) -> usize {
        let departure_time = stops[from].departure_time().unwrap_or_else(|| {
            unreachable!("departure_time must be set.");
        });
        let arrival_time = stops[to].arrival_time().unwrap_or_else(|| {
            unreachable!("arrival_time must be set.");
        });
        Self::diff_time(arrival_time, departure_time) as usize
    }

    fn build_connections(
        entries: &[(String, Vec<Entry>)],
        departure_time: usize,
    ) -> Vec<((Entry, Entry), i32)> {
        let mut connections = Vec::<((Entry, Entry), i32)>::new();

        for (_, from_entries) in entries {
            for (_, to_entries) in entries {
                for from_entry in from_entries {
                    for to_entry in to_entries {
                        let from_value = from_entry
                            .value()
                            .unwrap_or_else(|| {
                                unreachable!("from_entry.value() must not be empty.")
                            })
                            .downcast_ref::<Section>()
                            .unwrap_or_else(|| unreachable!("from_entry.value() must be Section."));
                        let to_value = to_entry
                            .value()
                            .unwrap_or_else(|| unreachable!("to_entry.value() must not be empty."))
                            .downcast_ref::<Section>()
                            .unwrap_or_else(|| unreachable!("to_entry.value() must be Section."));
                        if from_value.to() != to_value.from() {
                            continue;
                        }

                        let from_arrival_time = from_value.train().stops()[from_value.to()]
                            .arrival_time()
                            .unwrap_or_else(|| {
                                unreachable!("from arrival_time must be set.");
                            });
                        let to_departure_time = to_value.train().stops()[to_value.from()]
                            .departure_time()
                            .unwrap_or_else(|| {
                                unreachable!("to departure_time must be set.");
                            });
                        let cost =
                            i32::try_from(Self::diff_time(to_departure_time, from_arrival_time))
                                .expect("Time difference should fit in i32");
                        if cost > 60 {
                            continue;
                        }
                        if from_value.train().number() != to_value.train().number() {
                            connections.push(((from_entry.clone(), to_entry.clone()), cost + 1));
                        } else {
                            connections.push(((from_entry.clone(), to_entry.clone()), cost));
                        }
                    }
                }
            }
        }

        for (_, entries) in entries {
            for entry in entries {
                let section = entry
                    .value()
                    .unwrap_or_else(|| unreachable!("entry.value() must not be empty."))
                    .downcast_ref::<Section>()
                    .unwrap_or_else(|| unreachable!("entry.value() must be Section."));
                let section_departure_time = section.train().stops()[section.from()]
                    .departure_time()
                    .unwrap_or_else(|| {
                        unreachable!("departure_time() must be set.");
                    });
                let bos_cost =
                    i32::try_from(Self::diff_time(section_departure_time, departure_time))
                        .expect("Time difference should fit in i32");
                if bos_cost <= 240 {
                    connections.push(((Entry::BosEos, entry.clone()), bos_cost * 9 / 10));
                }
                connections.push(((entry.clone(), Entry::BosEos), 0));
            }
        }

        connections
    }

    const fn add_time(time: usize, duration: isize) -> usize {
        assert!(time < 1440);
        assert!(-1440 < duration && duration < 1440);
        (time as isize + 1440 + duration) as usize % 1440
    }

    const fn diff_time(time1: usize, time2: usize) -> isize {
        assert!(time1 < 1440);
        assert!(time2 < 1440);
        (time1 as isize + 1440 - time2 as isize) % 1440
    }

    fn entry_hash_value(entry: &Entry) -> u64 {
        let mut hasher = DefaultHasher::new();

        hasher.write_u64(if let Some(key) = entry.key() {
            key.hash_value()
        } else {
            0
        });
        let section = if let Some(value) = entry.value() {
            value.downcast_ref::<Section>()
        } else {
            None
        };
        if let Some(section) = section {
            section.train().number().hash(&mut hasher);
            section.train().name().hash(&mut hasher);
            section.from().hash(&mut hasher);
            section.to().hash(&mut hasher);
        } else {
            "".hash(&mut hasher);
            "".hash(&mut hasher);
            0usize.hash(&mut hasher);
            0usize.hash(&mut hasher);
        }
        hasher.finish()
    }

    fn entry_equal_to(one: &Entry, another: &Entry) -> bool {
        if let Some(one_value) = one.value() {
            if let Some(another_value) = another.value() {
                let Some(one_section) = one_value.downcast_ref::<Section>() else {
                    unreachable!("one.value() must be Section.");
                };
                let Some(another_section) = another_value.downcast_ref::<Section>() else {
                    unreachable!("another.value() must be Section.");
                };
                (if let Some(one_key) = one.key() {
                    if let Some(another_key) = another.key() {
                        one_key.equal_to(another_key)
                    } else {
                        false
                    }
                } else {
                    another.key().is_none()
                } && one_section.train().number() == another_section.train().number()
                    && one_section.train().name() == another_section.train().name()
                    && one_section.from() == another_section.from()
                    && one_section.to() == another_section.to())
            } else {
                false
            }
        } else if another.value().is_none() {
            if let Some(one_key) = one.key() {
                if let Some(another_key) = another.key() {
                    one_key.equal_to(another_key)
                } else {
                    false
                }
            } else {
                another.key().is_none()
            }
        } else {
            false
        }
    }
}
