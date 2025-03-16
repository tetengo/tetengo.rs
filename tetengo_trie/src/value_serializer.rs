/*!
 * A value serializer.
 *
 * Copyright (C) 2023-2025 kaoru  <https://www.tetengo.org/>
 */

use std::any::type_name_of_val;
use std::fmt::{self, Debug, Formatter};

use crate::error::Error;

/**
 * A serialize function type
 */
pub type Serialize<'a, Value> = Box<dyn FnMut(&Value) -> Vec<u8> + 'a>;

/**
 * A value serializer.
 *
 * # Type Parameters
 * * `Value` - A value type.
 */
pub struct ValueSerializer<'a, Value: ?Sized> {
    serialize: Serialize<'a, Value>,
    fixed_value_size: usize,
}

impl<'a, Value: ?Sized> ValueSerializer<'a, Value> {
    /**
     * Creates a value serializer.
     *
     * # Arguments
     * * `serialize`        - A serializing function.
     * * `fixed_value_size` - The value size if it is fixed. Or 0 if the size is variable.
     */
    pub fn new(serialize: Serialize<'a, Value>, fixed_value_size: usize) -> Self {
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
    pub fn serialize(&mut self, value: &Value) -> Vec<u8> {
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

impl<Value: ?Sized> Debug for ValueSerializer<'_, Value> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("ValueSerializer")
            .field("serialize", &type_name_of_val(&self.serialize))
            .field("fixed_value_size", &self.fixed_value_size)
            .finish()
    }
}

/**
 * A deserialize function type
 */
pub type Deserialize<Value> = Box<dyn FnMut(&[u8]) -> Result<Value, Error>>;

/**
 * A value deserializer.
 *
 * # Type Parameters
 * * `Value` - A value type.
 */
pub struct ValueDeserializer<Value: Clone> {
    deserialize: Deserialize<Value>,
}

impl<Value: Clone> ValueDeserializer<Value> {
    /**
     * Creates a value deserializer.
     *
     * # Arguments
     * * `deserialize` - A deserializing function.
     */
    pub fn new(deserialize: Deserialize<Value>) -> Self {
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
    pub fn deserialize(&mut self, serialized: &[u8]) -> Result<Value, Error> {
        (self.deserialize)(serialized)
    }
}

impl<Value: Clone> Debug for ValueDeserializer<Value> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("ValueDeserializer")
            .field("deserialize", &type_name_of_val(&self.deserialize))
            .finish()
    }
}

#[cfg(test)]
mod tests {
    mod value_serializer {
        use std::cell::RefCell;

        use crate::integer_serializer::IntegerSerializer;
        use crate::serializer::Serializer;

        use super::super::*;

        #[test]
        fn new() {
            {
                let _serializer = ValueSerializer::new(
                    Box::new(|value: &i32| IntegerSerializer::new(false).serialize(value)),
                    size_of::<i32>(),
                );
            }
            {
                let _serializer = ValueSerializer::new(Box::new(|_: &str| vec![3, 1, 4]), 0);
            }
        }

        #[test]
        fn serialize() {
            {
                let mut serializer = ValueSerializer::new(
                    Box::new(|value: &i32| IntegerSerializer::new(false).serialize(value)),
                    size_of::<i32>(),
                );

                let expected = IntegerSerializer::new(false).serialize(&42);
                let serialized = serializer.serialize(&42);
                assert_eq!(serialized, expected);
            }
            {
                let mut serializer = ValueSerializer::new(Box::new(|_: &str| vec![3, 1, 4]), 0);

                let expected = vec![3, 1, 4];
                let serialized = serializer.serialize("hoge");
                assert_eq!(serialized, expected);
            }
            {
                let modified_in_closure = RefCell::new(0);
                let mut serializer = ValueSerializer::new(
                    Box::new(|_: &str| {
                        *modified_in_closure.borrow_mut() = 42;
                        vec![4, 2]
                    }),
                    0,
                );

                let expected = vec![4, 2];
                let serialized = serializer.serialize("hoge");
                assert_eq!(serialized, expected);
                assert_eq!(*modified_in_closure.borrow(), 42);
            }
        }

        #[test]
        fn fixed_value_size() {
            {
                let serializer = ValueSerializer::new(
                    Box::new(|value: &i32| IntegerSerializer::new(false).serialize(value)),
                    size_of::<i32>(),
                );

                assert_eq!(serializer.fixed_value_size(), size_of::<i32>());
            }
            {
                let serializer = ValueSerializer::new(Box::new(|_: &str| vec![3, 1, 4]), 0);

                assert_eq!(serializer.fixed_value_size(), 0);
            }
        }
    }

    mod value_deserializer {
        use crate::integer_serializer::{IntegerDeserializer, IntegerSerializer};
        use crate::serializer::{Deserializer, Serializer};

        use super::super::*;

        #[test]
        fn new() {
            {
                let _deserializer = ValueDeserializer::new(Box::new(|serialized: &[u8]| {
                    IntegerDeserializer::<i32>::new(false)
                        .deserialize(serialized)
                        .map_err(Into::into)
                }));
            }
            {
                let _deserializer =
                    ValueDeserializer::new(Box::new(|_: &[u8]| Ok("hoge".to_string())));
            }
        }

        #[test]
        fn deserialize() {
            {
                let mut deserializer = ValueDeserializer::new(Box::new(|serialized: &[u8]| {
                    IntegerDeserializer::<i32>::new(false)
                        .deserialize(serialized)
                        .map_err(Into::into)
                }));

                let expected = 42;
                let serialized = IntegerSerializer::<i32>::new(false).serialize(&expected);
                let deserialized = deserializer.deserialize(&serialized).unwrap();
                assert_eq!(deserialized, expected);
            }
            {
                let mut deserializer =
                    ValueDeserializer::new(Box::new(|_: &[u8]| Ok("hoge".to_string())));
                let expected = "hoge";
                let serialized = vec![3, 1, 4];
                let deserialized = deserializer.deserialize(&serialized).unwrap();
                assert_eq!(deserialized, expected);
            }
        }
    }
}
