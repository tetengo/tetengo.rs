/*!
 * A timetable vocabulary.
 *
 * Copyright (C) 2023-2024 kaoru  <https://www.tetengo.org/>
 */

use std::io::BufRead;

use anyhow::Result;

/**
 * A station.
 */
pub(crate) struct Station {
    _name: String,
    _telegram_code: String,
}

impl Station {
    /**
     * Creates a station.
     *
     * # Arguments
     * * `name`          - A name.
     * * `telegram_code` - A telegram code.
     */
    pub(crate) const fn _new(name: String, telegram_code: String) -> Self {
        Self {
            _name: name,
            _telegram_code: telegram_code,
        }
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
     * Returns the telegram code.
     *
     * # Returns
     * The telegram code.
     */
    pub(crate) fn _telegram_code(&self) -> &str {
        self._telegram_code.as_str()
    }
}

/**
 * A stop.
 */
pub(crate) struct Stop {
    _arrival_time: Option<usize>,
    _departure_time: Option<usize>,
}

impl Stop {
    /**
     * Creates a stop.
     *
     * # Arguments
     * * `arrival_time`   - An arrival time.
     * * `departure_time` - A departure time.
     */
    pub(crate) const fn _new(arrival_time: Option<usize>, departure_time: Option<usize>) -> Self {
        Self {
            _arrival_time: arrival_time,
            _departure_time: departure_time,
        }
    }

    /**
     * Returns the arrival time.
     *
     * # Returns
     * The arrival time.
     */
    pub(crate) const fn _arrival_time(&self) -> Option<usize> {
        self._arrival_time
    }

    /**
     * Sets an arrival time.
     *
     * # Arguments
     * * `time` - An arrival time.
     */
    pub(crate) fn _set_arrival_time(&mut self, time: usize) {
        self._arrival_time = Some(time);
    }

    /**
     * Returns the departure time.
     *
     * # Returns
     * The departure time.
     */
    pub(crate) const fn _departure_time(&self) -> Option<usize> {
        self._departure_time
    }

    /**
     * Sets a departure time.
     *
     * # Arguments
     * * `time` - A departure time.
     */
    pub(crate) fn _set_departure_time(&mut self, time: usize) {
        self._departure_time = Some(time);
    }
}

/**
 * A train.
 */
pub(crate) struct Train {
    _number: String,
    _name: String,
    _stops: Vec<Stop>,
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
    pub(crate) const fn _new(number: String, name: String, stops: Vec<Stop>) -> Self {
        Self {
            _number: number,
            _name: name,
            _stops: stops,
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
    pub(crate) fn _stops(&self) -> &[Stop] {
        self._stops.as_slice()
    }

    /**
     * Returns the stops.
     *
     * # Returns
     * The stops.
     */
    pub(crate) fn _stops_mut(&mut self) -> &Vec<Stop> {
        &self._stops
    }
}

/**
 * A section.
 */
pub(crate) struct _Section<'a> {
    train: &'a Train,
    from: usize,
    to: usize,
}

impl<'a> _Section<'a> {
    /**
     * Creates a section.
     *
     * # Arguments
     * * `train` - A train.
     * * `from`  - A departure station index.
     * * `to`    - An arrival station index.
     */
    pub(crate) const fn _new(train: &'a Train, from: usize, to: usize) -> Self {
        Self { train, from, to }
    }

    /**
     * Returns the train.
     *
     * # Returns
     * The train.
     */
    pub(crate) const fn _train(&self) -> &Train {
        self.train
    }

    /**
     * Returns the departure station index.
     *
     * # Returns
     * The departure station index.
     */
    pub(crate) const fn _from(&self) -> usize {
        self.from
    }

    /**
     * Returns the arrival station index.
     *
     * # Returns
     * The arrival station index.
     */
    pub(crate) const fn _to(&self) -> usize {
        self.to
    }
}

struct TimetableValue {
    _stations: Vec<Station>,
    _trains: Vec<Train>,
}

impl TimetableValue {
    fn new(stations: Vec<Station>, trains: Vec<Train>) -> Self {
        Self {
            _stations: stations,
            _trains: trains,
        }
    }
}

/**
 * A timetable vocabulary.
 */
pub(crate) struct Timetable {
    _value: TimetableValue,
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
            _value: Self::build_timetable(reader)?,
        })
    }

    fn build_timetable(reader: Box<dyn BufRead>) -> Result<TimetableValue> {
        let value = Self::parse_input(&reader)?;
        //guess_arrival_times(&mut value);
        Ok(value)
    }
    /*
    private:
        // static functions

        static timetable_value build_timetable(std::unique_ptr<std::istream>&& p_input_stream)
        {
            assert(p_input_stream);
            auto timetable_value = parse_input(*p_input_stream);
            guess_arrival_times(timetable_value);
            return timetable_value;
        }
    */

    fn parse_input(_reader: &dyn BufRead) -> Result<TimetableValue> {
        let stations = Vec::new();

        let trains = Vec::new();

        Ok(TimetableValue::new(stations, trains))
    }
    /*
        static timetable_value parse_input(std::istream& input_stream)
        {
            if (!input_stream)
            {
                throw std::runtime_error{ "Input file format error: Empty data." };
            }

            std::vector<station> stations{};
            {
                auto line1 = read_line(input_stream);
                auto line2 = read_line(input_stream);
                stations = parse_stations(std::move(line1), std::move(line2));
            }

            std::vector<train> trains{};
            while (input_stream)
            {
                auto line = read_line(input_stream);
                if (std::empty(line) || (std::size(line) == 1 && std::empty(line[0])))
                {
                    continue;
                }
                trains.push_back(parse_train(std::move(line), std::size(stations)));
            }

            return timetable_value{ std::move(stations), std::move(trains) };
        }
    */
    /*
        static std::vector<std::string> read_line(std::istream& input_stream)
        {
            std::string line;
            std::getline(input_stream, line);

            std::vector<std::string> elements;
            boost::algorithm::split(elements, std::move(line), boost::is_any_of(","));
            std::for_each(
                std::begin(elements), std::end(elements), [](auto& element) { return boost::algorithm::trim(element); });
            return elements;
        }
    */
    /*
        static std::vector<station> parse_stations(std::vector<std::string>&& line1, std::vector<std::string>&& line2)
        {
            line1.erase(std::begin(line1), std::next(std::begin(line1), 2));
            line2.erase(std::begin(line2), std::next(std::begin(line2), 2));
            if (std::size(line1) != std::size(line2))
            {
                throw std::runtime_error{ "Input file format error: Station names and telegram codes unmatch." };
            }
            std::vector<station> stations{};
            stations.reserve(std::size(line1));
            for (auto i = static_cast<std::size_t>(0); i < std::size(line1); ++i)
            {
                stations.emplace_back(std::move(line1[i]), std::move(line2[i]));
            }
            return stations;
        }
    */
    /*
        static train parse_train(std::vector<std::string>&& line, const std::size_t station_count)
        {
            if (std::size(line) > station_count + 2)
            {
                throw std::runtime_error{ "Input file format error: Invalid train line found." };
            }
            line.resize(station_count + 2);

            std::vector<stop> stops{};
            stops.reserve(station_count);
            std::transform(std::next(std::begin(line), 2), std::end(line), std::back_inserter(stops), [](auto&& e) {
                return to_stop(std::move(e));
            });
            return train{ std::move(line[0]), std::move(line[1]), std::move(stops) };
        }
    */
    /*
        static stop to_stop(std::string&& element)
        {
            std::vector<std::string> string_times{};
            boost::algorithm::split(string_times, std::move(element), boost::is_any_of("/"));
            std::for_each(std::begin(string_times), std::end(string_times), [](auto&& e) { return boost::trim(e); });
            if (std::size(string_times) == 0 || std::size(string_times) > 2)
            {
                throw std::runtime_error{ "Input file format error: Invalid arrival/depature time found." };
            }
            else if (std::size(string_times) == 1)
            {
                return stop{ std::nullopt, to_minutes(std::move(string_times[0])) };
            }
            else
            {
                assert(std::size(string_times) == 2);
                return stop{ to_minutes(std::move(string_times[0])), to_minutes(std::move(string_times[1])) };
            }
        }
    */
    /*
        static std::optional<std::size_t> to_minutes(std::string&& string_time)
        {
            if (std::empty(string_time) || string_time == "-")
            {
                return std::nullopt;
            }

            auto int_time = static_cast<std::size_t>(0);
            try
            {
                int_time = boost::lexical_cast<std::size_t>(string_time);
            }
            catch (const boost::bad_lexical_cast&)
            {
                throw std::runtime_error{ "Input file format error: Invalid time found." };
            }

            const auto hour = int_time / 100;
            const auto minute = int_time - hour * 100;
            if (hour >= 24 || minute >= 60)
            {
                throw std::runtime_error{ "Input file format error: Invalid time found." };
            }

            return hour * 60 + minute;
        }
    */
    /*
        static void guess_arrival_times(timetable_value& timetable_)
        {
            for (auto from = static_cast<std::size_t>(0); from < std::size(timetable_.stations) - 1; ++from)
            {
                for (auto to = from + 1; to < std::size(timetable_.stations); ++to)
                {
                    const auto minimum_duration_ = minimum_duration(timetable_.trains, from, to);

                    for (auto& train: timetable_.trains)
                    {
                        if (!all_passing(train.stops(), from, to))
                        {
                            continue;
                        }

                        if (!train.stops()[to].arrival_time())
                        {
                            train.stops()[to].set_arrival_time(
                                add_time(*train.stops()[from].departure_time(), minimum_duration_));
                        }
                        else if (!train.stops()[from].departure_time())
                        {
                            train.stops()[from].set_departure_time(
                                add_time(*train.stops()[to].arrival_time(), -minimum_duration_));
                        }
                    }
                }
            }
        }
    */
    /*
        static std::ptrdiff_t
        minimum_duration(const std::vector<train>& trains, const std::size_t from, const std::size_t to)
        {
            auto minimum = std::numeric_limits<std::ptrdiff_t>::max();
            for (const auto& train: trains)
            {
                if (!all_passing(train.stops(), from, to))
                {
                    continue;
                }

                const auto from_time = train.stops()[from].departure_time() ? *train.stops()[from].departure_time() :
                                                                              *train.stops()[from].arrival_time();
                const auto to_time = train.stops()[to].arrival_time() ? *train.stops()[to].arrival_time() :
                                                                        *train.stops()[to].departure_time();

                if (diff_time(to_time, from_time) < minimum)
                {
                    minimum = diff_time(to_time, from_time);
                }
            }
            return minimum;
        }
    */
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
        static bool all_passing(const std::vector<stop>& stops, const std::size_t from, const std::size_t to)
        {
            if (!stops[from].arrival_time() && !stops[from].departure_time())
            {
                return false;
            }
            if (!stops[to].arrival_time() && !stops[to].departure_time())
            {
                return false;
            }
            for (auto i = from + 1; i + 1 < to + 1; ++i)
            {
                if (stops[i].arrival_time() || stops[i].departure_time())
                {
                    return false;
                }
            }
            return true;
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
    */
    /*
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
    /*
        static std::size_t add_time(const std::size_t time, const std::ptrdiff_t duration)
        {
            assert(time < 1440);
            assert(-1440 < duration && duration < 1440);
            return (time + 1440 + duration) % 1440;
        }
    */
    /*
        static std::ptrdiff_t diff_time(const std::size_t time1, const std::size_t time2)
        {
            assert(time1 < 1440);
            assert(time2 < 1440);
            return (time1 + 1440 - time2) % 1440;
        }
    */
}

/*
/*!
    \brief A timetable vocabulary.
*/
class timetable : private boost::noncopyable
{
public:
    // constructors and destructor

    /*!
        \brief Creates a timetable vocabulary.

        \param p_input_stream A unique pointer to an input stream.
    */
    explicit timetable(std::unique_ptr<std::istream>&& p_input_stream);

    /*!
        \brief Destroys the timetable vocabulary.
    */
    ~timetable();
*/
/*
    // functions

    /*!
        \brief Returns the stations.

        \return The stations.
    */
    [[nodiscard]] const std::vector<station>& stations() const;
*/
/*
    /*!
        \brief Returns the station index.

        \param name_or_telegram_code A name or telegram code.

        \return The index. Or std::size(stations()) when no station is found.
    */
    [[nodiscard]] std::size_t station_index(const std::string& name_or_telegram_code) const;
*/
/*
    /*!
        \brief Creates a vocabulary.

        \param departure_time A departure time.

        \return A vocabulary.
    */
    [[nodiscard]] std::unique_ptr<tetengo::lattice::vocabulary> create_vocabulary(std::size_t departure_time) const;
*/
/*
private:
    // types

    class impl;


    // variables

    const std::unique_ptr<impl> m_p_impl;
};
 */
/*
class timetable::impl : private boost::noncopyable
{
public:
    // constructors and destructor

    explicit impl(std::unique_ptr<std::istream>&& p_input_stream) :
    m_timetable{ build_timetable(std::move(p_input_stream)) }
    {}
*/
/*
    // functions

    const std::vector<station>& stations() const
    {
        return m_timetable.stations;
    }
*/
/*
    std::size_t station_index(const std::string& name_or_telegram_code) const
    {
        for (auto i = static_cast<std::size_t>(0); i < std::size(m_timetable.stations); ++i)
        {
            if (const auto& station = m_timetable.stations[i];
                boost::algorithm::to_lower_copy(station.name()) ==
                    boost::algorithm::to_lower_copy(name_or_telegram_code) ||
                boost::algorithm::to_upper_copy(station.telegram_code()) ==
                    boost::algorithm::to_upper_copy(name_or_telegram_code))
            {
                return i;
            }
        }
        return std::size(m_timetable.stations);
    }
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

/*
timetable::timetable(std::unique_ptr<std::istream>&& p_input_stream) :
m_p_impl{ std::make_unique<impl>(std::move(p_input_stream)) }
{}

timetable::~timetable() = default;

const std::vector<station>& timetable::stations() const
{
    return m_p_impl->stations();
}

std::size_t timetable::station_index(const std::string& name_or_telegram_code) const
{
    return m_p_impl->station_index(name_or_telegram_code);
}

std::unique_ptr<tetengo::lattice::vocabulary> timetable::create_vocabulary(const std::size_t departure_time) const
{
    return m_p_impl->create_vocabulary(departure_time);
}
 */

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
