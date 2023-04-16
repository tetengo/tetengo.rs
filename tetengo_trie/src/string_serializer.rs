/*!
    # String Serializer

    Copyright 2023 kaoru  <https://www.tetengo.org/>
*/

use crate::serializer::Serializer;

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

    fn serialize<'a>(&self, object: &'a str) -> &'a [u8] {
        object.as_bytes()
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
            std::str::from_utf8(serialized).unwrap_or_default(),
            expected_serialized
        );
        assert!(!serialized.iter().any(|&b| b == 0x00u8));
    }
}
