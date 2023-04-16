/*!
    # Default Serializer

    Copyright 2023 kaoru  <https://www.tetengo.org/>
*/

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
pub struct DefaultSerializer<Object> {
    _fe_escape: bool,
    _phantom: std::marker::PhantomData<Object>,
}

impl DefaultSerializer<&str> {
    /**
       # Creates a default serializer.

       ## Arguments
       * `_` - Ignored.
    */
    pub fn new(_: bool) -> Self {
        DefaultSerializer {
            _fe_escape: false,
            _phantom: std::marker::PhantomData,
        }
    }

    /**
       # Serializes an object.

       ## Arguments
       * `object` - An object.

       ## Returns
       * The serialized object.
    */
    pub fn serialize<'a>(&self, object: &'a str) -> &'a str {
        object
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        {
            let serializer = DefaultSerializer::<&str>::new(false);

            let object = "Sakuramachi";
            let expected_serialized = "Sakuramachi";
            let serialized: &str = serializer.serialize(object);
            assert_eq!(serialized, expected_serialized);
            assert_eq!(serialized.find('\0'), None);
        }
        {
            let serializer = DefaultSerializer::<&str>::new(true);

            let object = "Sakuramachi";
            let expected_serialized = "Sakuramachi";
            let serialized: &str = serializer.serialize(object);
            assert_eq!(serialized, expected_serialized);
            assert_eq!(serialized.find('\0'), None);
        }
    }
}
