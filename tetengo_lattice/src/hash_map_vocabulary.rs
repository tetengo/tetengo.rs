/*!
 * A hash map vocabulary.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use crate::connection::Connection;
use crate::entry::{Entry, EntryView};
use crate::node::Node;
use crate::vocabulary::Vocabulary;

type EntryMap = HashMap<String, Vec<Entry>>;

#[derive(Clone, Debug)]
struct HashableEntry {
    entry: Entry,
    hash_value: fn(&EntryView<'_>) -> u64,
    equal: fn(&EntryView<'_>, &EntryView<'_>) -> bool,
}

impl HashableEntry {
    fn from(
        entry: Entry,
        hash_value: fn(&EntryView<'_>) -> u64,
        equal: fn(&EntryView<'_>, &EntryView<'_>) -> bool,
    ) -> Self {
        HashableEntry {
            entry,
            hash_value,
            equal,
        }
    }
}

impl Eq for HashableEntry {}

impl Hash for HashableEntry {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let entry_view = self.entry.as_view();
        let hash_value = (self.hash_value)(&entry_view);
        hash_value.hash(state);
    }
}

impl PartialEq for HashableEntry {
    fn eq(&self, other: &Self) -> bool {
        let self_entry_view = self.entry.as_view();
        let other_entry_view = other.entry.as_view();
        (self.equal)(&self_entry_view, &other_entry_view)
    }
}

type ConnectionMap = HashMap<(HashableEntry, HashableEntry), i32>;

/**
 * A hash map vocabulary.
 */
#[derive(Clone, Debug)]
pub struct HashMapVocabulary {
    _entry_map: EntryMap,
    _connection_map: ConnectionMap,
}

impl HashMapVocabulary {
    /**
     * Creates a hash map vocabulary.
     *
     * # Arguments
     * * `entries`          - Entries.
     * * `connections`      - Connections.
     * * `entry_hash_value` - A hash function for an entry.
     */
    pub fn new(
        entries: Vec<(String, Vec<Entry>)>,
        connections: Vec<((Entry, Entry), i32)>,
        entry_hash_value: fn(&EntryView<'_>) -> u64,
        entry_equal: fn(&EntryView<'_>, &EntryView<'_>) -> bool,
    ) -> Self {
        let entry_map = Self::make_entry_map(entries);
        let connection_map = Self::make_connection_map(connections, entry_hash_value, entry_equal);
        HashMapVocabulary {
            _entry_map: entry_map,
            _connection_map: connection_map,
        }
    }

    fn make_entry_map(entries: Vec<(String, Vec<Entry>)>) -> EntryMap {
        let mut entry_map = EntryMap::new();
        for (key, entries) in entries {
            let _prev_value = entry_map.insert(key, entries);
        }
        entry_map
    }

    fn make_connection_map(
        connections: Vec<((Entry, Entry), i32)>,
        entry_hash_value: fn(&EntryView<'_>) -> u64,
        entry_equal: fn(&EntryView<'_>, &EntryView<'_>) -> bool,
    ) -> ConnectionMap {
        let mut connection_map = ConnectionMap::new();
        for ((from, to), cost) in connections {
            let from = HashableEntry::from(from, entry_hash_value, entry_equal);
            let to = HashableEntry::from(to, entry_hash_value, entry_equal);
            let _prev_value = connection_map.insert((from, to), cost);
        }
        connection_map
    }
    /*
    static void build_connection_map(
        std::vector<std::pair<std::pair<entry, entry>, int>>      connections,
        std::function<std::size_t(const entry_view&)>             entry_hash,
        std::function<bool(const entry_view&, const entry_view&)> entry_equal_to,
        std::vector<std::pair<entry, entry>>&                     connection_keys,
        std::unique_ptr<connection_map_type>&                     p_connection_map)
    {
        connection_keys.reserve(std::size(connections));
        for (auto&& e: connections)
        {
            connection_keys.push_back(std::move(e.first));
        }

        auto p_map = std::make_unique<connection_map_type>(
            std::size(connections),
            connection_map_hash{ std::move(entry_hash) },
            connection_map_key_eq{ std::move(entry_equal_to) });
        p_map->reserve(std::size(connections));
        for (auto i = static_cast<std::size_t>(0); i < std::size(connections); ++i)
        {
            const auto&      connection_key = connection_keys[i];
            const entry_view from{ connection_key.first.p_key(),
                                   &connection_key.first.value(),
                                   connection_key.first.cost() };
            const entry_view to{ connection_key.second.p_key(),
                                 &connection_key.second.value(),
                                 connection_key.second.cost() };
            p_map->insert(std::make_pair(std::make_pair(from, to), connections[i].second));
        }

        p_connection_map = std::move(p_map);
    }
    */
}

impl Vocabulary for HashMapVocabulary {
    fn find_entries(&self, _key: &dyn crate::Input) -> Vec<EntryView<'_>> {
        todo!()
    }

    fn find_connection(&self, _from: &Node<'_>, _to: &EntryView<'_>) -> Connection {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::string_input::StringInput;

    use super::*;

    fn entry_hash_value(entry: &EntryView<'_>) -> u64 {
        let Some(key) = entry.key() else {
            return 0;
        };
        key.hash_value()
    }

    fn entry_equal(one: &EntryView<'_>, other: &EntryView<'_>) -> bool {
        match (one.key(), other.key()) {
            (Some(one_key), Some(other_key)) => one_key.equal_to(other_key),
            (None, None) => true,
            _ => false,
        }
    }

    /*
     namespace
     {
         tetengo::lattice::node make_node(const tetengo::lattice::entry_view& entry)
         {
             static const std::vector<int> preceding_edge_costs{};
             return tetengo::lattice::node{ entry,
                                            0,
                                            std::numeric_limits<std::size_t>::max(),
                                            &preceding_edge_costs,
                                            std::numeric_limits<std::size_t>::max(),
                                            std::numeric_limits<int>::max() };
         }

    }
    */

    #[test]
    fn new() {
        {
            let entries = Vec::<(String, Vec<Entry>)>::new();
            let connections = Vec::<((Entry, Entry), i32)>::new();
            let _vocaburary =
                HashMapVocabulary::new(entries, connections, entry_hash_value, entry_equal);
        }
        {
            let entries = vec![
                (
                    String::from("みずほ"),
                    vec![Entry::new(
                        Box::new(StringInput::new(String::from("みずほ"))),
                        Box::new(String::from("瑞穂")),
                        42,
                    )],
                ),
                (
                    String::from("さくら"),
                    vec![
                        Entry::new(
                            Box::new(StringInput::new(String::from("さくら"))),
                            Box::new(String::from("桜")),
                            24,
                        ),
                        Entry::new(
                            Box::new(StringInput::new(String::from("さくら"))),
                            Box::new(String::from("さくら")),
                            2424,
                        ),
                    ],
                ),
            ];
            let connections = vec![(
                (
                    Entry::new(
                        Box::new(StringInput::new(String::from("みずほ"))),
                        Box::new(String::from("瑞穂")),
                        42,
                    ),
                    Entry::new(
                        Box::new(StringInput::new(String::from("さくら"))),
                        Box::new(String::from("桜")),
                        24,
                    ),
                ),
                4242,
            )];
            let _vocaburary =
                HashMapVocabulary::new(entries, connections, entry_hash_value, entry_equal);
        }
    }
    /*
    BOOST_AUTO_TEST_CASE(construction)
    {
        BOOST_TEST_PASSPOINT();

        {
            std::vector<std::pair<std::string, std::vector<tetengo::lattice::entry>>>                entries{};
            std::vector<std::pair<std::pair<tetengo::lattice::entry, tetengo::lattice::entry>, int>> connections{};
            const tetengo::lattice::unordered_map_vocabulary                                         vocabulary{
                std::move(entries), std::move(connections), cpp_entry_hash, cpp_entry_equal_to
            };
        }
        {
            std::vector<std::pair<std::string, std::vector<tetengo::lattice::entry>>> entries{
                { key_mizuho, { { std::make_unique<tetengo::lattice::string_input>(key_mizuho), surface_mizuho, 42 } } },
                { key_sakura,
                  { { std::make_unique<tetengo::lattice::string_input>(key_sakura), surface_sakura1, 24 },
                    { std::make_unique<tetengo::lattice::string_input>(key_sakura), surface_sakura2, 2424 } } }
            };
            std::vector<std::pair<std::pair<tetengo::lattice::entry, tetengo::lattice::entry>, int>> connections{
                { std::make_pair(
                      tetengo::lattice::entry{
                          std::make_unique<tetengo::lattice::string_input>(key_mizuho), surface_mizuho, 42 },
                      tetengo::lattice::entry{
                          std::make_unique<tetengo::lattice::string_input>(key_sakura), surface_sakura1, 24 }),
                  4242 }
            };
            const tetengo::lattice::unordered_map_vocabulary vocabulary{
                std::move(entries), std::move(connections), cpp_entry_hash, cpp_entry_equal_to
            };
        }
    }
    */
    /*
    BOOST_AUTO_TEST_CASE(find_entries)
    {
        BOOST_TEST_PASSPOINT();

        {
            std::vector<std::pair<std::string, std::vector<tetengo::lattice::entry>>>                entries{};
            std::vector<std::pair<std::pair<tetengo::lattice::entry, tetengo::lattice::entry>, int>> connections{};
            const tetengo::lattice::unordered_map_vocabulary                                         vocabulary{
                std::move(entries), std::move(connections), cpp_entry_hash, cpp_entry_equal_to
            };

            {
                const auto found = vocabulary.find_entries(key_type{ key_mizuho });
                BOOST_TEST(std::empty(found));
            }
            {
                const auto found = vocabulary.find_entries(key_type{ key_sakura });
                BOOST_TEST(std::empty(found));
            }
        }
        {
            std::vector<std::pair<std::string, std::vector<tetengo::lattice::entry>>> entries{
                { key_mizuho, { { std::make_unique<key_type>(key_mizuho), surface_mizuho, 42 } } },
                { key_sakura,
                  { { std::make_unique<key_type>(key_sakura), surface_sakura1, 24 },
                    { std::make_unique<key_type>(key_sakura), surface_sakura2, 2424 } } }
            };
            std::vector<std::pair<std::pair<tetengo::lattice::entry, tetengo::lattice::entry>, int>> connections{
                { std::make_pair(
                      tetengo::lattice::entry{ std::make_unique<key_type>(key_mizuho), surface_mizuho, 42 },
                      tetengo::lattice::entry{ std::make_unique<key_type>(key_sakura), surface_sakura1, 24 }),
                  4242 }
            };
            const tetengo::lattice::unordered_map_vocabulary vocabulary{
                std::move(entries), std::move(connections), cpp_entry_hash, cpp_entry_equal_to
            };

            {
                const auto found = vocabulary.find_entries(key_type{ key_mizuho });
                BOOST_TEST_REQUIRE(std::size(found) == 1U);
                BOOST_TEST_REQUIRE(found[0].p_key());
                BOOST_TEST_REQUIRE(found[0].p_key()->is<key_type>());
                BOOST_TEST(found[0].p_key()->as<key_type>().value() == key_mizuho);
                BOOST_TEST(*std::any_cast<std::string>(found[0].value()) == surface_mizuho);
                BOOST_TEST(found[0].cost() == 42);
            }
            {
                const auto found = vocabulary.find_entries(key_type{ key_sakura });
                BOOST_TEST_REQUIRE(std::size(found) == 2U);
                BOOST_TEST_REQUIRE(found[0].p_key());
                BOOST_TEST_REQUIRE(found[0].p_key()->is<key_type>());
                BOOST_TEST(found[0].p_key()->as<key_type>().value() == key_sakura);
                BOOST_TEST(*std::any_cast<std::string>(found[0].value()) == surface_sakura1);
                BOOST_TEST(found[0].cost() == 24);
                BOOST_TEST_REQUIRE(found[1].p_key());
                BOOST_TEST_REQUIRE(found[1].p_key()->is<key_type>());
                BOOST_TEST(found[1].p_key()->as<key_type>().value() == key_sakura);
                BOOST_TEST(*std::any_cast<std::string>(found[1].value()) == surface_sakura2);
                BOOST_TEST(found[1].cost() == 2424);
            }
        }
    }
    */
    /*
    BOOST_AUTO_TEST_CASE(find_connection)
    {
        BOOST_TEST_PASSPOINT();

        {
            std::vector<std::pair<std::string, std::vector<tetengo::lattice::entry>>> entries{
                { key_mizuho, { { std::make_unique<tetengo::lattice::string_input>(key_mizuho), surface_mizuho, 42 } } },
                { key_sakura,
                  { { std::make_unique<tetengo::lattice::string_input>(key_sakura), surface_sakura1, 24 },
                    { std::make_unique<tetengo::lattice::string_input>(key_sakura), surface_sakura2, 2424 } } }
            };
            std::vector<std::pair<std::pair<tetengo::lattice::entry, tetengo::lattice::entry>, int>> connections{
                { std::make_pair(
                      tetengo::lattice::entry{
                          std::make_unique<tetengo::lattice::string_input>(key_mizuho), surface_mizuho, 42 },
                      tetengo::lattice::entry{
                          std::make_unique<tetengo::lattice::string_input>(key_sakura), surface_sakura1, 24 }),
                  4242 }
            };
            const tetengo::lattice::unordered_map_vocabulary vocabulary{
                std::move(entries), std::move(connections), cpp_entry_hash, cpp_entry_equal_to
            };

            const auto entries_mizuho = vocabulary.find_entries(key_type{ key_mizuho });
            BOOST_TEST_REQUIRE(std::size(entries_mizuho) == 1U);
            const auto entries_sakura = vocabulary.find_entries(key_type{ key_sakura });
            BOOST_TEST_REQUIRE(std::size(entries_sakura) == 2U);

            {
                const auto connection = vocabulary.find_connection(make_node(entries_mizuho[0]), entries_sakura[0]);

                BOOST_TEST(connection.cost() == 4242);
            }
            {
                const auto connection = vocabulary.find_connection(make_node(entries_mizuho[0]), entries_mizuho[0]);

                BOOST_TEST(connection.cost() == std::numeric_limits<int>::max());
            }
        }
    }
    */
}
