/*!
 * A shared storage.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

use anyhow::Result;
use std::any::Any;
use std::io::{Read, Write};
use std::rc::Rc;

use crate::memory_storage::MemoryStorage;
use crate::storage::Storage;
use crate::value_serializer::{ValueDeserializer, ValueSerializer};

/**
 * A shared storage.
 *
 * # Type Parameters
 * * `Value` - A value type.
 */
#[derive(Debug, Default)]
pub struct SharedStorage<Value: Clone> {
    entity: Rc<MemoryStorage<Value>>,
}

impl<Value: Clone + 'static> SharedStorage<Value> {
    /**
     * Creates a shared storage.
     */
    pub fn new() -> Self {
        let entity = MemoryStorage::<Value>::new();
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
     * * When it fails to read the memory.
     */
    pub fn from_reader(
        reader: &mut dyn Read,
        value_deserializer: &ValueDeserializer<Value>,
    ) -> Result<Self> {
        let entity = MemoryStorage::<Value>::from_reader(reader, value_deserializer)?;
        Ok(Self {
            entity: Rc::new(entity),
        })
    }
}

impl<Value: Clone + 'static> Storage<Value> for SharedStorage<Value> {
    fn base_check_size(&self) -> Result<usize> {
        self.entity.base_check_size()
    }

    fn base_at(&self, base_check_index: usize) -> Result<i32> {
        self.entity.base_at(base_check_index)
    }

    fn set_base_at(&mut self, base_check_index: usize, base: i32) -> Result<()> {
        let entity = Rc::get_mut(&mut self.entity).expect("Must not be called when shared.");
        entity.set_base_at(base_check_index, base)
    }

    fn check_at(&self, base_check_index: usize) -> Result<u8> {
        self.entity.check_at(base_check_index)
    }

    fn set_check_at(&mut self, base_check_index: usize, check: u8) -> Result<()> {
        let entity = Rc::get_mut(&mut self.entity).expect("Must not be called when shared.");
        entity.set_check_at(base_check_index, check)
    }

    fn value_count(&self) -> Result<usize> {
        self.entity.value_count()
    }

    fn for_value_at(
        &self,
        value_index: usize,
        operation: &dyn Fn(&Option<Value>) -> Result<()>,
    ) -> Result<()> {
        self.entity.for_value_at(value_index, operation)
    }

    fn for_value_at_mut(
        &self,
        value_index: usize,
        operation: &mut dyn FnMut(&Option<Value>) -> Result<()>,
    ) -> Result<()> {
        self.entity.for_value_at_mut(value_index, operation)
    }

    fn add_value_at(&mut self, value_index: usize, value: Value) -> Result<()> {
        let entity = Rc::get_mut(&mut self.entity).expect("Must not be called when shared.");
        entity.add_value_at(value_index, value)
    }

    fn filling_rate(&self) -> Result<f64> {
        self.entity.filling_rate()
    }

    fn serialize(
        &self,
        writer: &mut dyn Write,
        value_serializer: &ValueSerializer<Value>,
    ) -> Result<()> {
        self.entity.serialize(writer, value_serializer)
    }

    fn clone_box(&self) -> Box<dyn Storage<Value>> {
        Box::new(Self {
            entity: self.entity.clone(),
        })
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use once_cell::sync::Lazy;
    use std::io::Cursor;

    use crate::double_array::VACANT_CHECK_VALUE;
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

    fn base_check_array_of<Value>(storage: &dyn Storage<Value>) -> Vec<u32> {
        let size = storage.base_check_size().unwrap();
        let mut array = Vec::<u32>::with_capacity(size);
        for i in 0..size {
            array.push(
                ((storage.base_at(i).unwrap() as u32) << 8u32)
                    | storage.check_at(i).unwrap() as u32,
            );
        }
        array
    }

    #[test]
    fn from_reader() {
        {
            let mut reader = create_input_stream();
            let deserializer = ValueDeserializer::<String>::new(|serialized| {
                static STRING_DESERIALIZER: Lazy<StringDeserializer> =
                    Lazy::new(|| StringDeserializer::new(false));
                STRING_DESERIALIZER.deserialize(serialized)
            });
            let storage = SharedStorage::from_reader(&mut reader, &deserializer).unwrap();

            assert_eq!(base_check_array_of(&storage), BASE_CHECK_ARRAY);
            storage
                .for_value_at(4, &|value| {
                    assert_eq!(value.as_ref().unwrap(), "hoge");
                    Ok(())
                })
                .unwrap();
            storage
                .for_value_at(2, &|value| {
                    assert_eq!(value.as_ref().unwrap(), "fuga");
                    Ok(())
                })
                .unwrap();
            storage
                .for_value_at(1, &|value| {
                    assert_eq!(value.as_ref().unwrap(), "piyo");
                    Ok(())
                })
                .unwrap();
        }
        {
            let mut reader = create_input_stream_broken();
            let deserializer = ValueDeserializer::<String>::new(|serialized| {
                static STRING_DESERIALIZER: Lazy<StringDeserializer> =
                    Lazy::new(|| StringDeserializer::new(false));
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
            assert!(storage.base_check_size().unwrap() >= 1);
        }
        {
            let storage = SharedStorage::<u32>::new();
            let _ = storage.base_at(42).unwrap();
            assert!(storage.base_check_size().unwrap() >= 43);
        }
    }

    #[test]
    fn base_at() {
        let storage = SharedStorage::<u32>::new();

        assert_eq!(storage.base_at(42).unwrap(), 0);
    }

    #[test]
    fn set_base_at() {
        let mut storage = SharedStorage::<u32>::new();

        storage.set_base_at(42, 4242).unwrap();

        assert_eq!(storage.base_at(42).unwrap(), 4242);
    }

    #[test]
    fn check_at() {
        let storage = SharedStorage::<u32>::new();

        assert_eq!(storage.check_at(42).unwrap(), VACANT_CHECK_VALUE);
    }

    #[test]
    fn set_check_at() {
        let mut storage = SharedStorage::<u32>::new();

        storage.set_check_at(24, 124).unwrap();

        assert_eq!(storage.check_at(24).unwrap(), 124);
    }

    #[test]
    fn value_count() {
        let mut storage = SharedStorage::<String>::new();
        assert_eq!(storage.value_count().unwrap(), 0);

        storage.add_value_at(24, String::from("hoge")).unwrap();
        assert_eq!(storage.value_count().unwrap(), 25);

        storage.add_value_at(42, String::from("fuga")).unwrap();
        assert_eq!(storage.value_count().unwrap(), 43);

        storage.add_value_at(0, String::from("piyo")).unwrap();
        assert_eq!(storage.value_count().unwrap(), 43);
    }

    #[test]
    fn for_value_at() {
        let storage = SharedStorage::<u32>::new();

        storage
            .for_value_at(42, &|value| {
                assert!(value.is_none());
                Ok(())
            })
            .unwrap();
    }

    #[test]
    fn for_value_at_mut() {
        let storage = SharedStorage::<u32>::new();

        storage
            .for_value_at_mut(42, &mut |value| {
                assert!(value.is_none());
                Ok(())
            })
            .unwrap();
    }

    #[test]
    fn add_value_at() {
        let mut storage = SharedStorage::<String>::new();

        storage.add_value_at(24, String::from("hoge")).unwrap();

        storage
            .for_value_at(0, &|value| {
                assert!(value.is_none());
                Ok(())
            })
            .unwrap();
        storage
            .for_value_at(24, &|value| {
                assert_eq!(value.as_ref().unwrap(), "hoge");
                Ok(())
            })
            .unwrap();
        storage
            .for_value_at(42, &|value| {
                assert!(value.is_none());
                Ok(())
            })
            .unwrap();

        storage.add_value_at(42, String::from("fuga")).unwrap();

        storage
            .for_value_at(42, &|value| {
                assert_eq!(value.as_ref().unwrap(), "fuga");
                Ok(())
            })
            .unwrap();
        storage
            .for_value_at(4242, &|value| {
                assert!(value.is_none());
                Ok(())
            })
            .unwrap();

        storage.add_value_at(0, String::from("piyo")).unwrap();

        storage
            .for_value_at(0, &|value| {
                assert_eq!(value.as_ref().unwrap(), "piyo");
                Ok(())
            })
            .unwrap();
        storage
            .for_value_at(42, &|value| {
                assert_eq!(value.as_ref().unwrap(), "fuga");
                Ok(())
            })
            .unwrap();
    }

    #[test]
    fn filling_rate() {
        let mut storage = SharedStorage::<u32>::new();

        for i in 0..9 {
            if i % 3 == 0 {
                storage.set_base_at(i, (i * i) as i32).unwrap();
                storage.set_check_at(i, i as u8).unwrap();
            } else {
                storage.set_base_at(i, storage.base_at(i).unwrap()).unwrap();
                storage
                    .set_check_at(i, storage.check_at(i).unwrap())
                    .unwrap();
            }
        }

        assert!((storage.filling_rate().unwrap() - 3.0 / 9.0).abs() < 0.1);
    }

    #[test]
    fn serialize() {
        let mut storage = SharedStorage::<String>::new();

        storage.set_base_at(0, 42).unwrap();
        storage.set_base_at(1, 0xFE).unwrap();
        storage.set_check_at(1, 24).unwrap();

        storage.add_value_at(4, String::from("hoge")).unwrap();
        storage.add_value_at(2, String::from("fuga")).unwrap();
        storage.add_value_at(1, String::from("piyo")).unwrap();

        let mut writer = Cursor::new(Vec::<u8>::new());
        let serializer = ValueSerializer::<String>::new(
            |value| {
                static STRING_SERIALIZER: Lazy<StringSerializer> =
                    Lazy::new(|| StringSerializer::new(false));
                STRING_SERIALIZER.serialize(&value.as_str())
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

    impl<Value: Clone> SharedStorage<Value> {
        fn shared_with(&self, another: &SharedStorage<Value>) -> bool {
            Rc::ptr_eq(&self.entity, &another.entity)
        }
    }

    #[test]
    fn clone_box() {
        let mut storage = SharedStorage::<u32>::new();

        storage.set_base_at(0, 42).unwrap();
        storage.set_base_at(1, 0xFE).unwrap();
        storage.set_check_at(1, 24).unwrap();

        let clone = storage.clone_box();

        let clone_as_shared_storage = clone.as_any().downcast_ref::<SharedStorage<u32>>().unwrap();
        assert!(clone_as_shared_storage.shared_with(&storage));

        let base_check_array = base_check_array_of(clone.as_ref());

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

    #[test]
    fn as_any() {
        let storage = SharedStorage::<u32>::new();

        let _ = storage.as_any();
    }

    #[test]
    fn as_any_mut() {
        let mut storage = SharedStorage::<u32>::new();

        let _ = storage.as_any_mut();
    }
}
