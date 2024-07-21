/*!
 * A value serializer.
 *
 * Copyright (C) 2023-2024 kaoru  <https://www.tetengo.org/>
 */

use anyhow::Result;
use std::fmt::{self, Debug, Formatter};

/**
 * A value serializer.
 *
 * # Type Parameters
 * * `Value` - A value type.
 */
#[derive(Clone, Copy)]
pub struct ValueSerializer<Value: ?Sized> {
    serialize: fn(value: &Value) -> Vec<u8>,
    fixed_value_size: usize,
}

impl<Value: ?Sized> ValueSerializer<Value> {
    /**
     * Creates a value serializer.
     *
     * # Arguments
     * * `serialize`        - A serializing function.
     * * `fixed_value_size` - The value size if it is fixed. Or 0 if the size is variable.
     */
    pub const fn new(serialize: fn(value: &Value) -> Vec<u8>, fixed_value_size: usize) -> Self {
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
    pub fn serialize(&self, value: &Value) -> Vec<u8> {
        (self.serialize)(value)
    }

    /**
     * Returns the fixed value size.
     *
     * # Returns
     * The fixed value size.
     */
    pub const fn fixed_value_size(&self) -> usize {
        self.fixed_value_size
    }
}

impl<Value: ?Sized> Debug for ValueSerializer<Value> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("ValueSerializer")
            .field("serialize", &"<fn>")
            .field("fixed_value_size", &self.fixed_value_size)
            .finish()
    }
}

/**
 * A value deserializer.
 *
 * # Type Parameters
 * * `Value` - A value type.
 */
#[derive(Clone, Copy)]
pub struct ValueDeserializer<Value: Clone> {
    deserialize: fn(serialized: &[u8]) -> Result<Value>,
}

impl<Value: Clone> ValueDeserializer<Value> {
    /**
     * Creates a value deserializer.
     *
     * # Arguments
     * * `deserialize` - A deserializing function.
     */
    pub const fn new(deserialize: fn(serialized: &[u8]) -> Result<Value>) -> Self {
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
     * * When it fails to deserialize the value.
     */
    pub fn deserialize(&self, serialized: &[u8]) -> Result<Value> {
        (self.deserialize)(serialized)
    }
}

impl<Value: Clone> Debug for ValueDeserializer<Value> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("ValueDeserializer")
            .field("deserialize", &"<fn>")
            .finish()
    }
}

#[cfg(test)]
mod tests {
    mod value_serializer {
        use std::mem::size_of;

        use crate::integer_serializer::IntegerSerializer;
        use crate::serializer::Serializer;

        use super::super::*;

        #[test]
        const fn new() {
            {
                let _ = ValueSerializer::new(
                    |value: &i32| IntegerSerializer::new(false).serialize(value),
                    size_of::<i32>(),
                );
            }
            {
                let _ = ValueSerializer::new(|_: &str| vec![3, 1, 4], 0);
            }
        }

        #[test]
        fn serialize() {
            {
                let serializer = ValueSerializer::new(
                    |value: &i32| IntegerSerializer::new(false).serialize(value),
                    size_of::<i32>(),
                );

                let expected = IntegerSerializer::new(false).serialize(&42);
                let serialized = serializer.serialize(&42);
                assert_eq!(serialized, expected);
            }
            {
                let serializer = ValueSerializer::new(|_: &str| vec![3, 1, 4], 0);

                let expected = vec![3, 1, 4];
                let serialized = serializer.serialize("hoge");
                assert_eq!(serialized, expected);
            }
        }

        #[test]
        fn fixed_value_size() {
            {
                let serializer = ValueSerializer::new(
                    |value: &i32| IntegerSerializer::new(false).serialize(value),
                    size_of::<i32>(),
                );

                assert_eq!(serializer.fixed_value_size(), size_of::<i32>());
            }
            {
                let serializer = ValueSerializer::new(|_: &str| vec![3, 1, 4], 0);

                assert_eq!(serializer.fixed_value_size(), 0);
            }
        }
    }

    mod value_deserializer {
        use crate::integer_serializer::{IntegerDeserializer, IntegerSerializer};
        use crate::serializer::{Deserializer, Serializer};

        use super::super::*;

        #[test]
        const fn new() {
            {
                let _ = ValueDeserializer::new(|serialized: &[u8]| {
                    IntegerDeserializer::<i32>::new(false).deserialize(serialized)
                });
            }
            {
                let _ = ValueDeserializer::new(|_: &[u8]| Ok("hoge".to_string()));
            }
        }

        #[test]
        fn deserialize() {
            {
                let deserializer = ValueDeserializer::new(|serialized: &[u8]| {
                    IntegerDeserializer::<i32>::new(false).deserialize(serialized)
                });

                let expected = 42;
                let serialized = IntegerSerializer::<i32>::new(false).serialize(&expected);
                let deserialized = deserializer.deserialize(&serialized).unwrap();
                assert_eq!(deserialized, expected);
            }
            {
                let deserializer = ValueDeserializer::new(|_: &[u8]| Ok("hoge".to_string()));
                let expected = "hoge";
                let serialized = vec![3, 1, 4];
                let deserialized = deserializer.deserialize(&serialized).unwrap();
                assert_eq!(deserialized, expected);
            }
        }
    }
}
