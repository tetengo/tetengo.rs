/*!
 * A trie.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

use anyhow::Result;
use std::cell::RefCell;
use std::fmt::{self, Debug, Formatter};

use crate::double_array::{self, DoubleArray, DEFAULT_DENSITY_FACTOR};
use crate::serializer::{Serializer, SerializerOf};

/**
 * A building observer set.
 */
pub struct BuldingObserverSet<'a> {
    adding: &'a mut dyn FnMut(&[u8]),
    done: &'a mut dyn FnMut(),
}

impl<'a> BuldingObserverSet<'a> {
    /**
     * Creates a building observer set.
     *
     * # Parameters
     * * `adding` - An adding observer.
     * * `done` - A done observer.
     */
    pub fn new(adding: &'a mut dyn FnMut(&[u8]), done: &'a mut dyn FnMut()) -> Self {
        Self { adding, done }
    }

    /**
     * Calls `adding`.
     *
     * # Arguments
     * * `serialized_key` - A serialized key.
     */
    pub fn adding(&mut self, serialized_key: &[u8]) {
        (self.adding)(serialized_key);
    }

    /**
     * Calls `done`.
     */
    pub fn done(&mut self) {
        (self.done)();
    }
}

impl Debug for BuldingObserverSet<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("BuldingObserverSet")
            .field("adding", &"Box<dyn FnOnce(&[u8])>")
            .field("done", &"Box<dyn FnOnce()>")
            .finish()
    }
}

/// The default double array density factor.
const DEFAULT_DOUBLE_ARRAY_DENSITY_FACTOR: usize = DEFAULT_DENSITY_FACTOR;

/**
 * A trie.
 */
#[derive(Debug)]
pub struct Trie<Key, Value, KeySerializer: Serializer = <() as SerializerOf<Key>>::Type> {
    _phantom: std::marker::PhantomData<Key>,
    _double_array: DoubleArray<Value>,
    _key_serializer: KeySerializer,
}

impl<Key, Value: Clone + 'static, KeySerializer: Serializer> Trie<Key, Value, KeySerializer> {
    /**
     * Creates a trie.
     */
    pub fn new() -> Result<Self> {
        Self::new_with_keyserializer(KeySerializer::new(true))
    }

    /**
     * Creates a trie.
     *
     * # Arguments
     * * `key_serializer` - A key serializer.
     */
    pub fn new_with_keyserializer(key_serializer: KeySerializer) -> Result<Self> {
        Ok(Self {
            _phantom: std::marker::PhantomData,
            _double_array: DoubleArray::new()?,
            _key_serializer: key_serializer,
        })
    }

    /**
     * Creates a trie.
     *
     * # Arguments
     * * `elements` - Elements.
     */
    pub fn new_with_elements(elements: Vec<(KeySerializer::Object<'_>, Value)>) -> Result<Self> {
        Self::new_with_elements_keyserializer(elements, KeySerializer::new(true))
    }

    /**
     * Creates a trie.
     *
     * # Arguments
     * * `elements`       - Elements.
     * * `key_serializer` - A key serializer.
     */
    pub fn new_with_elements_keyserializer(
        elements: Vec<(KeySerializer::Object<'_>, Value)>,
        key_serializer: KeySerializer,
    ) -> Result<Self> {
        Self::new_with_elements_keyserializer_buildingobserverset(
            elements,
            key_serializer,
            &mut BuldingObserverSet::new(&mut |_| {}, &mut || {}),
        )
    }

    /**
     * Creates a trie.
     *
     * # Arguments
     * * `elements`              - Elements.
     * * `key_serializer`        - A key serializer.
     * * `building_observer_set` - A building observer set.
     */
    pub fn new_with_elements_keyserializer_buildingobserverset(
        elements: Vec<(KeySerializer::Object<'_>, Value)>,
        key_serializer: KeySerializer,
        building_observer_set: &mut BuldingObserverSet<'_>,
    ) -> Result<Self> {
        Self::new_with_elements_keyserializer_buildingobserverset_densityfactor(
            elements,
            key_serializer,
            building_observer_set,
            DEFAULT_DOUBLE_ARRAY_DENSITY_FACTOR,
        )
    }

    /**
     * Creates a trie.
     *
     * # Arguments
     * * `elements`                    - Elements.
     * * `key_serializer`              - A key serializer.
     * * `building_observer_set`       - A building observer set.
     * * `double_array_density_factor` - A double array density factor.
     */
    pub fn new_with_elements_keyserializer_buildingobserverset_densityfactor(
        elements: Vec<(KeySerializer::Object<'_>, Value)>,
        key_serializer: KeySerializer,
        building_observer_set: &mut BuldingObserverSet<'_>,
        double_array_density_factor: usize,
    ) -> Result<Self> {
        let mut double_array_content_keys = Vec::<Vec<u8>>::with_capacity(elements.len());
        for element in &elements {
            let (key, _) = &element;
            let serialized_key = key_serializer.serialize(key);
            double_array_content_keys.push(serialized_key);
        }
        let mut double_array_contents = Vec::<(&[u8], i32)>::with_capacity(elements.len());
        for (i, _) in elements.iter().enumerate() {
            double_array_contents.push((&double_array_content_keys[i], i as i32));
        }

        let building_observer_set_ref_cell = RefCell::new(building_observer_set);
        let adding = &mut |&(key, _): &(&[u8], i32)| {
            building_observer_set_ref_cell.borrow_mut().adding(key);
        };
        let done = &mut || {
            building_observer_set_ref_cell.borrow_mut().done();
        };
        let observer_set = &mut double_array::BuldingObserverSet::new(adding, done);

        let mut double_array =
            DoubleArray::<Value>::new_with_elements_buldingobserverset_densityfactor(
                double_array_contents,
                observer_set,
                double_array_density_factor,
            )?;

        for (i, element) in elements.into_iter().enumerate() {
            let (_, value) = element;
            double_array.storage_mut().add_value_at(i, value)?;
        }

        Ok(Self {
            _phantom: std::marker::PhantomData,
            _double_array: double_array,
            _key_serializer: key_serializer,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::serializer::Deserializer;
    use crate::string_serializer::{StringDeserializer, StringSerializer};

    use super::*;

    const KUMAMOTO: &str = "熊本";

    static TAMANA: &str = "玉名";

    static _TAMARAI: &str = "玉来";

    static _TAMA: &str = "玉";

    static _UTO: &str = "宇土";

    #[test]
    fn test_new() {
        let _trie = Trie::<&str, i32>::new().unwrap();
    }

    #[test]
    fn new_with_keyserializer() {
        let key_serializer = StringSerializer::new(true);
        let _trie = Trie::<&str, i32>::new_with_keyserializer(key_serializer).unwrap();
    }

    #[test]
    fn new_with_elements() {
        {
            let _trie =
                Trie::<&str, i32>::new_with_elements([("Kumamoto", 42), ("Tamana", 24)].to_vec())
                    .unwrap();
        }
        {
            let _trie = Trie::<&str, String>::new_with_elements(
                [
                    (KUMAMOTO, KUMAMOTO.to_string()),
                    (TAMANA, TAMANA.to_string()),
                ]
                .to_vec(),
            )
            .unwrap();
        }
    }

    #[test]
    fn new_with_elements_keyserializer() {
        let content = [
            ("kumamoto", KUMAMOTO.to_string()),
            ("tamana", TAMANA.to_string()),
        ]
        .to_vec();
        let _trie = Trie::<&str, String>::new_with_elements_keyserializer(
            content,
            StringSerializer::new(true),
        )
        .unwrap();
    }

    #[test]
    fn new_with_elements_keyserializer_buildingobserverset() {
        let mut added_serialized_keys = Vec::<Vec<u8>>::new();
        let mut done = false;
        let _trie = Trie::<&str, i32>::new_with_elements_keyserializer_buildingobserverset(
            [("Kumamoto", 42), ("Tamana", 24)].to_vec(),
            StringSerializer::new(true),
            &mut BuldingObserverSet::new(
                &mut |serialized_keys| {
                    added_serialized_keys.push(serialized_keys.to_vec());
                },
                &mut || {
                    done = true;
                },
            ),
        )
        .unwrap();

        let key_deserializer = StringDeserializer::new(true);
        assert_eq!(added_serialized_keys.len(), 2);
        assert_eq!(
            key_deserializer
                .deserialize(added_serialized_keys[0].as_ref())
                .unwrap(),
            "Kumamoto"
        );
        assert_eq!(
            key_deserializer
                .deserialize(added_serialized_keys[1].as_ref())
                .unwrap(),
            "Tamana"
        );
        assert!(done);
    }

    #[test]
    fn new_with_elements_buldingobserverset_densityfactor() {
        let mut added_serialized_keys = Vec::<Vec<u8>>::new();
        let mut done = false;
        let _trie =
            Trie::<&str, i32>::new_with_elements_keyserializer_buildingobserverset_densityfactor(
                [("Kumamoto", 42), ("Tamana", 24)].to_vec(),
                StringSerializer::new(true),
                &mut BuldingObserverSet::new(
                    &mut |serialized_keys| {
                        added_serialized_keys.push(serialized_keys.to_vec());
                    },
                    &mut || {
                        done = true;
                    },
                ),
                DEFAULT_DOUBLE_ARRAY_DENSITY_FACTOR,
            )
            .unwrap();
    }
}
