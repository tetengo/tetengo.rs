/*!
    # Default Serializer

    Copyright 2023 kaoru  <https://www.tetengo.org/>
*/

use std::ops::ShrAssign;

use crate::serializer::Serializer;

/// A trait alias for Object.
pub trait ObjectTrait<Object>:
    Copy + ShrAssign<u128> + std::ops::BitAnd<Object, Output = Object> + From<u8> + Into<i128>
{
}

impl<T, U> ObjectTrait<U> for T where
    T: Copy + ShrAssign<u128> + std::ops::BitAnd<U, Output = U> + From<u8> + Into<i128>
{
}

/**
    # Default Serializer

    When the argument `fe_escape` of the constructor is true, binary bytes are
    serialized as following:

    |original byte|serialized byte|
    |-|-|
    |0x00     |0xFE       (0b11111110)            |
    |0x01-0xFC|0x01-0xFC  (0b00000001-0b11111100) |
    |0xFD     |0xFD, 0xFD (0b11111101, 0b11111101)|
    |0xFE     |0xFD, 0xFE (0b11111101, 0b11111110)|
    |0xFF     |0xFF       (0b11111111)            |

    ## Type Parameters
    * `Object` - An object type.
*/
#[derive(Debug)]
pub struct DefaultSerializer<Object: ObjectTrait<Object>> {
    fe_escape: bool,
    _phantom: std::marker::PhantomData<Object>,
}

impl<Object: ObjectTrait<Object>> DefaultSerializer<Object> {
    /**
       # Creates a default serializer.

       ## Arguments
       * `fe_escape` - Set true to escape 0xFE.
    */
    pub fn new(fe_escape: bool) -> Self {
        DefaultSerializer {
            fe_escape,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<Object: ObjectTrait<Object>> Serializer for DefaultSerializer<Object> {
    type Object = Object;

    fn serialize(&self, object: &Object) -> Vec<u8> {
        to_bytes(object, self.fe_escape)
    }
}

fn to_bytes<Object: ObjectTrait<Object>>(object: &Object, fe_escape: bool) -> Vec<u8> {
    if fe_escape {
        to_bytes_with_escape(object)
    } else {
        to_bytes_without_escape(object)
    }
}

fn to_bytes_with_escape<Object: ObjectTrait<Object>>(object: &Object) -> Vec<u8> {
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

fn to_bytes_without_escape<Object: ObjectTrait<Object>>(object: &Object) -> Vec<u8> {
    let mut bytes = vec![];
    bytes.reserve(std::mem::size_of::<Object>());
    let mut object = *object;
    for _ in 0..std::mem::size_of::<Object>() {
        let byte_object = object & Object::from(0xFFu8);
        let u128_object: i128 = byte_object.into();
        let u8_object = u128_object as u8;
        bytes.push(u8_object);
        object >>= 8;
    }
    bytes.reverse();
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    const fn nul_byte() -> u8 {
        return 0xFEu8;
    }

    #[test]
    fn serialize() {
        {
            let serializer = DefaultSerializer::<u32>::new(false);

            let object = 0x001234ABu32;
            let expected_serialized = vec![0x00u8, 0x12u8, 0x34u8, 0xABu8];
            let serialized = serializer.serialize(&object);
            assert_eq!(serialized, expected_serialized);
        }
        {
            let serializer = DefaultSerializer::<u32>::new(true);

            let object = 0x001234ABu32;
            let expected_serialized = vec![nul_byte(), 0x12u8, 0x34u8, 0xABu8];
            let serialized = serializer.serialize(&object);
            assert_eq!(serialized, expected_serialized);
            assert!(!serialized
                .iter()
                .any(|&b| b == 0x00u8 /* tetengo::trie::double_array::key_terminator() */));
        }
        {
            let serializer = DefaultSerializer::<u32>::new(false);

            let object = 0xFCFDFEFFu32;
            let expected_serialized = vec![0xFCu8, 0xFDu8, 0xFEu8, 0xFFu8];
            let serialized = serializer.serialize(&object);
            assert_eq!(serialized, expected_serialized);
        }
        {
            let serializer = DefaultSerializer::<u32>::new(true);

            let object = 0xFCFDFEFFu32;
            let expected_serialized = vec![0xFCu8, 0xFDu8, 0xFDu8, 0xFDu8, 0xFEu8, 0xFFu8];
            let serialized = serializer.serialize(&object);
            assert_eq!(serialized, expected_serialized);
            assert!(!serialized
                .iter()
                .any(|&b| b == 0x00u8 /* tetengo::trie::double_array::key_terminator() */));
        }
    }
}
