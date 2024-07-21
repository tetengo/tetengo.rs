/*!
 * A trie.
 *
 * Copyright (C) 2023-2024 kaoru  <https://www.tetengo.org/>
 */

use anyhow::Result;
use std::cell::RefCell;
use std::fmt::{self, Debug, Formatter};
use std::marker::PhantomData;
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
     * # Arguments
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
 * A trie builder.
 *
 * # Type Parameters
 * * `Key`           - A key type.
 * * `Value`         - A value type.
 * * `KeySerializer` - A key serializer type.
 */
#[derive(Debug)]
pub struct TrieBuilder<Key, Value, KeySerializer: Serializer> {
    phantom: PhantomData<Key>,
    elements: Vec<(KeySerializer::Object<'static>, Value)>,
    key_serializer: KeySerializer,
    double_array_density_factor: usize,
}

impl<Key, Value: Clone + 'static, KeySerializer: Serializer>
    TrieBuilder<Key, Value, KeySerializer>
{
    /**
     * Sets elements.
     */
    pub fn elements(mut self, elements: Vec<(KeySerializer::Object<'static>, Value)>) -> Self {
        self.elements = elements;
        self
    }

    /**
     * Sets a key serializer.
     */
    pub fn key_serializer(mut self, key_serializer: KeySerializer) -> Self {
        self.key_serializer = key_serializer;
        self
    }

    /**
     * Sets a double array density factor.
     */
    pub fn double_array_density_factor(mut self, double_array_density_factor: usize) -> Self {
        self.double_array_density_factor = double_array_density_factor;
        self
    }

    /**
     * Builds a trie.
     *
     * # Returns
     * A trie.
     *
     * # Errors
     * * When it fails to access the storage.
     */
    pub fn build(self) -> Result<Trie<Key, Value, KeySerializer>> {
        self.build_with_observer_set(&mut BuldingObserverSet::new(&mut |_| {}, &mut || {}))
    }

    /**
     * Builds a trie with a observer set.
     *
     * # Returns
     * A trie.
     *
     * # Errors
     * * When it fails to access the storage.
     */
    pub fn build_with_observer_set(
        self,
        building_observer_set: &mut BuldingObserverSet<'_>,
    ) -> Result<Trie<Key, Value, KeySerializer>> {
        let mut double_array_content_keys = Vec::<Vec<u8>>::with_capacity(self.elements.len());
        for element in &self.elements {
            let (key, _) = &element;
            let serialized_key = self.key_serializer.serialize(key);
            double_array_content_keys.push(serialized_key);
        }
        let mut double_array_contents = Vec::<(&[u8], i32)>::with_capacity(self.elements.len());
        for (i, _) in self.elements.iter().enumerate() {
            double_array_contents.push((&double_array_content_keys[i], i as i32));
        }

        let building_observer_set_ref_cell = RefCell::new(building_observer_set);
        let adding = &mut |&(key, _): &(&[u8], i32)| {
            building_observer_set_ref_cell.borrow_mut().adding(key);
        };
        let done = &mut || {
            building_observer_set_ref_cell.borrow_mut().done();
        };
        let observer_set = &mut double_array::BuildingObserverSet::new(adding, done);

        let mut double_array = DoubleArray::<Value>::builder()
            .elements(double_array_contents)
            .density_factor(self.double_array_density_factor)
            .build_with_observer_set(observer_set)?;

        for (i, element) in self.elements.into_iter().enumerate() {
            let (_, value) = element;
            double_array.storage_mut().add_value_at(i, value)?;
        }

        Ok(Trie {
            phantom: PhantomData,
            double_array,
            key_serializer: self.key_serializer,
        })
    }
}

/**
 * A trie builder with a storage.
 *
 * # Type Parameters
 * * `Key`           - A key type.
 * * `Value`         - A value type.
 * * `KeySerializer` - A key serializer type.
 */
pub struct TrieStorageBuilder<Key, Value: Clone, KeySerializer: Serializer> {
    phantom_key: PhantomData<Key>,
    storage: Box<dyn Storage<Value>>,
    key_serializer: KeySerializer,
}

impl<Key, Value: Clone + 'static, KeySerializer: Serializer>
    TrieStorageBuilder<Key, Value, KeySerializer>
{
    /**
     * Sets a key serializer.
     */
    pub fn key_serializer(mut self, key_serializer: KeySerializer) -> Self {
        self.key_serializer = key_serializer;
        self
    }

    /**
     * Builds a trie.
     *
     * # Returns
     * A trie.
     */
    pub fn build(self) -> Trie<Key, Value, KeySerializer> {
        Trie {
            phantom: PhantomData,
            double_array: DoubleArray::new(self.storage, 0),
            key_serializer: self.key_serializer,
        }
    }
}

impl<Key, Value: Clone, KeySerializer: Serializer> Debug
    for TrieStorageBuilder<Key, Value, KeySerializer>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("_TrieStorageBuilder")
            .field("storage", &"Box<dyn Storage<Value>>")
            .field("key_serializer", &"KeySerializer")
            .finish()
    }
}

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
    phantom: PhantomData<Key>,
    double_array: DoubleArray<Value>,
    key_serializer: KeySerializer,
}

impl<Key, Value: Clone + 'static, KeySerializer: Serializer + Clone>
    Trie<Key, Value, KeySerializer>
{
    /**
     * Creates a trie builder.
     *
     * # Returns
     * A trie builder.
     */
    pub fn builder() -> TrieBuilder<Key, Value, KeySerializer> {
        TrieBuilder {
            phantom: PhantomData,
            elements: Vec::new(),
            key_serializer: KeySerializer::new(true),
            double_array_density_factor: DEFAULT_DOUBLE_ARRAY_DENSITY_FACTOR,
        }
    }

    /**
     * Creates a trie builder with a storage.
     *
     * # Returns
     * A trie builder with a storage.
     */
    pub fn builder_with_storage(
        storage: Box<dyn Storage<Value>>,
    ) -> TrieStorageBuilder<Key, Value, KeySerializer> {
        TrieStorageBuilder {
            phantom_key: PhantomData,
            storage,
            key_serializer: KeySerializer::new(true),
        }
    }

    /**
     * Returns `true` if the trie is empty.
     *
     * # Returns
     * `true` if the trie is empty.
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
     * Returns `true` when the trie contains the given key.
     *
     * # Arguments
     * * `key` - A key.
     *
     * # Returns
     * `true` if the trie contains the given key.
     *
     * # Errors
     * * When it fails to access the storage.
     */
    pub fn contains(&self, key: KeySerializer::Object<'_>) -> Result<bool> {
        let serialized_key = self.key_serializer.serialize(&key);
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
        let serialized_key = self.key_serializer.serialize(&key);
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

    /**
     * Returns a subtrie.
     *
     * # Arguments
     * * `key_prefix` - A key prefix.
     *
     * # Returns
     * A subtrie. Or None when the trie does not have the given key prefix.
     *
     * # Errors
     * * When it fails to access the storage.
     */
    pub fn subtrie(&self, key_prefix: KeySerializer::Object<'_>) -> Result<Option<Self>> {
        let serialized_key_prefix = self.key_serializer.serialize(&key_prefix);
        let subdouble_array = self.double_array.subtrie(&serialized_key_prefix)?;
        let Some(subdouble_array) = subdouble_array else {
            return Ok(None);
        };
        Ok(Some(Self {
            phantom: PhantomData,
            double_array: subdouble_array,
            key_serializer: self.key_serializer.clone(),
        }))
    }

    /**
     * Returns the storage.
     *
     * # Returns
     * The storage.
     */
    pub fn storage(&self) -> &dyn Storage<Value> {
        self.double_array.storage()
    }
}

#[cfg(test)]
mod tests {
    use once_cell::sync::Lazy;
    use std::io::Cursor;

    use crate::memory_storage::MemoryStorage;
    use crate::serializer::Deserializer;
    use crate::string_serializer::{StrSerializer, StringDeserializer};
    use crate::value_serializer::{ValueDeserializer, ValueSerializer};

    use super::*;

    const KUMAMOTO: &str = "熊本";

    const TAMANA: &str = "玉名";

    const TAMARAI: &str = "玉来";

    const TAMA: &str = "玉";

    const UTO: &str = "宇土";

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
    fn builder() {
        {
            let _trie = Trie::<&str, i32>::builder().build().unwrap();
        }

        {
            let key_serializer = StrSerializer::new(true);
            let _trie = Trie::<&str, i32>::builder()
                .key_serializer(key_serializer)
                .build()
                .unwrap();
        }

        {
            let _trie = Trie::<&str, i32>::builder()
                .elements([("Kumamoto", 42), ("Tamana", 24)].to_vec())
                .build()
                .unwrap();
        }
        {
            let _trie = Trie::<&str, String>::builder()
                .elements(
                    [
                        (KUMAMOTO, KUMAMOTO.to_string()),
                        (TAMANA, TAMANA.to_string()),
                    ]
                    .to_vec(),
                )
                .build()
                .unwrap();
        }

        {
            let content = [
                ("kumamoto", KUMAMOTO.to_string()),
                ("tamana", TAMANA.to_string()),
            ]
            .to_vec();
            let _trie = Trie::<&str, String>::builder()
                .elements(content)
                .key_serializer(StrSerializer::new(true))
                .build()
                .unwrap();
        }

        {
            let mut added_serialized_keys = Vec::<Vec<u8>>::new();
            let mut done = false;
            let _trie = Trie::<&str, i32>::builder()
                .elements([("Kumamoto", 42), ("Tamana", 24)].to_vec())
                .key_serializer(StrSerializer::new(true))
                .build_with_observer_set(&mut BuldingObserverSet::new(
                    &mut |serialized_keys| {
                        added_serialized_keys.push(serialized_keys.to_vec());
                    },
                    &mut || {
                        done = true;
                    },
                ))
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

        {
            let mut added_serialized_keys = Vec::<Vec<u8>>::new();
            let mut done = false;
            let _trie = Trie::<&str, i32>::builder()
                .elements([("Kumamoto", 42), ("Tamana", 24)].to_vec())
                .key_serializer(StrSerializer::new(true))
                .double_array_density_factor(DEFAULT_DOUBLE_ARRAY_DENSITY_FACTOR)
                .build_with_observer_set(&mut BuldingObserverSet::new(
                    &mut |serialized_keys| {
                        added_serialized_keys.push(serialized_keys.to_vec());
                    },
                    &mut || {
                        done = true;
                    },
                ))
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
    }

    #[test]
    fn builder_with_storage() {
        {
            let mut reader = create_input_stream();
            let value_deserializer = ValueDeserializer::new(|serialized| {
                static STRING_DESERIALIZER: Lazy<StringDeserializer> =
                    Lazy::new(|| StringDeserializer::new(false));
                STRING_DESERIALIZER.deserialize(serialized)
            });
            let storage =
                Box::new(MemoryStorage::new_with_reader(&mut reader, &value_deserializer).unwrap());
            let _trie = Trie::<&str, String>::builder_with_storage(storage).build();
        }

        {
            let mut reader = create_input_stream();
            let value_deserializer = ValueDeserializer::new(|serialized| {
                static STRING_DESERIALIZER: Lazy<StringDeserializer> =
                    Lazy::new(|| StringDeserializer::new(false));
                STRING_DESERIALIZER.deserialize(serialized)
            });
            let storage =
                Box::new(MemoryStorage::new_with_reader(&mut reader, &value_deserializer).unwrap());
            let _trie = Trie::<&str, String>::builder_with_storage(storage)
                .key_serializer(StrSerializer::new(true))
                .build();
        }
    }

    #[test]
    fn is_empy() {
        {
            let trie = Trie::<&str, String>::builder().build().unwrap();

            assert!(trie.is_empty().unwrap());
        }
        {
            let trie = Trie::<&str, String>::builder()
                .elements([(KUMAMOTO, KUMAMOTO.to_string())].to_vec())
                .build()
                .unwrap();

            assert!(!trie.is_empty().unwrap());
        }
        {
            let trie = Trie::<&str, String>::builder()
                .elements(
                    [
                        (KUMAMOTO, KUMAMOTO.to_string()),
                        (TAMANA, TAMANA.to_string()),
                    ]
                    .to_vec(),
                )
                .build()
                .unwrap();

            assert!(!trie.is_empty().unwrap());
        }
    }

    #[test]
    fn size() {
        {
            let trie = Trie::<&str, String>::builder().build().unwrap();

            assert_eq!(trie.size().unwrap(), 0);
        }
        {
            let trie = Trie::<&str, String>::builder()
                .elements([(KUMAMOTO, KUMAMOTO.to_string())].to_vec())
                .build()
                .unwrap();

            assert_eq!(trie.size().unwrap(), 1);
        }
        {
            let trie = Trie::<&str, String>::builder()
                .elements(
                    [
                        (KUMAMOTO, KUMAMOTO.to_string()),
                        (TAMANA, TAMANA.to_string()),
                    ]
                    .to_vec(),
                )
                .build()
                .unwrap();

            assert_eq!(trie.size().unwrap(), 2);
        }
    }

    #[test]
    fn contains() {
        {
            let trie = Trie::<&str, String>::builder().build().unwrap();

            assert!(!trie.contains(KUMAMOTO).unwrap());
        }
        {
            let trie = Trie::<&str, String>::builder()
                .elements(
                    [
                        (KUMAMOTO, KUMAMOTO.to_string()),
                        (TAMANA, TAMANA.to_string()),
                    ]
                    .to_vec(),
                )
                .build()
                .unwrap();

            assert!(trie.contains(KUMAMOTO).unwrap());
            assert!(trie.contains(TAMANA).unwrap());
            assert!(!trie.contains(UTO).unwrap());
        }
    }

    #[test]
    fn find() {
        {
            let trie = Trie::<&str, String>::builder().build().unwrap();

            let found = trie.find(KUMAMOTO).unwrap();
            assert!(found.is_none());
        }
        {
            let trie = Trie::<&str, String>::builder()
                .elements(
                    [
                        (KUMAMOTO, KUMAMOTO.to_string()),
                        (TAMANA, TAMANA.to_string()),
                    ]
                    .to_vec(),
                )
                .build()
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
            let trie = Trie::<&str, String>::builder().build().unwrap();

            let _iterator = trie.iter();
        }
        {
            let trie = Trie::<&str, String>::builder()
                .elements(
                    [
                        (KUMAMOTO, KUMAMOTO.to_string()),
                        (TAMANA, TAMANA.to_string()),
                    ]
                    .to_vec(),
                )
                .build()
                .unwrap();

            let _iterator = trie.iter();
        }
    }

    #[test]
    fn subtrie() {
        {
            let trie = Trie::<&str, String>::builder().build().unwrap();

            let subtrie = trie.subtrie(TAMA).unwrap();
            assert!(subtrie.is_none());
        }
        {
            let trie = Trie::<&str, String>::builder()
                .elements([(KUMAMOTO, KUMAMOTO.to_string())].to_vec())
                .build()
                .unwrap();

            let subtrie = trie.subtrie(TAMA).unwrap();
            assert!(subtrie.is_none());
        }
        {
            let trie = Trie::<&str, String>::builder()
                .elements(
                    [
                        (KUMAMOTO, KUMAMOTO.to_string()),
                        (TAMANA, TAMANA.to_string()),
                    ]
                    .to_vec(),
                )
                .build()
                .unwrap();

            let subtrie = trie.subtrie(TAMA).unwrap().unwrap();

            let mut iterator = subtrie.iter();
            assert_eq!(*iterator.next().unwrap(), TAMANA.to_string());
            assert!(iterator.next().is_none());
        }
        {
            let trie = Trie::<&str, String>::builder()
                .elements(
                    [
                        (KUMAMOTO, KUMAMOTO.to_string()),
                        (TAMANA, TAMANA.to_string()),
                        (TAMARAI, TAMARAI.to_string()),
                    ]
                    .to_vec(),
                )
                .build()
                .unwrap();

            let _mem = trie
                .double_array
                .storage()
                .as_any()
                .downcast_ref::<MemoryStorage<String>>()
                .unwrap();

            let subtrie = trie.subtrie(TAMA).unwrap().unwrap();

            let mut iterator = subtrie.iter();
            assert_eq!(*iterator.next().unwrap(), TAMANA.to_string());
            assert_eq!(*iterator.next().unwrap(), TAMARAI.to_string());
            assert!(iterator.next().is_none());
        }

        {
            let trie = Trie::<&str, i32>::builder().build().unwrap();

            let subtrie = trie.subtrie("Kuma").unwrap();
            assert!(subtrie.is_none());
        }
        {
            let trie = Trie::<&str, i32>::builder()
                .elements([("Kumamoto", 42)].to_vec())
                .build()
                .unwrap();

            let subtrie = trie.subtrie("Kuma").unwrap().unwrap();

            let mut iterator = subtrie.iter();
            assert_eq!(*iterator.next().unwrap(), 42);
            assert!(iterator.next().is_none());
        }

        {
            let trie = Trie::<&str, String>::builder().build().unwrap();

            let subtrie = trie.subtrie("Kuma").unwrap();
            assert!(subtrie.is_none());
        }
        {
            let trie = Trie::<&str, String>::builder()
                .elements(
                    [
                        ("Kumamoto", KUMAMOTO.to_string()),
                        ("Tamana", TAMANA.to_string()),
                    ]
                    .to_vec(),
                )
                .build()
                .unwrap();

            let subtrie = trie.subtrie("Kuma").unwrap().unwrap();

            let mut iterator = subtrie.iter();
            assert_eq!(*iterator.next().unwrap(), KUMAMOTO.to_string());
            assert!(iterator.next().is_none());
        }
    }

    #[test]
    fn storage() {
        {
            let trie = Trie::<&str, String>::builder()
                .elements([(KUMAMOTO, KUMAMOTO.to_string())].to_vec())
                .build()
                .unwrap();

            let _storage = trie.storage();
        }
        {
            let trie = Trie::<&str, i32>::builder()
                .elements([("Kumamoto", 42)].to_vec())
                .build()
                .unwrap();

            let _storage = trie.storage();
        }
        {
            let mut reader = create_input_stream();
            let value_deserializer = ValueDeserializer::new(|serialized| {
                static STRING_DESERIALIZER: Lazy<StringDeserializer> =
                    Lazy::new(|| StringDeserializer::new(false));
                STRING_DESERIALIZER.deserialize(serialized)
            });
            let storage =
                Box::new(MemoryStorage::new_with_reader(&mut reader, &value_deserializer).unwrap());
            let trie = Trie::<&str, String>::builder_with_storage(storage).build();

            let storage = trie.storage();

            let mut writer = Cursor::new(Vec::<u8>::new());
            let serializer = ValueSerializer::<String>::new(
                |value| {
                    static STR_SERIALIZER: Lazy<StrSerializer> =
                        Lazy::new(|| StrSerializer::new(false));
                    STR_SERIALIZER.serialize(&value.as_str())
                },
                0,
            );
            storage.serialize(&mut writer, &serializer).unwrap();
            let storage_serialized = writer.get_ref();

            assert_eq!(storage_serialized.as_slice(), SERIALIZED);
        }
    }
}
