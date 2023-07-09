/*!
 * A string serializer.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

use crate::serializer::{Deserializer, DeserializerOf, Result, Serializer, SerializerOf};

/**
 * A string serializer.
 */
#[derive(Debug, Default, Clone, Copy)]
pub struct StringSerializer;

impl Serializer for StringSerializer {
    type Object<'a> = &'a str;

    fn new(_: bool) -> Self {
        StringSerializer {}
    }

    fn serialize(&self, object: &Self::Object<'_>) -> Vec<u8> {
        object.as_bytes().to_vec()
    }
}

/**
 * A string deserializer.
 */
#[derive(Debug, Default, Clone, Copy)]
pub struct StringDeserializer;

impl Deserializer for StringDeserializer {
    type Object = String;

    fn new(_: bool) -> Self {
        StringDeserializer {}
    }

    fn deserialize(&self, bytes: &[u8]) -> Result<Self::Object> {
        String::from_utf8(bytes.to_vec()).map_err(Into::into)
    }
}

impl SerializerOf<&str> for () {
    type Type = StringSerializer;
}

impl DeserializerOf<String> for () {
    type Type = StringDeserializer;
}

#[cfg(test)]
mod tests {
    use std::string::FromUtf8Error;

    use super::*;

    #[test]
    fn serialize() {
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
                e.downcast_ref::<FromUtf8Error>().is_some()
            } else {
                false
            });
        }
    }
}
