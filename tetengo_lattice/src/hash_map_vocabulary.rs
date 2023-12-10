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
use crate::string_input::StringInput;
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
    entry_map: EntryMap,
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
            entry_map,
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
}

impl Vocabulary for HashMapVocabulary {
    fn find_entries(&self, key: &dyn crate::Input) -> Vec<EntryView<'_>> {
        let Some(key) = key.as_any().downcast_ref::<StringInput>() else {
            return Vec::new();
        };
        let Some(found) = self.entry_map.get(key.value()) else {
            return Vec::new();
        };

        found.iter().map(|entry| entry.as_view()).collect()
    }

    fn find_connection(&self, _from: &Node<'_>, _to: &EntryView<'_>) -> Connection {
        todo!()
    }
    /*
    connection find_connection_impl(const node& from, const entry_view& to) const
    {
        const entry_view from_entry_view{ from.p_key(), &from.value(), from.node_cost() };
        const auto       found = m_p_connection_map->find(std::make_pair(from_entry_view, to));
        if (found == std::end(*m_p_connection_map))
        {
            return connection{ std::numeric_limits<int>::max() };
        }
        return connection{ found->second };
    }
    */
}

#[cfg(test)]
mod tests {
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

    #[test]
    fn find_entries() {
        {
            let entries = Vec::<(String, Vec<Entry>)>::new();
            let connections = Vec::<((Entry, Entry), i32)>::new();
            let vocaburary =
                HashMapVocabulary::new(entries, connections, entry_hash_value, entry_equal);

            {
                let found = vocaburary.find_entries(&StringInput::new(String::from("みずほ")));
                assert!(found.is_empty());
            }
            {
                let found = vocaburary.find_entries(&StringInput::new(String::from("さくら")));
                assert!(found.is_empty());
            }
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
            let vocaburary =
                HashMapVocabulary::new(entries, connections, entry_hash_value, entry_equal);

            {
                let found = vocaburary.find_entries(&StringInput::new(String::from("みずほ")));
                assert_eq!(found.len(), 1);
                assert_eq!(
                    found[0]
                        .key()
                        .unwrap()
                        .as_any()
                        .downcast_ref::<StringInput>()
                        .unwrap()
                        .value(),
                    "みずほ"
                );
                assert_eq!(
                    found[0]
                        .value()
                        .unwrap()
                        .as_any()
                        .downcast_ref::<String>()
                        .unwrap(),
                    "瑞穂"
                );
                assert_eq!(found[0].cost(), 42);
            }
            {
                let found = vocaburary.find_entries(&StringInput::new(String::from("さくら")));
                assert_eq!(found.len(), 2);
                assert_eq!(
                    found[0]
                        .key()
                        .unwrap()
                        .as_any()
                        .downcast_ref::<StringInput>()
                        .unwrap()
                        .value(),
                    "さくら"
                );
                assert_eq!(
                    found[0]
                        .value()
                        .unwrap()
                        .as_any()
                        .downcast_ref::<String>()
                        .unwrap(),
                    "桜"
                );
                assert_eq!(found[0].cost(), 24);
                assert_eq!(
                    found[1]
                        .key()
                        .unwrap()
                        .as_any()
                        .downcast_ref::<StringInput>()
                        .unwrap()
                        .value(),
                    "さくら"
                );
                assert_eq!(
                    found[1]
                        .value()
                        .unwrap()
                        .as_any()
                        .downcast_ref::<String>()
                        .unwrap(),
                    "さくら"
                );
                assert_eq!(found[1].cost(), 2424);
            }
        }
    }

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
