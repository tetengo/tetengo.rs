/*!
    # Default Serializer

    Copyright 2023 kaoru  <https://www.tetengo.org/>
*/

/**
    # Default Serializer

    When the argument `fe_escape` of the constructor is true, binary bytes are
    serialized as following:

    <table>
        <tr><th>original byte</th><th>serialized byte</th></tr>
        <tr><td>0x00     </td><td>0xFE       (0b11111110)            </td></tr>
        <tr><td>0x01-0xFC</td><td>0x01-0xFC  (0b00000001-0b11111100) </td></tr>
        <tr><td>0xFD     </td><td>0xFD, 0xFD (0b11111101, 0b11111101)</td></tr>
        <tr><td>0xFE     </td><td>0xFD, 0xFE (0b11111101, 0b11111110)</td></tr>
        <tr><td>0xFF     </td><td>0xFF       (0b11111111)            </td></tr>
    </table>

    ## Type Parameters
    * `Object` - An object type.
*/
pub struct DefaultSerializer<Object> {
    _fe_escape: bool,
    _phantom: std::marker::PhantomData<Object>,
}

impl DefaultSerializer<&str> {
    pub fn new(_: bool) -> Self {
        DefaultSerializer {
            _fe_escape: false,
            _phantom: std::marker::PhantomData,
        }
    }

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
