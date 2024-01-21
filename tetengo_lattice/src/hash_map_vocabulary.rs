/*!
 * A hash map vocabulary.
 *
 * Copyright (C) 2023-2024 kaoru  <https://www.tetengo.org/>
 */

use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

use crate::connection::Connection;
use crate::entry::{Entry, EntryView};
use crate::node::Node;
use crate::string_input::StringInput;
use crate::vocabulary::Vocabulary;

type EntryMap = HashMap<String, Vec<Entry>>;

#[derive(Clone)]
struct HashableEntryEntity<'a> {
    entry: Entry,
    hash_value: &'a dyn Fn(&EntryView<'_>) -> u64,
    equal: &'a dyn Fn(&EntryView<'_>, &EntryView<'_>) -> bool,
}

impl Debug for HashableEntryEntity<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HashableEntryEntity")
            .field("entry", &self.entry)
            .field("hash_value", &"&'a dyn Fn(&EntryView<'_>) -> u64")
            .field(
                "equal",
                &"&'a dyn Fn(&EntryView<'_>, &EntryView<'_>) -> bool",
            )
            .finish()
    }
}

#[derive(Clone)]
struct HashableEntryView<'a> {
    entry_view: EntryView<'a>,
    hash_value: &'a dyn Fn(&EntryView<'_>) -> u64,
    equal: &'a dyn Fn(&EntryView<'_>, &EntryView<'_>) -> bool,
}

impl Debug for HashableEntryView<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HashableEntryEntity")
            .field("entry_view", &self.entry_view)
            .field("hash_value", &"&'a dyn Fn(&EntryView<'_>) -> u64")
            .field(
                "equal",
                &"&'a dyn Fn(&EntryView<'_>, &EntryView<'_>) -> bool",
            )
            .finish()
    }
}

#[derive(Clone, Debug)]
enum HashableEntry<'a> {
    Entity(HashableEntryEntity<'a>),
    View(HashableEntryView<'a>),
}

impl<'a> HashableEntry<'a> {
    fn from_entity(
        entry: Entry,
        hash_value: &'a dyn Fn(&EntryView<'_>) -> u64,
        equal: &'a dyn Fn(&EntryView<'_>, &EntryView<'_>) -> bool,
    ) -> Self {
        HashableEntry::Entity(HashableEntryEntity {
            entry,
            hash_value,
            equal,
        })
    }

    fn from_view(
        entry_view: EntryView<'a>,
        hash_value: &'a dyn Fn(&EntryView<'_>) -> u64,
        equal: &'a dyn Fn(&EntryView<'_>, &EntryView<'_>) -> bool,
    ) -> Self {
        HashableEntry::View(HashableEntryView {
            entry_view,
            hash_value,
            equal,
        })
    }
}

impl Eq for HashableEntry<'_> {}

impl Hash for HashableEntry<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            HashableEntry::Entity(entity) => {
                let entry_view = entity.entry.as_view();
                let hash_value = (entity.hash_value)(&entry_view);
                hash_value.hash(state);
            }
            HashableEntry::View(view) => {
                let hash_value = (view.hash_value)(&view.entry_view);
                hash_value.hash(state);
            }
        }
    }
}

impl PartialEq for HashableEntry<'_> {
    fn eq(&self, other: &Self) -> bool {
        let self_entry_view = match self {
            HashableEntry::Entity(entity) => entity.entry.as_view(),
            HashableEntry::View(view) => view.entry_view.clone(),
        };
        let other_entry_view = match other {
            HashableEntry::Entity(entity) => entity.entry.as_view(),
            HashableEntry::View(view) => view.entry_view.clone(),
        };
        let equal = match self {
            HashableEntry::Entity(entity) => entity.equal,
            HashableEntry::View(view) => view.equal,
        };
        equal(&self_entry_view, &other_entry_view)
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
    entry_hash_value: &'a dyn Fn(&EntryView<'_>) -> u64,
    entry_equal: &'a dyn Fn(&EntryView<'_>, &EntryView<'_>) -> bool,
}

impl Debug for HashMapVocabulary<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HashableEntryEntity")
            .field("entry_map", &self.entry_map)
            .field("connection_map", &self.connection_map)
            .field("hash_value", &"&'a dyn Fn(&EntryView<'_>) -> u64")
            .field(
                "equal",
                &"&'a dyn Fn(&EntryView<'_>, &EntryView<'_>) -> bool",
            )
            .finish()
    }
}

impl<'a> HashMapVocabulary<'a> {
    /**
     * Creates a hash map vocabulary.
     *
     * # Arguments
     * * `entries`          - Entries.
     * * `connections`      - Connections.
     * * `entry_hash_value` - A hash function for an entry.
     * * `entry_equal`      - An equality function for entries.
     */
    pub fn new(
        entries: Vec<(String, Vec<Entry>)>,
        connections: Vec<((Entry, Entry), i32)>,
        entry_hash_value: &'a dyn Fn(&EntryView<'_>) -> u64,
        entry_equal: &'a dyn Fn(&EntryView<'_>, &EntryView<'_>) -> bool,
    ) -> Self {
        let entry_map = Self::make_entry_map(entries);
        let connection_map = Self::make_connection_map(connections, entry_hash_value, entry_equal);
        HashMapVocabulary {
            entry_map,
            connection_map,
            entry_hash_value,
            entry_equal,
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
        entry_hash_value: &'a dyn Fn(&EntryView<'_>) -> u64,
        entry_equal: &'a dyn Fn(&EntryView<'_>, &EntryView<'_>) -> bool,
    ) -> ConnectionMap<'a> {
        let mut connection_map = ConnectionMap::new();
        for ((from, to), cost) in connections {
            let from = HashableEntry::from_entity(from, entry_hash_value, entry_equal);
            let to = HashableEntry::from_entity(to, entry_hash_value, entry_equal);
            let _prev_value = connection_map.insert((from, to), cost);
        }
        connection_map
    }
}

impl Vocabulary for HashMapVocabulary<'_> {
    fn find_entries(&self, key: &dyn crate::Input) -> Vec<EntryView<'_>> {
        let Some(key) = key.as_any().downcast_ref::<StringInput>() else {
            return Vec::new();
        };
        let Some(found) = self.entry_map.get(key.value()) else {
            return Vec::new();
        };

        found.iter().map(|entry| entry.as_view()).collect()
    }

    fn find_connection(&self, from: &Node<'_>, to: &EntryView<'_>) -> Connection {
        let Some(from_key) = from.key() else {
            return Connection::new(i32::MAX);
        };
        let Some(from_value) = from.value() else {
            return Connection::new(i32::MAX);
        };
        let from_entry_view = EntryView::new(from_key, from_value, from.node_cost());
        let key = (
            HashableEntry::from_view(from_entry_view, self.entry_hash_value, self.entry_equal),
            HashableEntry::from_view(to.clone(), self.entry_hash_value, self.entry_equal),
        );
        let Some(found) = self.connection_map.get(&key) else {
            return Connection::new(i32::MAX);
        };
        Connection::new(*found)
    }
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

    fn make_node<'a>(entry: &'a EntryView<'_>) -> Node<'a> {
        static PRECEDING_EDGE_COSTS: Vec<i32> = Vec::new();
        match entry {
            EntryView::BosEos => Node::bos(&PRECEDING_EDGE_COSTS),
            EntryView::Middle(_) => Node::new_with_entry_view(
                entry,
                0,
                usize::MAX,
                &PRECEDING_EDGE_COSTS,
                usize::MAX,
                i32::MAX,
            )
            .unwrap(),
        }
    }

    #[test]
    fn new() {
        {
            let entries = Vec::<(String, Vec<Entry>)>::new();
            let connections = Vec::<((Entry, Entry), i32)>::new();
            let _vocaburary =
                HashMapVocabulary::new(entries, connections, &entry_hash_value, &entry_equal);
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
                HashMapVocabulary::new(entries, connections, &entry_hash_value, &entry_equal);
        }
    }

    #[test]
    fn find_entries() {
        {
            let entries = Vec::<(String, Vec<Entry>)>::new();
            let connections = Vec::<((Entry, Entry), i32)>::new();
            let vocaburary =
                HashMapVocabulary::new(entries, connections, &entry_hash_value, &entry_equal);

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
                HashMapVocabulary::new(entries, connections, &entry_hash_value, &entry_equal);

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

    #[test]
    fn find_connection() {
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
                HashMapVocabulary::new(entries, connections, &entry_hash_value, &entry_equal);

            let entries_mizuho = vocaburary.find_entries(&StringInput::new(String::from("みずほ")));
            assert_eq!(entries_mizuho.len(), 1);
            let entries_sakura = vocaburary.find_entries(&StringInput::new(String::from("さくら")));
            assert_eq!(entries_sakura.len(), 2);

            {
                let connection =
                    vocaburary.find_connection(&make_node(&entries_mizuho[0]), &entries_sakura[0]);

                assert_eq!(connection.cost(), 4242);
            }
            {
                let connection =
                    vocaburary.find_connection(&make_node(&entries_mizuho[0]), &entries_mizuho[0]);

                assert_eq!(connection.cost(), i32::MAX);
            }
        }
    }
}
