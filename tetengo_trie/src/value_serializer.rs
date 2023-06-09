/*!
 * A value serializer.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

use std::fmt;

use crate::serializer::Result;

/**
 * A value serializer.
 */
#[derive(Clone, Copy)]
pub struct ValueSerializer<T: ?Sized> {
    serialize: fn(value: &T) -> Vec<u8>,

    fixed_value_size: usize,
}

impl<T: ?Sized> fmt::Debug for ValueSerializer<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ValueSerializer")
            .field("serialize", &"<fn>")
            .field("fixed_value_size", &self.fixed_value_size)
            .finish()
    }
}

impl<T: ?Sized> ValueSerializer<T> {
    /**
     * Creates a value serializer.
     *
     * # Arguments
     * * `serialize`        - A serializing function.
     * * `fixed_value_size` - The value size if it is fixed. Or 0 if the size is variable.
     */
    pub fn new(serialize: fn(value: &T) -> Vec<u8>, fixed_value_size: usize) -> Self {
        Self {
            serialize,
            fixed_value_size,
        }
    }

    /**
     * Serializes a value.
     *
     * # Arguments
     * * `value` - A value.
     *
     * # Returns
     * The serialized value.
     */
    pub fn serialize(&self, value: &T) -> Vec<u8> {
        (self.serialize)(value)
    }

    /**
     * Returns the fixed value size.
     *
     * # Returns
     * The fixed value size.
     */
    pub fn fixed_value_size(&self) -> usize {
        self.fixed_value_size
    }
}

/**
 * A value deserializer.
 */
#[derive(Clone, Copy)]
pub struct ValueDeserializer<T> {
    deserialize: fn(serialized: &[u8]) -> Result<T>,
}

impl<T> fmt::Debug for ValueDeserializer<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ValueDeserializer")
            .field("deserialize", &"<fn>")
            .finish()
    }
}

impl<T> ValueDeserializer<T> {
    /**
     * Creates a value deserializer.
     *
     * # Arguments
     * * `deserialize` - A deserializing function.
     */
    pub fn new(deserialize: fn(serialized: &[u8]) -> Result<T>) -> Self {
        Self { deserialize }
    }

    /**
     * Deserializes a value.
     *
     * # Arguments
     * * `serialized` - A serialized value.
     *
     * # Returns
     * A value.
     *
     * # Errors
     * * `DeserializationError` - Failed to deserialize.
     */
    pub fn deserialize(&self, serialized: &[u8]) -> Result<T> {
        (self.deserialize)(serialized)
    }
}

#[cfg(test)]
mod tests {
    mod value_serializer {
        use std::mem::size_of;

        use crate::{integer_serializer::IntegerSerializer, serializer::Serializer};

        use super::super::*;

        #[test]
        fn new() {
            {
                let _ = ValueSerializer::new(
                    |value: &i32| return IntegerSerializer::new(false).serialize(value),
                    size_of::<i32>(),
                );
            }
            {
                let _ = ValueSerializer::new(|_: &str| return vec![3, 1, 4], 0);
            }
        }

        #[test]
        fn serialize() {
            {
                let serializer = ValueSerializer::new(
                    |value: &i32| return IntegerSerializer::new(false).serialize(value),
                    size_of::<i32>(),
                );

                let expected = IntegerSerializer::new(false).serialize(&42);
                let serialized = serializer.serialize(&42);
                assert_eq!(serialized, expected);
            }
            {
                let serializer = ValueSerializer::new(|_: &str| return vec![3, 1, 4], 0);

                let expected = vec![3, 1, 4];
                let serialized = serializer.serialize("hoge");
                assert_eq!(serialized, expected);
            }
        }

        #[test]
        fn fixed_value_size() {
            {
                let serializer = ValueSerializer::new(
                    |value: &i32| return IntegerSerializer::new(false).serialize(value),
                    size_of::<i32>(),
                );

                assert_eq!(serializer.fixed_value_size(), size_of::<i32>());
            }
            {
                let serializer = ValueSerializer::new(|_: &str| return vec![3, 1, 4], 0);

                assert_eq!(serializer.fixed_value_size(), 0);
            }
        }
    }

    mod value_deserializer {
        use crate::{
            integer_serializer::{IntegerDeserializer, IntegerSerializer},
            serializer::{Deserializer, Serializer},
        };

        use super::super::*;

        #[test]
        fn new() {
            {
                let _ = ValueDeserializer::new(|serialized: &[u8]| {
                    return IntegerDeserializer::<i32>::new(false).deserialize(serialized);
                });
            }
            {
                let _ = ValueDeserializer::new(|_: &[u8]| {
                    return Ok("hoge".to_string());
                });
            }
        }

        #[test]
        fn deserialize() {
            {
                let deserializer = ValueDeserializer::new(|serialized: &[u8]| {
                    return IntegerDeserializer::<i32>::new(false).deserialize(serialized);
                });

                let expected = 42;
                let serialized = IntegerSerializer::<i32>::new(false).serialize(&expected);
                let Ok(deserialized) = deserializer.deserialize(&serialized) else {
                    assert!(false);
                    panic!("Serialized must be successfully deserialized.")
                };
                assert_eq!(deserialized, expected);
            }
            {
                let deserializer = ValueDeserializer::new(|_: &[u8]| {
                    return Ok("hoge".to_string());
                });
                let expected = "hoge";
                let serialized = vec![3, 1, 4];
                let Ok(deserialized) = deserializer.deserialize(&serialized) else {
                    assert!(false);
                    panic!("Serialized must be successfully deserialized.")
                };
                assert_eq!(deserialized, expected);
            }
        }
    }
}
