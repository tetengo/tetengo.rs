mod usage {
    use anyhow as _;
    use hashlink as _;
    use memmap2 as _;
    use once_cell as _;
    use tempfile as _;
    use thiserror as _;

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
        let _trie = Trie::<&str, i32>::new_with_elements_keyserializer_buildingobserverset(
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
            StringSerializer::new(true),
            &mut building_observer_set,
        )
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
    }
    // void search()
    // {
    //     // Searches the trie.
    //     // If a perfect-matching key is found, its value is returned.
    //     [[maybe_unused]] const int* const p_found_for_gionbashi = trie_.find("gionbashi");
    //     assert(p_found_for_gionbashi);
    //     assert(*p_found_for_gionbashi == 5);

    //     // If not found, nullptr is returned.
    //     [[maybe_unused]] const int* const p_found_for_hanabatachou = trie_.find("hanabatachou");
    //     assert(!p_found_for_hanabatachou);

    //     // Creates a subtrie consisting of the elements with the common key prefix.
    //     const auto p_subtrie = trie_.subtrie("ka");

    //     // Enumerates the values in the subtrie.
    //     std::vector<int> subtrie_values{};
    //     std::copy(std::begin(*p_subtrie), std::end(*p_subtrie), std::back_inserter(subtrie_values));
    //     assert(
    //         (subtrie_values == std::vector<int>{
    //                                22, // karashimachou
    //                                14, // kawaramachi
    //                            }));
    // }
}
