/*!
    # String Serializer

    Copyright 2023 kaoru  <https://www.tetengo.org/>
*/

use crate::serializer::{Deserializer, Serializer};

/**
    # String Serializer
*/
#[derive(Debug, Default, Clone, Copy)]
pub struct StringSerializer;

impl StringSerializer {
    /**
       # Creates a string serializer.
    */
    pub fn new() -> Self {
        StringSerializer {}
    }
}

impl Serializer for StringSerializer {
    type Object = str;

    fn serialize(&self, object: &str) -> Vec<u8> {
        object.as_bytes().to_vec()
    }
}

/**
   # String Deserializer
*/
#[derive(Debug, Default, Clone, Copy)]
pub struct StringDeserializer;

impl StringDeserializer {
    /**
       # Creates a string deserializer.
    */
    pub fn new() -> Self {
        StringDeserializer {}
    }
}

impl Deserializer for StringDeserializer {
    type Object = String;

    fn deserialize(&self, bytes: &[u8]) -> String {
        String::from_utf8(bytes.to_vec()).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let serializer = StringSerializer::new();

        let object = "Sakuramachi";
        let expected_serialized = "Sakuramachi";
        let serialized = serializer.serialize(object);
        assert_eq!(
            std::str::from_utf8(serialized.as_slice()).unwrap_or_default(),
            expected_serialized
        );
        assert!(!serialized.iter().any(|&b| b == 0x00u8));
    }

    #[test]
    fn deserialize() {
        let deserializer = StringDeserializer::new();

        let serialized = "Sakuramachi".as_bytes();
        let expected_object = "Sakuramachi";
        let object = deserializer.deserialize(serialized);
        assert_eq!(object.as_str(), expected_object);
    }
}
