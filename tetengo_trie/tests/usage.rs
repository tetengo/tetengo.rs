mod usage {
    use std::cell::RefCell;
    use tetengo_trie::{BuldingObserverSet, Serializer, StringSerializer, Trie};

    #[test]
    fn usage() {
        // Prepares a trie building observer set.
        // The observer set records the inserted keys and the end.
        let building_observer_reports = RefCell::<Vec<String>>::new(Vec::new());
        let mut adding = |key: &[u8]| {
            building_observer_reports
                .borrow_mut()
                .push(String::from_utf8(key.to_vec()).unwrap());
        };
        let mut done = || {
            building_observer_reports
                .borrow_mut()
                .push("DONE".to_string());
        };
        let mut building_observer_set = BuldingObserverSet::new(&mut adding, &mut done);

        // Builds a trie with initial elements.
        let trie = Trie::<&str, i32>::builder()
            .elements(
                [
                    ("tasakibashi", -5),
                    ("nihongiguchi", -3),
                    ("kumamotoekimae", 0),
                    ("gionbashi", 5),
                    ("gofukumachi", 10),
                    ("kawaramachi", 14),
                    ("keitokukoumae", 18),
                    ("karashimachou", 22),
                ]
                .to_vec(),
            )
            .key_serializer(StringSerializer::new(true))
            .build_with_observer_set(&mut building_observer_set)
            .unwrap();
        let stored_keys = &*building_observer_reports.borrow();
        let expected = [
            "gionbashi".to_string(),
            "gofukumachi".to_string(),
            "karashimachou".to_string(),
            "kawaramachi".to_string(),
            "keitokukoumae".to_string(),
            "kumamotoekimae".to_string(),
            "nihongiguchi".to_string(),
            "tasakibashi".to_string(),
            "DONE".to_string(),
        ]
        .to_vec();
        assert_eq!(stored_keys, &expected);

        // Searches the trie.
        // If a perfect-matching key is found, its value is returned.
        let found_for_gionbashi = trie.find("gionbashi").unwrap().unwrap();
        assert_eq!(*found_for_gionbashi, 5);

        // If not found, None is returned.
        let found_for_hanabatachou = trie.find("hanabatachou").unwrap();
        assert!(found_for_hanabatachou.is_none());

        // Creates a subtrie consisting of the elements with the common key prefix.
        let subtrie = trie.subtrie("ka").unwrap().unwrap();

        // Enumerates the values in the subtrie.
        let subtrie_values = subtrie.iter().map(|v| *v).collect::<Vec<_>>();
        assert_eq!(
            subtrie_values,
            [
                22, // karashimachou
                14, // kawaramachi
            ]
            .to_vec()
        );
    }
}
