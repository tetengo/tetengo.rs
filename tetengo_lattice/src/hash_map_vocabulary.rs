/*!
 * A hash map vocabulary.
 *
 * Copyright (C) 2023-2025 kaoru  <https://www.tetengo.org/>
 */

use std::any::type_name_of_val;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use crate::connection::Connection;
use crate::entry::Entry;
use crate::error::Error;
use crate::node::Node;
use crate::string_input::StringInput;
use crate::vocabulary::Vocabulary;

type EntryMap = HashMap<String, Vec<Rc<Entry>>>;

#[derive(Clone)]
struct HashableEntry<'a> {
    entry: Entry,
    hash_value: &'a dyn Fn(&Entry) -> u64,
    equal: &'a dyn Fn(&Entry, &Entry) -> bool,
}

impl<'a> HashableEntry<'a> {
    const fn new(
        entry: Entry,
        hash_value: &'a dyn Fn(&Entry) -> u64,
        equal: &'a dyn Fn(&Entry, &Entry) -> bool,
    ) -> Self {
        Self {
            entry,
            hash_value,
            equal,
        }
    }
}

impl Debug for HashableEntry<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HashableEntry")
            .field("entry", &self.entry)
            .field("hash_value", &type_name_of_val(&self.hash_value))
            .field("equal", &type_name_of_val(&self.equal))
            .finish()
    }
}

impl Eq for HashableEntry<'_> {}

impl Hash for HashableEntry<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let hash_value = (self.hash_value)(&self.entry);
        hash_value.hash(state);
    }
}

impl PartialEq for HashableEntry<'_> {
    fn eq(&self, other: &Self) -> bool {
        (self.equal)(&self.entry, &other.entry)
    }
}

type ConnectionMap<'a> = HashMap<(HashableEntry<'a>, HashableEntry<'a>), i32>;

/**
 * A hash map vocabulary.
 */
#[derive(Clone)]
pub struct HashMapVocabulary<'a> {
    entry_map: EntryMap,
    connection_map: ConnectionMap<'a>,
    entry_hash_value: &'a dyn Fn(&Entry) -> u64,
    entry_equal: &'a dyn Fn(&Entry, &Entry) -> bool,
}

impl Debug for HashMapVocabulary<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HashMapVocabulary")
            .field("entry_map", &self.entry_map)
            .field("connection_map", &self.connection_map)
            .field(
                "entry_hash_value",
                &type_name_of_val(&self.entry_hash_value),
            )
            .field("entry_equal", &type_name_of_val(&self.entry_equal))
            .finish()
    }
}

impl<'a> HashMapVocabulary<'a> {
    /**
     * Creates a hash map vocabulary.
     *
     * # Arguments
     * * `entry_mappings`   - Entry mappings.
     * * `connections`      - Connections.
     * * `entry_hash_value` - A hash function for an entry.
     * * `entry_equal`      - An equality function for entries.
     */
    pub fn new(
        entry_mappings: Vec<(String, Vec<Entry>)>,
        connections: Vec<((Entry, Entry), i32)>,
        entry_hash_value: &'a dyn Fn(&Entry) -> u64,
        entry_equal: &'a dyn Fn(&Entry, &Entry) -> bool,
    ) -> Self {
        let entry_map = Self::make_entry_map(entry_mappings);
        let connection_map = Self::make_connection_map(connections, entry_hash_value, entry_equal);
        HashMapVocabulary {
            entry_map,
            connection_map,
            entry_hash_value,
            entry_equal,
        }
    }

    fn make_entry_map(entry_mappings: Vec<(String, Vec<Entry>)>) -> EntryMap {
        let mut entry_map = EntryMap::new();
        for (key, entries) in entry_mappings {
            let entry_rcs = entries.into_iter().map(Rc::new).collect();
            let _prev_value = entry_map.insert(key, entry_rcs);
        }
        entry_map
    }

    fn make_connection_map(
        connections: Vec<((Entry, Entry), i32)>,
        entry_hash_value: &'a dyn Fn(&Entry) -> u64,
        entry_equal: &'a dyn Fn(&Entry, &Entry) -> bool,
    ) -> ConnectionMap<'a> {
        let mut connection_map = ConnectionMap::new();
        for ((from, to), cost) in connections {
            let from = HashableEntry::new(from, entry_hash_value, entry_equal);
            let to = HashableEntry::new(to, entry_hash_value, entry_equal);
            let _prev_value = connection_map.insert((from, to), cost);
        }
        connection_map
    }
}

impl Vocabulary for HashMapVocabulary<'_> {
    fn find_entries(&self, key: &dyn crate::Input) -> Result<Vec<Rc<Entry>>, Error> {
        let Some(key) = key.downcast_ref::<StringInput>() else {
            return Ok(Vec::new());
        };
        let Some(found) = self.entry_map.get(key.value()) else {
            return Ok(Vec::new());
        };

        Ok(found.clone())
    }

    fn find_connection(&self, from: &Node, to: &Entry) -> Result<Connection, Error> {
        let from_entry = from.entry().as_ref().clone();
        let key = (
            HashableEntry::new(from_entry, self.entry_hash_value, self.entry_equal),
            HashableEntry::new(to.clone(), self.entry_hash_value, self.entry_equal),
        );
        let Some(found) = self.connection_map.get(&key) else {
            return Ok(Connection::new(i32::MAX));
        };
        Ok(Connection::new(*found))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry_hash_value(entry: &Entry) -> u64 {
        let Some(key) = entry.key() else {
            return 0;
        };
        key.hash_value()
    }

    fn entry_equal(one: &Entry, other: &Entry) -> bool {
        match (one.key(), other.key()) {
            (Some(one_key), Some(other_key)) => one_key.equal_to(other_key),
            (None, None) => true,
            _ => false,
        }
    }

    fn make_node(entry: Rc<Entry>) -> Node {
        static PRECEDING_EDGE_COSTS: Vec<i32> = Vec::new();
        match entry.as_ref() {
            Entry::BosEos => Node::bos(Rc::new(PRECEDING_EDGE_COSTS.clone())),
            Entry::Middle(_) => Node::new_with_entry(
                entry,
                0,
                usize::MAX,
                Rc::new(PRECEDING_EDGE_COSTS.clone()),
                usize::MAX,
                i32::MAX,
            )
            .unwrap(),
        }
    }

    #[test]
    fn new() {
        {
            let entry_mappings = Vec::<(String, Vec<Entry>)>::new();
            let connections = Vec::<((Entry, Entry), i32)>::new();
            let _vocaburary = HashMapVocabulary::new(
                entry_mappings,
                connections,
                &entry_hash_value,
                &entry_equal,
            );
        }
        {
            let entry_mappings = vec![
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
            let _vocaburary = HashMapVocabulary::new(
                entry_mappings,
                connections,
                &entry_hash_value,
                &entry_equal,
            );
        }
    }

    #[test]
    fn find_entries() {
        {
            let entry_mappings = Vec::<(String, Vec<Entry>)>::new();
            let connections = Vec::<((Entry, Entry), i32)>::new();
            let vocaburary = HashMapVocabulary::new(
                entry_mappings,
                connections,
                &entry_hash_value,
                &entry_equal,
            );

            {
                let found = vocaburary
                    .find_entries(&StringInput::new(String::from("みずほ")))
                    .unwrap();
                assert!(found.is_empty());
            }
            {
                let found = vocaburary
                    .find_entries(&StringInput::new(String::from("さくら")))
                    .unwrap();
                assert!(found.is_empty());
            }
        }
        {
            let entry_mappings = vec![
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
            let vocaburary = HashMapVocabulary::new(
                entry_mappings,
                connections,
                &entry_hash_value,
                &entry_equal,
            );

            {
                let found = vocaburary
                    .find_entries(&StringInput::new(String::from("みずほ")))
                    .unwrap();
                assert_eq!(found.len(), 1);
                assert_eq!(
                    found[0]
                        .key()
                        .unwrap()
                        .downcast_ref::<StringInput>()
                        .unwrap()
                        .value(),
                    "みずほ"
                );
                assert_eq!(
                    found[0].value().unwrap().downcast_ref::<String>().unwrap(),
                    "瑞穂"
                );
                assert_eq!(found[0].cost(), 42);
            }
            {
                let found = vocaburary
                    .find_entries(&StringInput::new(String::from("さくら")))
                    .unwrap();
                assert_eq!(found.len(), 2);
                assert_eq!(
                    found[0]
                        .key()
                        .unwrap()
                        .downcast_ref::<StringInput>()
                        .unwrap()
                        .value(),
                    "さくら"
                );
                assert_eq!(
                    found[0].value().unwrap().downcast_ref::<String>().unwrap(),
                    "桜"
                );
                assert_eq!(found[0].cost(), 24);
                assert_eq!(
                    found[1]
                        .key()
                        .unwrap()
                        .downcast_ref::<StringInput>()
                        .unwrap()
                        .value(),
                    "さくら"
                );
                assert_eq!(
                    found[1].value().unwrap().downcast_ref::<String>().unwrap(),
                    "さくら"
                );
                assert_eq!(found[1].cost(), 2424);
            }
        }
    }

    #[test]
    fn find_connection() {
        {
            let entry_mappings = vec![
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
            let connections = vec![
                (
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
                ),
                ((Entry::BosEos, Entry::BosEos), 999),
            ];
            let vocaburary = HashMapVocabulary::new(
                entry_mappings,
                connections,
                &entry_hash_value,
                &entry_equal,
            );

            let entries_mizuho = vocaburary
                .find_entries(&StringInput::new(String::from("みずほ")))
                .unwrap();
            assert_eq!(entries_mizuho.len(), 1);
            let entries_sakura = vocaburary
                .find_entries(&StringInput::new(String::from("さくら")))
                .unwrap();
            assert_eq!(entries_sakura.len(), 2);

            {
                let connection = vocaburary
                    .find_connection(&make_node(entries_mizuho[0].clone()), &entries_sakura[0])
                    .unwrap();

                assert_eq!(connection.cost(), 4242);
            }
            {
                let connection = vocaburary
                    .find_connection(&Node::bos(Rc::new(Vec::new())), &Entry::BosEos)
                    .unwrap();

                assert_eq!(connection.cost(), 999);
            }
            {
                let connection = vocaburary
                    .find_connection(&make_node(entries_mizuho[0].clone()), &entries_mizuho[0])
                    .unwrap();

                assert_eq!(connection.cost(), i32::MAX);
            }
        }
    }
}
