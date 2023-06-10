/*!
 * A memory storage.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

use std::io::Read;

use crate::storage::Result;
use crate::value_serializer::ValueDeserializer;

/**
 * A memory storage.
 *
 * # Type Parameters
 * * `T` - A value type.
 */
#[derive(Clone, Debug, Default)]
pub struct MemoryStorage<T> {
    _base_check_array: Vec<u32>,
    _value_array: Vec<Option<T>>,
}

impl<T> MemoryStorage<T> {
    /**
     * Creates a memory storage.
     */
    pub fn new() -> Self {
        Self {
            _base_check_array: vec![
                0xFF, /* TODO: 0x00000000 | tetengo::trie::double_array::key_terminator() */
            ],
            _value_array: Vec::new(),
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
        _reader: &dyn Read,
        _value_deserializer: &ValueDeserializer<T>,
    ) -> Result<Self> {
        let base_check_array = Vec::<u32>::new();
        let value_array = Vec::<Option<T>>::new();
        //     deserialize(input_stream, value_deserializer_, m_base_check_array, m_value_array);
        Ok(Self {
            _base_check_array: base_check_array,
            _value_array: value_array,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let _storage = MemoryStorage::<i32>::new();
    }

    #[test]
    fn from_reader() {
        // let _storage = MemoryStorage::<i32>::new();
    }
}
