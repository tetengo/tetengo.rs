/*!
 * A trie.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

use crate::double_array::{DoubleArray, DEFAULT_DENSITY_FACTOR};
use crate::serializer::{Serializer, SerializerOf};

/**
 * A result type.
 *
 * # Type Parameters
 * * `T` - A type.
 */
pub type Result<T> = anyhow::Result<T>;

/// The default double array density factor.
const _DEFAULT_DOUBLE_ARRAY_DENSITY_FACTOR: usize = DEFAULT_DENSITY_FACTOR;

/**
 * A trie.
 */
#[derive(Debug)]
pub struct Trie<Key: ?Sized, Value, KeySerializer: Serializer = <() as SerializerOf<Key>>::Type> {
    _phantom: std::marker::PhantomData<Key>,
    _double_array: DoubleArray<Value>,
    _key_serializer: KeySerializer,
}

impl<Key: ?Sized, Value: Clone + 'static, KeySerializer: Serializer>
    Trie<Key, Value, KeySerializer>
{
    /**
     * Creates a trie.
     */
    pub fn new() -> Result<Self> {
        Ok(Self {
            _phantom: std::marker::PhantomData,
            _double_array: DoubleArray::new()?,
            _key_serializer: KeySerializer::new(true),
        })
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
}

#[cfg(test)]
mod tests {
    use crate::string_serializer::StringSerializer;

    use super::*;

    #[test]
    fn test_new() {
        let _trie = Trie::<str, i32>::new().unwrap();
    }

    #[test]
    fn new_with_keyserializer() {
        let key_serializer = StringSerializer::new(true);
        let _trie = Trie::<str, i32>::new_with_keyserializer(key_serializer).unwrap();
    }
}
