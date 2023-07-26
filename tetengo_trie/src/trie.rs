/*!
 * A trie.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

use anyhow::Result;
use std::cell::RefCell;
use std::fmt::{self, Debug, Formatter};
use std::rc::Rc;

use crate::double_array::{self, DoubleArray, DEFAULT_DENSITY_FACTOR};
use crate::serializer::{Serializer, SerializerOf};
use crate::storage::Storage;
use crate::trie_iterator::TrieIterator;

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
 *
 * # Type Parameters
 * * `Key`           - A key type.
 * * `Value`         - A value type.
 * * `KeySerializer` - A key serializer type.
 */
#[derive(Debug)]
pub struct Trie<Key, Value, KeySerializer: Serializer = <() as SerializerOf<Key>>::Type> {
    _phantom: std::marker::PhantomData<Key>,
    double_array: DoubleArray<Value>,
    _key_serializer: KeySerializer,
}

impl<Key, Value: Clone + 'static, KeySerializer: Serializer> Trie<Key, Value, KeySerializer> {
    /**
     * Creates a trie.
     *
     * # Errors
     * * When it fails to access the storage.
     */
    pub fn new() -> Result<Self> {
        Self::new_with_keyserializer(KeySerializer::new(true))
    }

    /**
     * Creates a trie.
     *
     * # Arguments
     * * `key_serializer` - A key serializer.
     *
     * # Errors
     * * When it fails to access the storage.
     */
    pub fn new_with_keyserializer(key_serializer: KeySerializer) -> Result<Self> {
        Ok(Self {
            _phantom: std::marker::PhantomData,
            double_array: DoubleArray::new()?,
            _key_serializer: key_serializer,
        })
    }

    /**
     * Creates a trie.
     *
     * # Arguments
     * * `elements` - Elements.
     *
     * # Errors
     * * When it fails to access the storage.
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
     *
     * # Errors
     * * When it fails to access the storage.
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
     *
     * # Errors
     * * When it fails to access the storage.
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
     *
     * # Errors
     * * When it fails to access the storage.
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
            double_array,
            _key_serializer: key_serializer,
        })
    }

    /**
     * Creates a trie.
     *
     * # Arguments
     * * `storage` - A storage.
     */
    pub fn new_with_storage(storage: Box<dyn Storage<Value>>) -> Self {
        Self::new_with_storage_keyserializer(storage, KeySerializer::new(true))
    }

    /**
     * Creates a trie.
     *
     * # Arguments
     * * `storage`        - A storage.
     * * `key_serializer` - A key serializer.
     */
    pub fn new_with_storage_keyserializer(
        storage: Box<dyn Storage<Value>>,
        key_serializer: KeySerializer,
    ) -> Self {
        Self {
            _phantom: std::marker::PhantomData,
            double_array: DoubleArray::new_with_storage(storage, 0),
            _key_serializer: key_serializer,
        }
    }

    /**
     * Returns true when the trie is empty.
     *
     * # Returns
     * True when the trie is empty.
     *
     * # Errors
     * * When it fails to access the storage.
     */
    pub fn is_empty(&self) -> Result<bool> {
        Ok(self.double_array.storage().value_count()? == 0)
    }

    /**
     * Returns the size of the trie.
     *
     * # Returns
     * The size.
     *
     * # Errors
     * * When it fails to access the storage.
     */
    pub fn size(&self) -> Result<usize> {
        self.double_array.storage().value_count()
    }

    /**
     * Returns true when the trie contains the given key.
     *
     * # Arguments
     * * `key` - A key.
     *
     * # Returns
     * True when the trie contains the given key.
     *
     * # Errors
     * * When it fails to access the storage.
     */
    pub fn contains(&self, key: KeySerializer::Object<'_>) -> Result<bool> {
        let serialized_key = self._key_serializer.serialize(&key);
        Ok(self.double_array.find(&serialized_key)?.is_some())
    }

    /**
     * Finds the value object correspoinding the given key.
     *
     * # Arguments
     * * `key` - A key.
     *
     * # Returns
     * The value object. Or None when the trie does not have the given key.
     *
     * # Errors
     * * When it fails to access the storage.
     */
    pub fn find(&self, key: KeySerializer::Object<'_>) -> Result<Option<Rc<Value>>> {
        let serialized_key = self._key_serializer.serialize(&key);
        let index = self.double_array.find(&serialized_key)?;
        let Some(index) = index else {
            return Ok(None);
        };

        self.double_array.storage().value_at(index as usize)
    }

    /**
     * Returns an iterator.
     *
     * # Returns
     * A double array iterator.
     */
    pub fn iter(&self) -> TrieIterator<'_, Value> {
        TrieIterator::new(self.double_array.iter(), self.double_array.storage())
    }

    // /*!
    //     \brief Returns a subtrie.

    //     \param key_prefix A key prefix.

    //     \return A unique pointer to a subtrie.
    //             Or nullptr when the trie does not have the given key prefix.
    // */
    // [[nodiscard]] std::unique_ptr<trie> subtrie(const key_type& key_prefix) const
    // {
    //     auto p_trie_impl = [this, &key_prefix]() {
    //         if constexpr (std::is_same_v<key_type, std::string_view> || std::is_same_v<key_type, std::string>)
    //         {
    //             return m_impl.subtrie(m_key_serializer(key_prefix));
    //         }
    //         else
    //         {
    //             const auto serialized_key_prefix = m_key_serializer(key_prefix);
    //             return m_impl.subtrie(
    //                 std::string_view{ std::data(serialized_key_prefix), std::size(serialized_key_prefix) });
    //         }
    //     }();
    //     if (!p_trie_impl)
    //     {
    //         return nullptr;
    //     }
    //     std::unique_ptr<trie> p_trie{ new trie{ std::move(p_trie_impl), m_key_serializer } };
    //     return p_trie;
    // }
    // std::unique_ptr<trie_impl> subtrie(const std::string_view& key_prefix) const
    // {
    //     auto p_subtrie = m_p_double_array->subtrie(key_prefix);
    //     if (!p_subtrie)
    //     {
    //         return nullptr;
    //     }
    //     return std::make_unique<trie_impl>(std::move(p_subtrie));
    // }

    /**
     * Returns a subtrie.
     *
     * # Arguments
     * * `key_prefix` - A key prefix.
     *
     * # Returns
     * A subtrie. Or None when the trie does not have the given key prefix.
     */
    pub fn subtrie(&self, _key_prefix: KeySerializer::Object<'_>) -> Result<Option<Self>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use once_cell::sync::Lazy;
    use std::io::Cursor;

    use crate::memory_storage::MemoryStorage;
    use crate::serializer::Deserializer;
    use crate::string_serializer::{StringDeserializer, StringSerializer};
    use crate::value_serializer::ValueDeserializer;

    use super::*;

    const KUMAMOTO: &str = "熊本";

    static TAMANA: &str = "玉名";

    static _TAMARAI: &str = "玉来";

    static _TAMA: &str = "玉";

    static UTO: &str = "宇土";

    #[rustfmt::skip]
    const SERIALIZED: [u8;76] = [
        // base check array
        0x00u8, 0x00u8, 0x00u8, 0x0Bu8,
        0xFFu8, 0xFFu8, 0x90u8, 0xFFu8,
        0xFFu8, 0xFFu8, 0x78u8, 0x71u8,
        0xFFu8, 0xFFu8, 0x9Du8, 0x8Au8,
        0xFFu8, 0xFFu8, 0x7Eu8, 0x73u8,
        0xFFu8, 0xFFu8, 0xD9u8, 0x67u8,
        0x00u8, 0x00u8, 0x06u8, 0x2Cu8,
        0x00u8, 0x00u8, 0x00u8, 0x00u8,
        0xFFu8, 0xFFu8, 0xB4u8, 0x89u8,
        0xFFu8, 0xFFu8, 0xFCu8, 0x54u8,
        0x00u8, 0x00u8, 0x0Au8, 0x0Du8,
        0x00u8, 0x00u8, 0x01u8, 0x00u8,

        // value array
        0x00u8, 0x00u8, 0x00u8, 0x02u8,
        0x00u8, 0x00u8, 0x00u8, 0x00u8,
        0x00u8, 0x00u8, 0x00u8, 0x06u8,
        0xE7u8, 0x86u8, 0x8Au8, 0xE6u8, 0x9Cu8, 0xACu8,
        0x00u8, 0x00u8, 0x00u8, 0x06u8,
        0xE7u8, 0x8Eu8, 0x89u8, 0xE5u8, 0x90u8, 0x8Du8,
    ];

    fn create_input_stream() -> Box<dyn std::io::Read> {
        Box::new(Cursor::new(SERIALIZED))
    }

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

    #[test]
    fn new_with_storage() {
        let mut reader = create_input_stream();
        let value_deserializer = ValueDeserializer::new(|serialized| {
            static STRING_DESERIALIZER: Lazy<StringDeserializer> =
                Lazy::new(|| StringDeserializer::new(false));
            STRING_DESERIALIZER.deserialize(serialized)
        });
        let storage =
            Box::new(MemoryStorage::from_reader(&mut reader, &value_deserializer).unwrap());
        let _trie = Trie::<&str, String>::new_with_storage(storage);
    }

    #[test]
    fn new_with_storage_keyserializer() {
        let mut reader = create_input_stream();
        let value_deserializer = ValueDeserializer::new(|serialized| {
            static STRING_DESERIALIZER: Lazy<StringDeserializer> =
                Lazy::new(|| StringDeserializer::new(false));
            STRING_DESERIALIZER.deserialize(serialized)
        });
        let storage =
            Box::new(MemoryStorage::from_reader(&mut reader, &value_deserializer).unwrap());
        let _trie = Trie::<&str, String>::new_with_storage_keyserializer(
            storage,
            StringSerializer::new(true),
        );
    }

    #[test]
    fn is_empy() {
        {
            let trie = Trie::<&str, String>::new().unwrap();

            assert!(trie.is_empty().unwrap());
        }
        {
            let trie = Trie::<&str, String>::new_with_elements(
                [(KUMAMOTO, KUMAMOTO.to_string())].to_vec(),
            )
            .unwrap();

            assert!(!trie.is_empty().unwrap());
        }
        {
            let trie = Trie::<&str, String>::new_with_elements(
                [
                    (KUMAMOTO, KUMAMOTO.to_string()),
                    (TAMANA, TAMANA.to_string()),
                ]
                .to_vec(),
            )
            .unwrap();

            assert!(!trie.is_empty().unwrap());
        }
    }

    #[test]
    fn size() {
        {
            let trie = Trie::<&str, String>::new().unwrap();

            assert_eq!(trie.size().unwrap(), 0);
        }
        {
            let trie = Trie::<&str, String>::new_with_elements(
                [(KUMAMOTO, KUMAMOTO.to_string())].to_vec(),
            )
            .unwrap();

            assert_eq!(trie.size().unwrap(), 1);
        }
        {
            let trie = Trie::<&str, String>::new_with_elements(
                [
                    (KUMAMOTO, KUMAMOTO.to_string()),
                    (TAMANA, TAMANA.to_string()),
                ]
                .to_vec(),
            )
            .unwrap();

            assert_eq!(trie.size().unwrap(), 2);
        }
    }

    #[test]
    fn contains() {
        {
            let trie = Trie::<&str, String>::new().unwrap();

            assert!(!trie.contains(KUMAMOTO).unwrap());
        }
        {
            let trie = Trie::<&str, String>::new_with_elements(
                [
                    (KUMAMOTO, KUMAMOTO.to_string()),
                    (TAMANA, TAMANA.to_string()),
                ]
                .to_vec(),
            )
            .unwrap();

            assert!(trie.contains(KUMAMOTO).unwrap());
            assert!(trie.contains(TAMANA).unwrap());
            assert!(!trie.contains(UTO).unwrap());
        }
    }

    #[test]
    fn find() {
        {
            let trie = Trie::<&str, String>::new().unwrap();

            let found = trie.find(KUMAMOTO).unwrap();
            assert!(found.is_none());
        }
        {
            let trie = Trie::<&str, String>::new_with_elements(
                [
                    (KUMAMOTO, KUMAMOTO.to_string()),
                    (TAMANA, TAMANA.to_string()),
                ]
                .to_vec(),
            )
            .unwrap();

            {
                let found = trie.find(KUMAMOTO).unwrap().unwrap();
                assert_eq!(*found, KUMAMOTO.to_string());
            }
            {
                let found = trie.find(TAMANA).unwrap().unwrap();
                assert_eq!(*found, TAMANA.to_string());
            }
            {
                let found = trie.find(UTO).unwrap();
                assert!(found.is_none());
            }
        }
    }

    #[test]
    fn iter() {
        {
            let trie = Trie::<&str, String>::new().unwrap();

            let _iterator = trie.iter();
        }
        {
            let trie = Trie::<&str, String>::new_with_elements(
                [
                    (KUMAMOTO, KUMAMOTO.to_string()),
                    (TAMANA, TAMANA.to_string()),
                ]
                .to_vec(),
            )
            .unwrap();

            let _iterator = trie.iter();
        }
    }

    #[test]
    fn subtrie() {
        // TODO: Implement it.
    }
}
