/*!
 * A memory storage.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

use once_cell::sync::Lazy;
use std::cell::RefCell;
use std::io::Read;
use std::mem::size_of;

use crate::integer_serializer::IntegerDeserializer;
use crate::serializer::Deserializer;
use crate::storage::{Result, Storage};
use crate::value_serializer::ValueDeserializer;

/**
 * A memory storage.
 *
 * # Type Parameters
 * * `T` - A value type.
 */
#[derive(Clone, Debug, Default)]
pub struct MemoryStorage<T> {
    base_check_array: RefCell<Vec<u32>>,
    value_array: Vec<Option<T>>,
}

impl<T> MemoryStorage<T> {
    /**
     * Creates a memory storage.
     */
    pub fn new() -> Self {
        Self {
            base_check_array: RefCell::new(vec![
                0xFF, /* TODO: 0x00000000 | tetengo::trie::double_array::key_terminator() */
            ]),
            value_array: Vec::new(),
        }
    }

    /**
     * Creates a memory storage.
     *
     * # Arguments
     * * `reader`             - A reader.
     * * `value_deserializer` - A deserializer for value objects.
     */
    pub fn from_reader(
        reader: &mut dyn Read,
        value_deserializer: &ValueDeserializer<T>,
    ) -> Result<Self> {
        let (base_check_array, value_array) = Self::deserialize(reader, value_deserializer)?;
        Ok(Self {
            base_check_array: RefCell::new(base_check_array),
            value_array,
        })
    }

    fn deserialize(
        reader: &mut dyn Read,
        value_deserializer: &ValueDeserializer<T>,
    ) -> Result<(Vec<u32>, Vec<Option<T>>)> {
        let base_check_array = Self::deserialize_base_check_array(reader)?;
        let value_array = Self::deserialize_value_array(reader, value_deserializer)?;
        Ok((base_check_array, value_array))
    }

    fn deserialize_base_check_array(reader: &mut dyn Read) -> Result<Vec<u32>> {
        let size = Self::read_u32(reader)? as usize;
        let mut base_check_array = Vec::with_capacity(size);
        for _ in 0..size {
            base_check_array.push(Self::read_u32(reader)?);
        }
        Ok(base_check_array)
    }

    fn deserialize_value_array(
        reader: &mut dyn Read,
        value_deserializer: &ValueDeserializer<T>,
    ) -> Result<Vec<Option<T>>> {
        let size = Self::read_u32(reader)? as usize;

        let fixed_value_size = Self::read_u32(reader)? as usize;
        let mut value_array = Vec::with_capacity(size);
        if fixed_value_size == 0 {
            for _ in 0..size {
                let element_size = Self::read_u32(reader)? as usize;
                if element_size > 0 {
                    let mut to_deserialize = vec![0; element_size];
                    reader.read_exact(&mut to_deserialize)?;
                    value_array.push(Some(value_deserializer.deserialize(&to_deserialize)?));
                } else {
                    value_array.push(None);
                }
            }
        } else {
            for _ in 0..size {
                let mut to_deserialize = vec![0; fixed_value_size];
                reader.read_exact(&mut to_deserialize)?;
                if to_deserialize
                    .iter()
                    .all(|&e| e == Self::UNINITIALIZED_BYTE)
                {
                    value_array.push(None);
                } else {
                    value_array.push(Some(value_deserializer.deserialize(&to_deserialize)?));
                }
            }
        }
        Ok(value_array)
    }
    //     else
    //     {
    //         for (auto i = static_cast<std::uint32_t>(0); i < size; ++i)
    //         {
    //             std::vector<char> to_deserialize(fixed_value_size, 0);
    //             input_stream.read(std::data(to_deserialize), fixed_value_size);
    //             if (input_stream.gcount() < static_cast<std::streamsize>(fixed_value_size))
    //             {
    //                 throw std::ios_base::failure("Can't read value.");
    //             }
    //             if (std::all_of(std::begin(to_deserialize), std::end(to_deserialize), [](const auto e) {
    //                     return e == uninitialized_byte();
    //                 }))
    //             {
    //                 std::remove_reference_t<decltype(value_array)>::value_type nullopt_{};
    //                 value_array.push_back(std::move(nullopt_));
    //             }
    //             else
    //             {
    //                 value_array.push_back(value_deserializer_(to_deserialize));
    //             }
    //         }
    //     }
    // }

    fn read_u32(reader: &mut dyn Read) -> Result<u32> {
        static U32_DESERIALIZER: Lazy<IntegerDeserializer<u32>> =
            Lazy::new(|| IntegerDeserializer::new(false));

        let mut to_deserialize: [u8; size_of::<u32>()] = [0u8; size_of::<u32>()];
        reader.read_exact(&mut to_deserialize)?;
        U32_DESERIALIZER.deserialize(&to_deserialize)
    }

    const UNINITIALIZED_BYTE: u8 = 0xFF;

    fn ensure_base_check_size(&self, size: usize) {
        if size > self.base_check_array.borrow().len() {
            self.base_check_array.borrow_mut().resize(
                size, 0xFF, /* TODO: 0x00000000U | double_array::vacant_check_value() */
            );
        }
    }
}

impl<T> Storage<T> for MemoryStorage<T> {
    fn base_check_size(&self) -> usize {
        self.base_check_array.borrow().len()
    }

    fn base_at(&self, base_check_index: usize) -> i32 {
        self.ensure_base_check_size(base_check_index + 1);
        (self.base_check_array.borrow()[base_check_index] >> 8u32) as i32
    }

    fn set_base_at(&mut self, base_check_index: usize, base: i32) {
        todo!()
    }

    fn check_at(&self, base_check_index: usize) -> u8 {
        self.ensure_base_check_size(base_check_index + 1);
        (self.base_check_array.borrow()[base_check_index] & 0xFF) as u8
    }

    fn set_check_at(&mut self, base_check_index: usize, check: u8) {
        todo!()
    }

    fn value_count(&self) -> usize {
        todo!()
    }

    fn value_at(&self, value_index: usize) -> Option<&T> {
        if value_index >= self.value_array.len() {
            None
        } else if let Some(value) = &self.value_array[value_index] {
            Some(value)
        } else {
            None
        }
    }

    fn add_value_at(&mut self, value_index: usize, value: T) {
        todo!()
    }

    fn filling_rate(&self) -> f64 {
        todo!()
    }

    fn serialize(
        &self,
        writer: &dyn std::io::Write,
        value_serializer: &crate::value_serializer::ValueSerializer<T>,
    ) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::{serializer::Deserializer, string_serializer::StringDeserializer};

    use super::*;

    #[test]
    fn new() {
        let _storage = MemoryStorage::<i32>::new();
    }

    #[rustfmt::skip]
    const SERIALIZED: &[u8; 52] = &[
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

    #[rustfmt::skip]
    const SERIALIZED_FIXED_VALUE_SIZE: &[u8; 40] = &[
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
            let mut reader = Cursor::new(SERIALIZED);
            let deserializer = ValueDeserializer::new(|serialized| {
                static STRING_DESERIALIZER: Lazy<StringDeserializer> =
                    Lazy::new(|| StringDeserializer::new());
                STRING_DESERIALIZER.deserialize(serialized)
            });
            let Ok(storage) = MemoryStorage::from_reader(&mut reader, &deserializer) else {
                assert!(false);
                panic!();
            };

            assert_eq!(base_check_array_of(&storage), BASE_CHECK_ARRAY);
            if let Some(value) = storage.value_at(4) {
                assert_eq!(value, "hoge");
            } else {
                assert!(false);
                panic!();
            }
            if let Some(value) = storage.value_at(2) {
                assert_eq!(value, "fuga");
            } else {
                assert!(false);
                panic!();
            }
            if let Some(value) = storage.value_at(1) {
                assert_eq!(value, "piyo");
            } else {
                assert!(false);
                panic!();
            }
        }
        {
            let mut reader = Cursor::new(SERIALIZED_FIXED_VALUE_SIZE);
            let deserializer = ValueDeserializer::new(|serialized| {
                static U32_DESERIALIZER: Lazy<IntegerDeserializer<u32>> =
                    Lazy::new(|| IntegerDeserializer::<u32>::new(false));
                U32_DESERIALIZER.deserialize(serialized)
            });
            let Ok(storage) = MemoryStorage::from_reader(&mut reader, &deserializer) else {
                assert!(false);
                panic!();
            };

            assert_eq!(base_check_array_of(&storage), BASE_CHECK_ARRAY);
            if let Some(value) = storage.value_at(4) {
                assert_eq!(*value, 3u32);
            } else {
                assert!(false);
                panic!();
            }
            if let Some(value) = storage.value_at(2) {
                assert_eq!(*value, 14u32);
            } else {
                assert!(false);
                panic!();
            }
            if let Some(value) = storage.value_at(1) {
                assert_eq!(*value, 159u32);
            } else {
                assert!(false);
                panic!();
            }
        }
    }
    // {
    //     const auto p_input_stream = create_broken_input_stream();

    //     const tetengo::trie::value_deserializer deserializer{ [](const std::vector<char>& serialized) {
    //         static const tetengo::trie::default_deserializer<std::string>string_deserializer{ false };
    //         return string_deserializer(std::string{ std::begin(serialized), std::end(serialized) });
    //     } };
    //     BOOST_CHECK_THROW(
    //         const tetengo::trie::memory_storage storage_(*p_input_stream, deserializer), std::ios_base::failure);
    // }
}
