/*!
 * A string serializer.
 *
 * Copyright (C) 2023-2025 kaoru  <https://www.tetengo.org/>
 */

use crate::error::Error;
use crate::serializer::{Deserializer, DeserializerOf, Serializer, SerializerOf};

/**
 * A string (&str) serializer.
 */
#[derive(Clone, Copy, Debug, Default)]
pub struct StrSerializer;

impl Serializer for StrSerializer {
    type Object<'a> = &'a str;

    fn new(_: bool) -> Self {
        StrSerializer {}
    }

    fn serialize(&self, object: &Self::Object<'_>) -> Vec<u8> {
        object.as_bytes().to_vec()
    }
}

/**
 * A string (String) serializer.
 */
#[derive(Clone, Copy, Debug, Default)]
pub struct StringSerializer;

impl Serializer for StringSerializer {
    type Object<'a> = String;

    fn new(_: bool) -> Self {
        StringSerializer {}
    }

    fn serialize(&self, object: &Self::Object<'_>) -> Vec<u8> {
        object.as_bytes().to_vec()
    }
}

/**
 * A string (String) deserializer.
 */
#[derive(Clone, Copy, Debug, Default)]
pub struct StringDeserializer;

impl Deserializer for StringDeserializer {
    type Object = String;

    fn new(_: bool) -> Self {
        StringDeserializer {}
    }

    fn deserialize(&self, bytes: &[u8]) -> Result<Self::Object, Error> {
        String::from_utf8(bytes.to_vec()).map_err(|_| {
            Error::InvalidSerializedBytes(String::from("invalid UTF-8 byte sequence."))
        })
    }
}

impl SerializerOf<&str> for () {
    type Type = StrSerializer;
}

impl SerializerOf<String> for () {
    type Type = StringSerializer;
}

impl DeserializerOf<String> for () {
    type Type = StringDeserializer;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        {
            let serializer = <() as SerializerOf<&str>>::Type::new(false);

            let object = "Sakuramachi";
            let expected_serialized = "Sakuramachi";
            let serialized = serializer.serialize(&object);
            assert_eq!(
                std::str::from_utf8(serialized.as_slice()).unwrap_or_default(),
                expected_serialized
            );
            assert!(!serialized.iter().any(|&b| b == 0x00u8));
        }
        {
            let serializer = <() as SerializerOf<String>>::Type::new(false);

            let object = String::from("Sakuramachi");
            let expected_serialized = String::from("Sakuramachi");
            let serialized = serializer.serialize(&object);
            assert_eq!(
                std::str::from_utf8(serialized.as_slice()).unwrap_or_default(),
                expected_serialized
            );
            assert!(!serialized.iter().any(|&b| b == 0x00u8));
        }
    }

    #[test]
    fn deserialize() {
        {
            let deserializer = <() as DeserializerOf<String>>::Type::new(false);

            let serialized = "Sakuramachi".as_bytes();
            let expected_object = "Sakuramachi";
            let object = deserializer.deserialize(serialized).unwrap();
            assert_eq!(object.as_str(), expected_object);
        }
        {
            let deserializer = <() as DeserializerOf<String>>::Type::new(false);

            let serialized = &[0xFFu8, 0xFFu8, 0xFFu8];
            assert!(if let Err(e) = deserializer.deserialize(serialized) {
                format!("{}", e).contains("invalid UTF-8 byte sequence")
            } else {
                false
            });
        }
    }
}
