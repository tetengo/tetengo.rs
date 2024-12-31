/*!
 * The usage of tetengo_lattice
 */

mod usage {
    use std::hash::{DefaultHasher, Hash, Hasher};
    use std::rc::Rc;

    use tetengo_lattice::{
        Constraint, Entry, EntryView, HashMapVocabulary, NBestIterator, Node, Path, StringInput,
        Vocabulary,
    };

    #[test]
    fn viterbi() {
        /*
            Makes the following lattice and searches it.

                    /-----[ab:AwaBizan]-----\
                   /  (7)      (9)      (1)  \
                  /                           \
                 /       (2)   (4)   (7)       \
            [BOS]-----[a:Alpha]---[b:Bravo]-----[EOS]
                 \ (3)         \ /(1)      (2) /
                  \(1)          X             /(6)
                   \           / \(5)        /
                    `-[a:Alice]---[b:Bob]---'
                         (1)   (9)  (8)
            Path                         Cost
            [BOS]-[Alice]-[Bravo]-[EOS]  1+1+1+7+2=12
            [BOS]---[AwaBizan]----[EOS]  7+9+1    =17
            [BOS]-[Alpha]-[Bravo]-[EOS]  3+2+4+7+2=18
            [BOS]-[Alpha]-[Bob]---[EOS]  3+2+5+8+6=24
            [BOS]-[Alice]-[Bob]---[EOS]  1+1+9+8+6=25
        */

        // Builds a vocabulary.
        let vocabulary = build_vocabulary();

        // Creates an object for a lattice.
        let mut lattice = tetengo_lattice::lattice::Lattice::new(vocabulary.as_ref());

        // Enters key characters to construct the lattice.
        let _ignored = lattice.push_back(Box::new(StringInput::new(String::from("a"))));
        let _ignored = lattice.push_back(Box::new(StringInput::new(String::from("b"))));

        // Finishes the lattice construction.
        let eos = lattice.settle().unwrap();

        // Creates an iterator to enumerate the paths in the lattice.
        let iterator = NBestIterator::new(&lattice, eos, Box::new(Constraint::new()));

        // Enumerates the paths.
        let paths = iterator.map(|path| to_string(&path)).collect::<Vec<_>>();
        let expected = vec![
            String::from("[BOS]-[Alice]-[Bravo]-[EOS] (12)"),
            String::from("[BOS]-[AwaBizan]-[EOS] (17)"),
            String::from("[BOS]-[Alpha]-[Bravo]-[EOS] (18)"),
            String::from("[BOS]-[Alpha]-[Bob]-[EOS] (24)"),
            String::from("[BOS]-[Alice]-[Bob]-[EOS] (25)"),
        ];
        assert_eq!(paths, expected);
    }

    fn build_vocabulary() -> Box<dyn Vocabulary> {
        // The contents of the vocabulary.
        let entries = [
            Entry::new(
                Rc::new(StringInput::new(String::from("a"))),
                Rc::new(String::from("Alpha")),
                2,
            ),
            Entry::new(
                Rc::new(StringInput::new(String::from("b"))),
                Rc::new(String::from("Bravo")),
                7,
            ),
            Entry::new(
                Rc::new(StringInput::new(String::from("a"))),
                Rc::new(String::from("Alice")),
                1,
            ),
            Entry::new(
                Rc::new(StringInput::new(String::from("b"))),
                Rc::new(String::from("Bob")),
                8,
            ),
            Entry::new(
                Rc::new(StringInput::new(String::from("ab"))),
                Rc::new(String::from("AwaBizan")),
                9,
            ),
        ];
        let entry_mappings = vec![
            (
                String::from("a"),
                vec![entries[0].clone(), entries[2].clone()],
            ),
            (
                String::from("b"),
                vec![entries[1].clone(), entries[3].clone()],
            ),
            (String::from("ab"), vec![entries[4].clone()]),
        ];
        let connections = vec![
            ((Entry::BosEos, entries[0].clone()), 3),
            ((Entry::BosEos, entries[2].clone()), 1),
            ((entries[0].clone(), entries[1].clone()), 4),
            ((entries[2].clone(), entries[1].clone()), 1),
            ((entries[0].clone(), entries[3].clone()), 5),
            ((entries[2].clone(), entries[3].clone()), 9),
            ((entries[1].clone(), Entry::BosEos), 2),
            ((entries[3].clone(), Entry::BosEos), 6),
            ((Entry::BosEos, entries[4].clone()), 7),
            ((entries[4].clone(), Entry::BosEos), 1),
        ];

        // Returns a vocabulary implemented with hash tables.
        Box::new(HashMapVocabulary::new(
            entry_mappings,
            connections,
            &|entry| {
                let mut hasher = DefaultHasher::new();
                hasher.write_u64(if let Some(key) = entry.key() {
                    key.hash_value()
                } else {
                    0
                });
                value_of_entry(entry).hash(&mut hasher);
                hasher.finish()
            },
            &|entry1, entry2| {
                let equal_keys = if let (Some(key1), Some(key2)) = (entry1.key(), entry2.key()) {
                    key1.equal_to(key2)
                } else {
                    entry1.key().is_none() && entry2.key().is_none()
                };
                if !equal_keys {
                    return false;
                }
                value_of_entry(entry1) == value_of_entry(entry2)
            },
        ))
    }

    fn to_string(path: &Path) -> String {
        // Each path object holds the nodes that make up itself, and the whole cost.
        let mut result = String::new();
        for node in path.nodes() {
            if !result.is_empty() {
                result += "-";
            }
            result += format!("[{}]", value_of_node(node, result.is_empty())).as_str();
        }
        result += format!(" ({})", path.cost()).as_str();
        result
    }

    fn value_of_node(node: &Node, first: bool) -> String {
        if let Some(value) = node.value() {
            // The value is stored in the Any object.
            value.downcast_ref::<String>().unwrap().clone()
        } else if first {
            String::from("BOS")
        } else {
            String::from("EOS")
        }
    }

    fn value_of_entry(entry: &EntryView) -> String {
        // The value is stored in the Any object.
        if let Some(value) = entry.value() {
            value.downcast_ref::<String>().unwrap().clone()
        } else {
            String::new()
        }
    }
}
