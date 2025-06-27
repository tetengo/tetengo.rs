/*!
 * A memory storage.
 *
 * Copyright (C) 2023-2025 kaoru  <https://www.tetengo.org/>
 */

use std::any::Any;
use std::cell::RefCell;
use std::fmt::Debug;
use std::io::{Read, Write};
use std::rc::Rc;
use std::sync::LazyLock;

use crate::double_array::VACANT_CHECK_VALUE;
use crate::error::Error;
use crate::integer_serializer::{IntegerDeserializer, IntegerSerializer};
use crate::serializer::{Deserializer, Serializer};
use crate::storage::Storage;
use crate::value_serializer::{ValueDeserializer, ValueSerializer};

type ValueArrayElement<Value> = Option<Rc<Value>>;

/**
 * A memory storage.
 *
 * # Type Parameters
 * * `Value` - A value type.
 */
#[derive(Debug, Default)]
pub struct MemoryStorage<Value: Clone> {
    base_check_array: RefCell<Vec<u32>>,
    value_array: Vec<ValueArrayElement<Value>>,
}

impl<Value: Clone + 'static> MemoryStorage<Value> {
    /**
     * Creates a memory storage.
     */
    pub fn new() -> Self {
        Self {
            base_check_array: RefCell::new(vec![u32::from(VACANT_CHECK_VALUE)]),
            value_array: Vec::new(),
        }
    }

    /**
     * Creates a memory storage.
     *
     * # Arguments
     * * `reader`             - A reader.
     * * `value_deserializer` - A deserializer for value objects.
     *
     * # Errors
     * * When it fails to read the memory.
     */
    pub fn new_with_reader(
        reader: &mut dyn Read,
        value_deserializer: &mut ValueDeserializer<Value>,
    ) -> Result<Self, Error> {
        let (base_check_array, value_array) = Self::deserialize(reader, value_deserializer)?;
        Ok(Self {
            base_check_array: RefCell::new(base_check_array),
            value_array,
        })
    }

    fn serialize_base_check_array(
        writer: &mut dyn Write,
        base_check_array: &[u32],
    ) -> Result<(), Error> {
        debug_assert!(base_check_array.len() < u32::MAX as usize);
        Self::write_u32(
            writer,
            u32::try_from(base_check_array.len()).map_err(|e| Error::InternalError(e.into()))?,
        )?;
        for v in base_check_array {
            Self::write_u32(writer, *v)?;
        }
        Ok(())
    }

    fn serialize_value_array(
        writer: &mut dyn Write,
        value_serializer: &mut ValueSerializer<'_, Value>,
        value_array: &[ValueArrayElement<Value>],
    ) -> Result<(), Error> {
        debug_assert!(value_array.len() < u32::MAX as usize);
        Self::write_u32(
            writer,
            u32::try_from(value_array.len()).map_err(|e| Error::InternalError(e.into()))?,
        )?;

        debug_assert!(value_serializer.fixed_value_size() < u32::MAX as usize);
        let fixed_value_size = u32::try_from(value_serializer.fixed_value_size())
            .expect("Fixed value size should fit in u32");
        Self::write_u32(writer, fixed_value_size)?;

        if fixed_value_size == 0 {
            for v in value_array {
                if let Some(v) = v {
                    let serialized = value_serializer.serialize(v);
                    debug_assert!(serialized.len() < u32::MAX as usize);
                    Self::write_u32(
                        writer,
                        u32::try_from(serialized.len())
                            .expect("Serialized length should fit in u32"),
                    )?;
                    writer
                        .write_all(&serialized)
                        .map_err(|e| Error::InternalError(e.into()))?;
                } else {
                    Self::write_u32(writer, 0)?;
                }
            }
        } else {
            for v in value_array {
                if let Some(v) = v {
                    let serialized = value_serializer.serialize(v);
                    debug_assert!(serialized.len() == fixed_value_size as usize);
                    writer
                        .write_all(&serialized)
                        .map_err(|e| Error::InternalError(e.into()))?;
                } else {
                    let uninitialized = vec![Self::UNINITIALIZED_BYTE; fixed_value_size as usize];
                    writer
                        .write_all(&uninitialized)
                        .map_err(|e| Error::InternalError(e.into()))?;
                }
            }
        }
        Ok(())
    }

    fn write_u32(writer: &mut dyn Write, value: u32) -> Result<(), Error> {
        static INTEGER_SERIALIZER: LazyLock<IntegerSerializer<u32>> =
            LazyLock::new(|| IntegerSerializer::new(false));

        let serialized = INTEGER_SERIALIZER.serialize(&value);
        writer
            .write_all(&serialized)
            .map_err(|e| Error::InternalError(e.into()))?;
        Ok(())
    }

    fn deserialize(
        reader: &mut dyn Read,
        value_deserializer: &mut ValueDeserializer<Value>,
    ) -> Result<(Vec<u32>, Vec<ValueArrayElement<Value>>), Error> {
        let base_check_array = Self::deserialize_base_check_array(reader)?;
        let value_array = Self::deserialize_value_array(reader, value_deserializer)?;
        Ok((base_check_array, value_array))
    }

    fn deserialize_base_check_array(reader: &mut dyn Read) -> Result<Vec<u32>, Error> {
        let size = Self::read_u32(reader)? as usize;
        let mut base_check_array = Vec::with_capacity(size);
        for _ in 0..size {
            base_check_array.push(Self::read_u32(reader)?);
        }
        Ok(base_check_array)
    }

    fn deserialize_value_array(
        reader: &mut dyn Read,
        value_deserializer: &mut ValueDeserializer<Value>,
    ) -> Result<Vec<ValueArrayElement<Value>>, Error> {
        let size = Self::read_u32(reader)? as usize;

        let fixed_value_size = Self::read_u32(reader)? as usize;
        let mut value_array = Vec::with_capacity(size);
        if fixed_value_size == 0 {
            for _ in 0..size {
                let element_size = Self::read_u32(reader)? as usize;
                if element_size > 0 {
                    let mut to_deserialize = vec![0; element_size];
                    reader
                        .read_exact(&mut to_deserialize)
                        .map_err(|e| Error::InternalError(e.into()))?;
                    value_array.push(Some(Rc::new(
                        value_deserializer.deserialize(&to_deserialize)?,
                    )));
                } else {
                    value_array.push(None);
                }
            }
        } else {
            for _ in 0..size {
                let mut to_deserialize = vec![0; fixed_value_size];
                reader
                    .read_exact(&mut to_deserialize)
                    .map_err(|e| Error::InternalError(e.into()))?;
                if to_deserialize
                    .iter()
                    .all(|&e| e == Self::UNINITIALIZED_BYTE)
                {
                    value_array.push(None);
                } else {
                    value_array.push(Some(Rc::new(
                        value_deserializer.deserialize(&to_deserialize)?,
                    )));
                }
            }
        }
        Ok(value_array)
    }

    fn read_u32(reader: &mut dyn Read) -> Result<u32, Error> {
        static U32_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
            LazyLock::new(|| IntegerDeserializer::new(false));

        let mut to_deserialize: [u8; size_of::<u32>()] = [0u8; size_of::<u32>()];
        reader
            .read_exact(&mut to_deserialize)
            .map_err(|e| Error::InternalError(e.into()))?;
        U32_DESERIALIZER.deserialize(&to_deserialize)
    }

    const UNINITIALIZED_BYTE: u8 = 0xFF;

    fn ensure_base_check_size(&self, size: usize) {
        if size > self.base_check_array.borrow().len() {
            self.base_check_array
                .borrow_mut()
                .resize(size, u32::from(VACANT_CHECK_VALUE));
        }
    }
}

impl<Value: Clone + Debug + 'static> Storage<Value> for MemoryStorage<Value> {
    fn base_check_size(&self) -> Result<usize, Error> {
        Ok(self.base_check_array.borrow().len())
    }

    fn base_at(&self, base_check_index: usize) -> Result<i32, Error> {
        self.ensure_base_check_size(base_check_index + 1);
        #[allow(clippy::cast_possible_wrap)]
        let result = self.base_check_array.borrow()[base_check_index] as i32 >> 8i32;
        Ok(result)
    }

    fn set_base_at(&mut self, base_check_index: usize, base: i32) -> Result<(), Error> {
        self.ensure_base_check_size(base_check_index + 1);
        self.base_check_array.borrow_mut()[base_check_index] &= 0x000000FF;
        #[allow(clippy::cast_sign_loss)]
        {
            self.base_check_array.borrow_mut()[base_check_index] |= (base as u32) << 8;
        }
        Ok(())
    }

    fn check_at(&self, base_check_index: usize) -> Result<u8, Error> {
        self.ensure_base_check_size(base_check_index + 1);
        Ok((self.base_check_array.borrow()[base_check_index] & 0xFF) as u8)
    }

    fn set_check_at(&mut self, base_check_index: usize, check: u8) -> Result<(), Error> {
        self.ensure_base_check_size(base_check_index + 1);
        self.base_check_array.borrow_mut()[base_check_index] &= 0xFFFFFF00;
        self.base_check_array.borrow_mut()[base_check_index] |= u32::from(check);
        Ok(())
    }

    fn value_count(&self) -> Result<usize, Error> {
        Ok(self.value_array.len())
    }

    fn value_at(&self, value_index: usize) -> Result<Option<Rc<Value>>, Error> {
        let Some(value) = self.value_array.get(value_index) else {
            return Ok(None);
        };
        Ok(value.clone())
    }

    fn add_value_at(&mut self, value_index: usize, value: Value) -> Result<(), Error> {
        if value_index >= self.value_array.len() {
            self.value_array.resize_with(value_index + 1, || None);
        }
        self.value_array[value_index] = Some(Rc::new(value));
        Ok(())
    }

    fn filling_rate(&self) -> Result<f64, Error> {
        let empty_count = self
            .base_check_array
            .borrow()
            .iter()
            .filter(|&&e| e == 0x000000FFu32)
            .count();
        let empty_count_f64 =
            f64::from(u32::try_from(empty_count).map_err(|e| Error::InternalError(e.into()))?);
        let total_count_f64 = f64::from(
            u32::try_from(self.base_check_array.borrow().len())
                .map_err(|e| Error::InternalError(e.into()))?,
        );
        Ok(1.0 - empty_count_f64 / total_count_f64)
    }

    fn serialize(
        &self,
        writer: &mut dyn Write,
        value_serializer: &mut ValueSerializer<'_, Value>,
    ) -> Result<(), Error> {
        Self::serialize_base_check_array(writer, &self.base_check_array.borrow())?;
        Self::serialize_value_array(writer, value_serializer, &self.value_array)?;

        Ok(())
    }
    fn clone_box(&self) -> Box<dyn Storage<Value>> {
        Box::new(Self {
            base_check_array: RefCell::new(self.base_check_array.borrow().clone()),
            value_array: self.value_array.clone(),
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
    use std::io::Cursor;

    use crate::string_serializer::{StrSerializer, StringDeserializer};

    use super::*;

    #[test]
    fn new() {
        let _storage = MemoryStorage::<i32>::new();
    }

    #[rustfmt::skip]
    const SERIALIZED: &[u8] = &[
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
    const SERIALIZED_FIXED_VALUE_SIZE: &[u8] = &[
        0x00u8, 0x00u8, 0x00u8, 0x02u8,
        0x00u8, 0x00u8, 0x2Au8, 0xFFu8,
        0x00u8, 0x00u8, 0xFEu8, 0x18u8,
        0x00u8, 0x00u8, 0x00u8, 0x05u8,
        0x00u8, 0x00u8, 0x00u8, 0x04u8,
        0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
        0x00u8, 0x00u8, 0x00u8, 0x9Fu8,
        0x00u8, 0x00u8, 0x00u8, 0x0Eu8,
        0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
        0x00u8, 0x00u8, 0x00u8, 0x03u8,
    ];

    fn create_input_stream_fixed_value_size() -> Box<dyn Read> {
        Box::new(Cursor::new(SERIALIZED_FIXED_VALUE_SIZE))
    }

    const BASE_CHECK_ARRAY: &[u32] = &[0x00002AFFu32, 0x0000FE18u32];

    fn base_check_array_of<Value: 'static>(storage: &dyn Storage<Value>) -> Vec<u32> {
        let size = storage.base_check_size().unwrap();
        let mut array = Vec::<u32>::with_capacity(size);
        for i in 0..size {
            #[allow(clippy::cast_sign_loss)]
            {
                array.push(
                    ((storage.base_at(i).unwrap() as u32) << 8u32)
                        | u32::from(storage.check_at(i).unwrap()),
                );
            }
        }
        array
    }

    #[rustfmt::skip]
    const SERIALIZED_BROKEN: &[u8] = &[
        0x00u8, 0x00u8, 0x00u8, 0x02u8,
        0x01u8, 0x23u8, 0x45u8, 0x67u8, 
        0x89u8,
    ];

    fn create_input_stream_broken() -> Box<dyn Read> {
        Box::new(Cursor::new(SERIALIZED_BROKEN))
    }

    #[test]
    fn new_with_reader() {
        {
            let mut reader = create_input_stream();
            let mut deserializer = ValueDeserializer::new(Box::new(|serialized| {
                static STRING_DESERIALIZER: LazyLock<StringDeserializer> =
                    LazyLock::new(|| StringDeserializer::new(false));
                STRING_DESERIALIZER.deserialize(serialized)
            }));
            let storage = MemoryStorage::new_with_reader(&mut reader, &mut deserializer).unwrap();

            assert_eq!(base_check_array_of(&storage), BASE_CHECK_ARRAY);
            assert_eq!(storage.value_at(4).unwrap().unwrap().as_ref(), "hoge");
            assert_eq!(storage.value_at(2).unwrap().unwrap().as_ref(), "fuga");
            assert_eq!(storage.value_at(1).unwrap().unwrap().as_ref(), "piyo");
        }
        {
            let mut reader = create_input_stream_fixed_value_size();
            let mut deserializer = ValueDeserializer::new(Box::new(|serialized| {
                static U32_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
                    LazyLock::new(|| IntegerDeserializer::<u32>::new(false));
                U32_DESERIALIZER.deserialize(serialized)
            }));
            let storage = MemoryStorage::new_with_reader(&mut reader, &mut deserializer).unwrap();

            assert_eq!(base_check_array_of(&storage), BASE_CHECK_ARRAY);
            assert_eq!(*storage.value_at(4).unwrap().unwrap(), 3);
            assert_eq!(*storage.value_at(2).unwrap().unwrap(), 14);
            assert_eq!(*storage.value_at(1).unwrap().unwrap(), 159);
        }
        {
            let mut reader = create_input_stream_broken();
            let mut deserializer = ValueDeserializer::new(Box::new(|serialized| {
                static STRING_DESERIALIZER: LazyLock<StringDeserializer> =
                    LazyLock::new(|| StringDeserializer::new(false));
                STRING_DESERIALIZER.deserialize(serialized)
            }));
            let result = MemoryStorage::new_with_reader(&mut reader, &mut deserializer);
            assert!(result.is_err());
        }
    }

    #[test]
    fn base_check_size() {
        {
            let storage = MemoryStorage::<u32>::new();
            assert!(storage.base_check_size().unwrap() >= 1);
        }
        {
            let storage = MemoryStorage::<u32>::new();
            let _ = storage.base_at(42).unwrap();
            assert!(storage.base_check_size().unwrap() >= 43);
        }
    }

    #[test]
    fn base_at() {
        let storage = MemoryStorage::<u32>::new();

        assert_eq!(storage.base_at(42).unwrap(), 0);
    }

    #[test]
    fn set_base_at() {
        let mut storage = MemoryStorage::<u32>::new();

        storage.set_base_at(42, 4242).unwrap();

        assert_eq!(storage.base_at(42).unwrap(), 4242);
    }

    #[test]
    fn check_at() {
        let storage = MemoryStorage::<u32>::new();

        assert_eq!(storage.check_at(42).unwrap(), VACANT_CHECK_VALUE);
    }

    #[test]
    fn set_check_at() {
        let mut storage = MemoryStorage::<u32>::new();

        storage.set_check_at(24, 124).unwrap();

        assert_eq!(storage.check_at(24).unwrap(), 124);
    }

    #[test]
    fn value_count() {
        let mut storage = MemoryStorage::<String>::new();
        assert_eq!(storage.value_count().unwrap(), 0);

        storage.add_value_at(24, "hoge".to_string()).unwrap();
        assert_eq!(storage.value_count().unwrap(), 25);

        storage.add_value_at(42, "fuga".to_string()).unwrap();
        assert_eq!(storage.value_count().unwrap(), 43);

        storage.add_value_at(0, "piyo".to_string()).unwrap();
        assert_eq!(storage.value_count().unwrap(), 43);
    }

    #[test]
    fn value_at() {
        let storage = MemoryStorage::<u32>::new();

        assert!(storage.value_at(42).unwrap().is_none());
    }

    #[test]
    fn add_value_at() {
        let mut storage = MemoryStorage::<String>::new();

        storage.add_value_at(24, String::from("hoge")).unwrap();

        assert!(storage.value_at(0).unwrap().is_none());
        assert_eq!(storage.value_at(24).unwrap().unwrap().as_ref(), "hoge");
        assert!(storage.value_at(42).unwrap().is_none());

        storage.add_value_at(42, String::from("fuga")).unwrap();

        assert_eq!(storage.value_at(42).unwrap().unwrap().as_ref(), "fuga");
        assert!(storage.value_at(4242).unwrap().is_none());

        storage.add_value_at(0, String::from("piyo")).unwrap();

        assert_eq!(storage.value_at(0).unwrap().unwrap().as_ref(), "piyo");
        assert_eq!(storage.value_at(42).unwrap().unwrap().as_ref(), "fuga");
    }

    #[test]
    fn filling_rate() {
        let mut storage = MemoryStorage::<u32>::new();

        for i in 0..9 {
            if i % 3 == 0 {
                storage
                    .set_base_at(i, i32::try_from(i * i).unwrap())
                    .unwrap();
                storage.set_check_at(i, u8::try_from(i).unwrap()).unwrap();
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
        {
            let mut storage = MemoryStorage::<String>::new();

            storage.set_base_at(0, 42).unwrap();
            storage.set_base_at(1, 0xFE).unwrap();
            storage.set_check_at(1, 24).unwrap();

            storage.add_value_at(4, String::from("hoge")).unwrap();
            storage.add_value_at(2, String::from("fuga")).unwrap();
            storage.add_value_at(1, String::from("piyo")).unwrap();

            let mut writer = Cursor::new(Vec::<u8>::new());
            let mut serializer = ValueSerializer::<String>::new(
                Box::new(|value: &String| {
                    static STR_SERIALIZER: LazyLock<StrSerializer> =
                        LazyLock::new(|| StrSerializer::new(false));
                    STR_SERIALIZER.serialize(&value.as_str())
                }),
                0,
            );
            let result = storage.serialize(&mut writer, &mut serializer);
            assert!(result.is_ok());

            #[rustfmt::skip]
            const EXPECTED: &[u8] = &[
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
            assert_eq!(serialized.as_slice(), EXPECTED);
        }
        {
            let mut storage = MemoryStorage::<u32>::new();

            storage.set_base_at(0, 42).unwrap();
            storage.set_base_at(1, 0xFE).unwrap();
            storage.set_check_at(1, 24).unwrap();

            storage.add_value_at(4, 3).unwrap();
            storage.add_value_at(2, 14).unwrap();
            storage.add_value_at(1, 159).unwrap();

            let mut writer = Cursor::new(Vec::<u8>::new());
            let mut serializer = ValueSerializer::<u32>::new(
                Box::new(|value| {
                    static INTEGER_SERIALIZER: LazyLock<IntegerSerializer<u32>> =
                        LazyLock::new(|| IntegerSerializer::new(false));
                    INTEGER_SERIALIZER.serialize(value)
                }),
                size_of::<u32>(),
            );
            let result = storage.serialize(&mut writer, &mut serializer);
            assert!(result.is_ok());

            #[rustfmt::skip]
            const EXPECTED: &[u8] = &[
                0x00u8, 0x00u8, 0x00u8, 0x02u8,
                0x00u8, 0x00u8, 0x2Au8, 0xFFu8,
                0x00u8, 0x00u8, 0xFEu8, 0x18u8,
                0x00u8, 0x00u8, 0x00u8, 0x05u8,
                0x00u8, 0x00u8, 0x00u8, 0x04u8,
                0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
                0x00u8, 0x00u8, 0x00u8, 0x9Fu8,
                0x00u8, 0x00u8, 0x00u8, 0x0Eu8,
                0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
                0x00u8, 0x00u8, 0x00u8, 0x03u8,
            ];
            let serialized = writer.get_ref();
            assert_eq!(serialized.as_slice(), EXPECTED);
        }
    }

    #[test]
    fn clone_box() {
        let mut storage = MemoryStorage::<u32>::new();

        storage.set_base_at(0, 42).unwrap();
        storage.set_base_at(1, 0xFE).unwrap();
        storage.set_check_at(1, 24).unwrap();

        let clone = storage.clone_box();

        let base_check_array = base_check_array_of(clone.as_ref());

        #[rustfmt::skip]
        const EXPECTED: &[u32] = &[
            0x00002AFFu32,
            0x0000FE18u32,
        ];
        assert_eq!(base_check_array, EXPECTED);
    }

    #[test]
    fn as_any() {
        let storage = MemoryStorage::<u32>::new();

        let _ = storage.as_any();
    }

    #[test]
    fn as_any_mut() {
        let mut storage = MemoryStorage::<u32>::new();

        let _ = storage.as_any_mut();
    }
}
