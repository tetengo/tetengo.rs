/*!
 * A shared storage.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

use std::io::{Read, Write};
use std::rc::Rc;

use crate::memory_storage::MemoryStorage;
use crate::storage::Result;
use crate::storage::Storage;
use crate::value_serializer::{ValueDeserializer, ValueSerializer};

/**
 * A shared storage.
 *
 * # Type Parameters
 * * `T` - A value type.
 */
#[derive(Clone, Debug, Default)]
pub struct SharedStorage<T> {
    entity: Rc<MemoryStorage<T>>,
}

impl<T> SharedStorage<T> {
    /**
     * Creates a shared storage.
     */
    pub fn new() -> Self {
        let entity = MemoryStorage::<T>::new();
        Self {
            entity: Rc::new(entity),
        }
    }

    /**
     * Creates a shared storage.
     *
     * # Arguments
     * * `reader`             - A reader.
     * * `value_deserializer` - A deserializer for value objects.
     *
     * # Errors
     * * `std::io::Error`       - If fails to read.
     * * `DeserializationError` - If fails to deserialize.
     */
    pub fn from_reader(
        reader: &mut dyn Read,
        value_deserializer: &ValueDeserializer<T>,
    ) -> Result<Self> {
        let entity = MemoryStorage::<T>::from_reader(reader, value_deserializer)?;
        Ok(Self {
            entity: Rc::new(entity),
        })
    }
}

impl<T> Storage<T> for SharedStorage<T> {
    fn base_check_size(&self) -> usize {
        self.entity.base_check_size()
    }

    fn base_at(&self, base_check_index: usize) -> i32 {
        self.entity.base_at(base_check_index)
    }

    fn set_base_at(&mut self, base_check_index: usize, base: i32) {
        let Some(entity) = Rc::get_mut(&mut self.entity) else {
            panic!("Must not be called when shared.");
        };
        entity.set_base_at(base_check_index, base);
    }

    fn check_at(&self, base_check_index: usize) -> u8 {
        self.entity.check_at(base_check_index)
    }

    fn set_check_at(&mut self, base_check_index: usize, check: u8) {
        let Some(entity) = Rc::get_mut(&mut self.entity) else {
            panic!("Must not be called when shared.");
        };
        entity.set_check_at(base_check_index, check);
    }

    fn value_count(&self) -> usize {
        self.entity.value_count()
    }

    fn value_at(&self, value_index: usize) -> Option<&T> {
        self.entity.value_at(value_index)
    }

    fn add_value_at(&mut self, value_index: usize, value: T) {
        let Some(entity) = Rc::get_mut(&mut self.entity) else {
            panic!("Must not be called when shared.");
        };
        entity.add_value_at(value_index, value);
    }

    fn filling_rate(&self) -> f64 {
        self.entity.filling_rate()
    }

    fn serialize(
        &self,
        writer: &mut dyn Write,
        value_serializer: &ValueSerializer<T>,
    ) -> Result<()> {
        self.entity.serialize(writer, value_serializer)
    }
}

#[cfg(test)]
mod tests {
    use once_cell::sync::Lazy;
    use std::io::Cursor;

    use crate::serializer::{Deserializer, Serializer};
    use crate::string_serializer::{StringDeserializer, StringSerializer};

    use super::*;

    #[test]
    fn new() {
        let _storage = SharedStorage::<u32>::new();
    }

    #[rustfmt::skip]
    const SERIALIZED: [u8; 52] = [
        0x00u8, 0x00u8, 0x00u8, 0x02u8,
        0x00u8, 0x00u8, 0x2Au8, 0xFFu8,
        0x00u8, 0x00u8, 0xFEu8, 0x18u8,
        0x00u8, 0x00u8, 0x00u8, 0x05u8,
        0x00u8, 0x00u8, 0x00u8, 0x00u8,
        0x00u8, 0x00u8, 0x00u8, 0x00u8,
        0x00u8, 0x00u8, 0x00u8, 0x04u8,
        0x70u8, 0x69u8, 0x79u8, 0x6Fu8,
        0x00u8, 0x00u8, 0x00u8, 0x04u8,
        0x66u8, 0x75u8, 0x67u8, 0x61u8,
        0x00u8, 0x00u8, 0x00u8, 0x00u8,
        0x00u8, 0x00u8, 0x00u8, 0x04u8,
        0x68u8, 0x6Fu8, 0x67u8, 0x65u8,
    ];

    fn create_input_stream() -> Box<dyn Read> {
        Box::new(Cursor::new(SERIALIZED))
    }

    #[rustfmt::skip]
    const SERIALIZED_BROKEN: [u8; 9] = [
        0x00u8, 0x00u8, 0x00u8, 0x02u8,
        0x01u8, 0x23u8, 0x45u8, 0x67u8,
        0x89u8,
    ];

    fn create_input_stream_broken() -> Box<dyn Read> {
        Box::new(Cursor::new(SERIALIZED_BROKEN))
    }

    const BASE_CHECK_ARRAY: &[u32] = &[0x00002AFFu32, 0x0000FE18u32];

    fn base_check_array_of<T>(storage: &dyn Storage<T>) -> Vec<u32> {
        let size = storage.base_check_size();
        let mut array = Vec::<u32>::with_capacity(size);
        for i in 0..size {
            array.push(((storage.base_at(i) as u32) << 8u32) | storage.check_at(i) as u32);
        }
        array
    }

    #[test]
    fn from_reader() {
        {
            let mut reader = create_input_stream();
            let deserializer = ValueDeserializer::<String>::new(|serialized| {
                static STRING_DESERIALIZER: Lazy<StringDeserializer> =
                    Lazy::new(|| StringDeserializer::new());
                STRING_DESERIALIZER.deserialize(serialized)
            });
            let Ok(storage) = SharedStorage::from_reader(&mut reader, &deserializer) else {
                panic!();
            };

            assert_eq!(base_check_array_of(&storage), BASE_CHECK_ARRAY);
            {
                let Some(value) = storage.value_at(4) else {
                    panic!();
                };
                assert_eq!(value, "hoge");
            }
            {
                let Some(value) = storage.value_at(2) else {
                    panic!();
                };
                assert_eq!(value, "fuga");
            }
            {
                let Some(value) = storage.value_at(1) else {
                    panic!();
                };
                assert_eq!(value, "piyo");
            }
        }
        {
            let mut reader = create_input_stream_broken();
            let deserializer = ValueDeserializer::<String>::new(|serialized| {
                static STRING_DESERIALIZER: Lazy<StringDeserializer> =
                    Lazy::new(|| StringDeserializer::new());
                STRING_DESERIALIZER.deserialize(serialized)
            });
            let result = SharedStorage::from_reader(&mut reader, &deserializer);
            assert!(result.is_err());
        }
    }

    #[test]
    fn base_check_size() {
        {
            let storage = SharedStorage::<u32>::new();
            assert!(storage.base_check_size() >= 1);
        }
        {
            let storage = SharedStorage::<u32>::new();
            let _ = storage.base_at(42);
            assert!(storage.base_check_size() >= 43);
        }
    }

    #[test]
    fn base_at() {
        let storage = SharedStorage::<u32>::new();

        assert_eq!(storage.base_at(42), 0);
    }

    #[test]
    fn set_base_at() {
        let mut storage = SharedStorage::<u32>::new();

        storage.set_base_at(42, 4242);

        assert_eq!(storage.base_at(42), 4242);
    }

    #[test]
    fn check_at() {
        let storage = SharedStorage::<u32>::new();

        assert_eq!(
            storage.check_at(42),
            0xFF, /* TODO: double_array::vacant_check_value() */
        );
    }

    #[test]
    fn set_check_at() {
        let mut storage = SharedStorage::<u32>::new();

        storage.set_check_at(24, 124);

        assert_eq!(storage.check_at(24), 124);
    }

    #[test]
    fn value_count() {
        let mut storage = SharedStorage::<String>::new();
        assert_eq!(storage.value_count(), 0);

        storage.add_value_at(24, String::from("hoge"));
        assert_eq!(storage.value_count(), 25);

        storage.add_value_at(42, String::from("fuga"));
        assert_eq!(storage.value_count(), 43);

        storage.add_value_at(0, String::from("piyo"));
        assert_eq!(storage.value_count(), 43);
    }

    #[test]
    fn value_at() {
        let storage = SharedStorage::<u32>::new();

        assert!(storage.value_at(42).is_none());
    }

    #[test]
    fn add_value_at() {
        let mut storage = SharedStorage::<String>::new();

        storage.add_value_at(24, String::from("hoge"));

        assert!(storage.value_at(0).is_none());
        {
            let Some(value) = storage.value_at(24) else {
                panic!();
            };
            assert_eq!(value, "hoge");
        }
        assert!(storage.value_at(42).is_none());

        storage.add_value_at(42, String::from("fuga"));

        {
            let Some(value) = storage.value_at(42)  else {
                panic!();
            };
            assert_eq!(value, "fuga");
        }
        assert!(storage.value_at(4242).is_none());

        storage.add_value_at(0, String::from("piyo"));

        {
            let Some(value) = storage.value_at(0) else {
                panic!();
            };
            assert_eq!(value, "piyo");
        }
        {
            let Some(value) = storage.value_at(42) else {
                panic!();
            };
            assert_eq!(value, "fuga");
        }
    }

    #[test]
    fn filling_rate() {
        let mut storage = SharedStorage::<u32>::new();

        for i in 0..9 {
            if i % 3 == 0 {
                storage.set_base_at(i, (i * i) as i32);
                storage.set_check_at(i, i as u8);
            } else {
                storage.set_base_at(i, storage.base_at(i));
                storage.set_check_at(i, storage.check_at(i));
            }
        }

        assert!((storage.filling_rate() - 3.0 / 9.0).abs() < 0.1);
    }

    #[test]
    fn serialize() {
        let mut storage = SharedStorage::<String>::new();

        storage.set_base_at(0, 42);
        storage.set_base_at(1, 0xFE);
        storage.set_check_at(1, 24);

        storage.add_value_at(4, String::from("hoge"));
        storage.add_value_at(2, String::from("fuga"));
        storage.add_value_at(1, String::from("piyo"));

        let mut writer = Cursor::new(Vec::<u8>::new());
        let serializer = ValueSerializer::<String>::new(
            |value| {
                static STRING_SERIALIZER: Lazy<StringSerializer> =
                    Lazy::new(|| StringSerializer::new());
                STRING_SERIALIZER.serialize(value)
            },
            0,
        );
        let result = storage.serialize(&mut writer, &serializer);
        assert!(result.is_ok());

        #[rustfmt::skip]
        const EXPECTED: [u8; 52] = [
            0x00u8, 0x00u8, 0x00u8, 0x02u8,
            0x00u8, 0x00u8, 0x2Au8, 0xFFu8,
            0x00u8, 0x00u8, 0xFEu8, 0x18u8,
            0x00u8, 0x00u8, 0x00u8, 0x05u8,
            0x00u8, 0x00u8, 0x00u8, 0x00u8,
            0x00u8, 0x00u8, 0x00u8, 0x00u8,
            0x00u8, 0x00u8, 0x00u8, 0x04u8,
            0x70u8, 0x69u8, 0x79u8, 0x6Fu8,
            0x00u8, 0x00u8, 0x00u8, 0x04u8,
            0x66u8, 0x75u8, 0x67u8, 0x61u8,
            0x00u8, 0x00u8, 0x00u8, 0x00u8,
            0x00u8, 0x00u8, 0x00u8, 0x04u8,
            0x68u8, 0x6Fu8, 0x67u8, 0x65u8,
        ];
        let serialized = writer.get_ref();
        assert_eq!(serialized, &EXPECTED);
    }

    impl<T> SharedStorage<T> {
        fn shared_with(&self, another: &SharedStorage<T>) -> bool {
            Rc::ptr_eq(&self.entity, &another.entity)
        }
    }

    #[test]
    fn clone() {
        let mut storage = SharedStorage::<u32>::new();

        storage.set_base_at(0, 42);
        storage.set_base_at(1, 0xFE);
        storage.set_check_at(1, 24);

        let /* mut */ clone = storage.clone();

        assert!(clone.shared_with(&storage));

        let base_check_array = base_check_array_of(&clone);

        const EXPECTED: [u32; 2] = [0x00002AFFu32, 0x0000FE18u32];
        assert_eq!(base_check_array, &EXPECTED);

        // Rust forbids to modify the object shared with others.
        // clone.set_base_at(0, 2424);
        // clone.set_check_at(5, 42);

        // assert_eq!(clone.base_at(0), 2424);
        // assert_eq!(clone.check_at(5), 42);

        // assert_eq!(storage.base_at(0), 2424);
        // assert_eq!(storage.check_at(5), 42);
    }
}
