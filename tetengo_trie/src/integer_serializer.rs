/*!
 * An integer serializer/deserializer.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

use std::marker;
use std::mem;
use std::ops;

use crate::serializer::{DeserializationError, Deserializer, Result, Serializer};

/**
 * A trait for integers.
 *
 * # Type Parameters
 * * `Object` - An object type.
 */
pub trait Integer<Object>:
    Copy
    + ops::ShlAssign<u128>
    + ops::ShrAssign<u128>
    + ops::BitAnd<Object, Output = Object>
    + ops::BitOrAssign<Object>
    + From<u8>
    + Into<i128>
{
}

impl<T, U> Integer<U> for T where
    T: Copy
        + ops::ShlAssign<u128>
        + ops::ShrAssign<u128>
        + ops::BitAnd<U, Output = U>
        + ops::BitOrAssign<U>
        + From<u8>
        + Into<i128>
{
}

/**
 * An integer serializer.
 *
 * When the argument `fe_escape` of the constructor is true, binary bytes are
 * serialized as following:
 *
 * |original byte|serialized byte|
 * |-|-|
 * |0x00     |0xFE       (0b11111110)            |
 * |0x01-0xFC|0x01-0xFC  (0b00000001-0b11111100) |
 * |0xFD     |0xFD, 0xFD (0b11111101, 0b11111101)|
 * |0xFE     |0xFD, 0xFE (0b11111101, 0b11111110)|
 * |0xFF     |0xFF       (0b11111111)            |
 *
 * # Type Parameters
 * * `Object` - An object type.
 */
#[derive(Debug)]
pub struct IntegerSerializer<Object: Integer<Object>> {
    fe_escape: bool,
    _phantom: marker::PhantomData<Object>,
}

impl<Object: Integer<Object>> IntegerSerializer<Object> {
    /**
     * Creates an integer serializer.
     *
     * # Arguments
     * * `fe_escape` - Set true to escape 0xFE.
     */
    pub fn new(fe_escape: bool) -> Self {
        IntegerSerializer {
            fe_escape,
            _phantom: marker::PhantomData,
        }
    }
}

impl<Object: Integer<Object>> Serializer for IntegerSerializer<Object> {
    type Object = Object;

    fn serialize(&self, object: &Object) -> Vec<u8> {
        to_bytes(object, self.fe_escape)
    }
}

/**
 * An integer deserialization error.
 */
#[derive(Clone, Copy, Debug, thiserror::Error)]
pub enum IntegerDeserialationError {
    /**
     * Invalid serialized length.
     */
    #[error("invalid serialized length")]
    InvalidSerializedLength,

    /**
     * Invalid serialized content.
     */
    #[error("invalid serialized content")]
    InvalidSerializedContent,
}

impl DeserializationError for IntegerDeserialationError {}

/**
 * An integer deserializer.
 *
 * When the argument `fe_escape` of the constructor is true, binary bytes are
 * deserialized as following:
 *
 * |original byte|serialized byte|
 * |-|-|
 * |0x00     |0xFE       (0b11111110)            |
 * |0x01-0xFC|0x01-0xFC  (0b00000001-0b11111100) |
 * |0xFD     |0xFD, 0xFD (0b11111101, 0b11111101)|
 * |0xFE     |0xFD, 0xFE (0b11111101, 0b11111110)|
 * |0xFF     |0xFF       (0b11111111)            |
 *
 * # Type Parameters
 * * `Object` - An object type.
 */
#[derive(Debug)]
pub struct IntegerDeserializer<Object: Integer<Object>> {
    fe_escape: bool,
    _phantom: marker::PhantomData<Object>,
}

impl<Object: Integer<Object>> IntegerDeserializer<Object> {
    /**
     * Creates an integer deserializer.
     *
     * # Arguments
     * * `fe_escape` - Set true to escape 0xFE.
     */
    pub fn new(fe_escape: bool) -> Self {
        IntegerDeserializer {
            fe_escape,
            _phantom: marker::PhantomData,
        }
    }
}

impl<Object: Integer<Object>> Deserializer for IntegerDeserializer<Object> {
    type Object = Object;

    fn deserialize(&self, bytes: &[u8]) -> Result<Object> {
        from_bytes(bytes, self.fe_escape)
    }
}

fn to_bytes<Object: Integer<Object>>(object: &Object, fe_escape: bool) -> Vec<u8> {
    if fe_escape {
        to_bytes_with_escape(object)
    } else {
        to_bytes_without_escape(object)
    }
}

fn to_bytes_with_escape<Object: Integer<Object>>(object: &Object) -> Vec<u8> {
    to_bytes_without_escape(object)
        .into_iter()
        .flat_map(|b| {
            if b == 0x00u8 {
                vec![0xFEu8]
            } else if b == 0xFDu8 || b == 0xFEu8 {
                vec![0xFDu8, b]
            } else {
                vec![b]
            }
        })
        .collect()
}

fn to_bytes_without_escape<Object: Integer<Object>>(object: &Object) -> Vec<u8> {
    let mut bytes = vec![];
    bytes.reserve(mem::size_of::<Object>());
    let mut object = *object;
    for _ in 0..mem::size_of::<Object>() {
        let byte_object = object & Object::from(0xFFu8);
        let u128_object: i128 = byte_object.into();
        let u8_object = u128_object as u8;
        bytes.push(u8_object);
        object >>= 8;
    }
    bytes.reverse();
    bytes
}

fn from_bytes<Object: Integer<Object>>(serialized: &[u8], fe_escape: bool) -> Result<Object> {
    if fe_escape {
        from_bytes_with_escape(serialized)
    } else {
        from_bytes_without_escape(serialized)
    }
}

fn from_bytes_with_escape<Object: Integer<Object>>(serialized: &[u8]) -> Result<Object> {
    if serialized.len() < mem::size_of::<Object>()
        || 2 * mem::size_of::<Object>() < serialized.len()
    {
        return Err(IntegerDeserialationError::InvalidSerializedLength.into());
    }
    let mut object = Object::from(0);
    let mut serialized_iter = serialized.iter();
    while let Some(byte) = serialized_iter.next() {
        object <<= 8;
        if *byte == 0xFDu8 {
            if let Some(byte2) = serialized_iter.next() {
                if *byte2 == 0xFDu8 || *byte2 == 0xFEu8 {
                    object |= Object::from(*byte2);
                } else {
                    return Err(IntegerDeserialationError::InvalidSerializedContent.into());
                }
            } else {
                return Err(IntegerDeserialationError::InvalidSerializedContent.into());
            }
        } else if *byte == 0xFEu8 {
            object |= Object::from(0x00u8);
        } else {
            object |= Object::from(*byte);
        }
    }
    Ok(object)
}

fn from_bytes_without_escape<Object: Integer<Object>>(serialized: &[u8]) -> Result<Object> {
    if serialized.len() < mem::size_of::<Object>()
        || 2 * mem::size_of::<Object>() < serialized.len()
    {
        return Err(IntegerDeserialationError::InvalidSerializedLength.into());
    }
    let mut object = Object::from(0);
    for byte in serialized {
        object <<= 8;
        object |= Object::from(*byte);
    }
    Ok(object)
}

#[cfg(test)]
mod tests {
    use crate::double_array::KEY_TERMINATOR;

    use super::*;

    const fn nul_byte() -> u8 {
        0xFEu8
    }

    #[test]
    fn serialize() {
        {
            let serializer = IntegerSerializer::<i32>::new(false);

            let object = 0x001234AB;
            let expected_serialized = vec![0x00u8, 0x12u8, 0x34u8, 0xABu8];
            let serialized = serializer.serialize(&object);
            assert_eq!(serialized, expected_serialized);
        }
        {
            let serializer = IntegerSerializer::<i32>::new(true);

            let object = 0x001234AB;
            let expected_serialized = vec![nul_byte(), 0x12u8, 0x34u8, 0xABu8];
            let serialized = serializer.serialize(&object);
            assert_eq!(serialized, expected_serialized);
            assert!(!serialized.iter().any(|&b| b == KEY_TERMINATOR));
        }
        {
            let serializer = IntegerSerializer::<u32>::new(false);

            let object = 0xFCFDFEFF;
            let expected_serialized = vec![0xFCu8, 0xFDu8, 0xFEu8, 0xFFu8];
            let serialized = serializer.serialize(&object);
            assert_eq!(serialized, expected_serialized);
        }
        {
            let serializer = IntegerSerializer::<u32>::new(true);

            let object = 0xFCFDFEFF;
            let expected_serialized = vec![0xFCu8, 0xFDu8, 0xFDu8, 0xFDu8, 0xFEu8, 0xFFu8];
            let serialized = serializer.serialize(&object);
            assert_eq!(serialized, expected_serialized);
            assert!(!serialized.iter().any(|&b| b == KEY_TERMINATOR));
        }
    }

    #[test]
    fn deserialize() {
        {
            let deserializer = IntegerDeserializer::<i32>::new(false);

            let serialized = vec![0x00u8, 0x12u8, 0x34u8, 0xABu8];
            let expected_object = 0x001234AB;
            let object = deserializer.deserialize(&serialized).unwrap();
            assert_eq!(object, expected_object);
        }
        {
            let deserializer = IntegerDeserializer::<i32>::new(true);

            let serialized = vec![nul_byte(), 0x12u8, 0x34u8, 0xABu8];
            let expected_object = 0x001234AB;
            let object = deserializer.deserialize(&serialized).unwrap();
            assert_eq!(object, expected_object);
        }
        {
            let deserializer = IntegerDeserializer::<u32>::new(false);

            let serialized = vec![0xFCu8, 0xFDu8, 0xFEu8, 0xFFu8];
            let expected_object = 0xFCFDFEFF;
            let object = deserializer.deserialize(&serialized).unwrap();
            assert_eq!(object, expected_object);
        }
        {
            let deserializer = IntegerDeserializer::<u32>::new(true);

            let serialized = vec![0xFCu8, 0xFDu8, 0xFDu8, 0xFDu8, 0xFEu8, 0xFFu8];
            let expected_object = 0xFCFDFEFF;
            let object = deserializer.deserialize(&serialized).unwrap();
            assert_eq!(object, expected_object);
        }
        {
            let deserializer = IntegerDeserializer::<i32>::new(false);

            let serialized = vec![0x00u8, 0x12u8, 0x34u8];
            assert!(if let Err(e) = deserializer.deserialize(&serialized) {
                matches!(
                    e.downcast_ref::<IntegerDeserialationError>(),
                    Some(IntegerDeserialationError::InvalidSerializedLength)
                )
            } else {
                false
            });
        }
        {
            let deserializer = IntegerDeserializer::<i32>::new(true);

            let serialized = vec![0x00u8, 0x12u8, 0x34u8];
            assert!(if let Err(e) = deserializer.deserialize(&serialized) {
                matches!(
                    e.downcast_ref::<IntegerDeserialationError>(),
                    Some(IntegerDeserialationError::InvalidSerializedLength)
                )
            } else {
                false
            });
        }
        {
            let deserializer = IntegerDeserializer::<u32>::new(true);

            let serialized = vec![0xFCu8, 0xFDu8, 0xFCu8, 0xFDu8, 0xFEu8, 0xFFu8];
            assert!(if let Err(e) = deserializer.deserialize(&serialized) {
                matches!(
                    e.downcast_ref::<IntegerDeserialationError>(),
                    Some(IntegerDeserialationError::InvalidSerializedContent)
                )
            } else {
                false
            });
        }
        {
            let deserializer = IntegerDeserializer::<u32>::new(true);

            let serialized = vec![0xFCu8, 0xFDu8, 0xFDu8, 0xFDu8, 0xFEu8, 0xFDu8];
            assert!(if let Err(e) = deserializer.deserialize(&serialized) {
                matches!(
                    e.downcast_ref::<IntegerDeserialationError>(),
                    Some(IntegerDeserialationError::InvalidSerializedContent)
                )
            } else {
                false
            });
        }
    }
}
