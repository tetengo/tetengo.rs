/*!
    # Serializer

    Copyright 2023 kaoru  <https://www.tetengo.org/>
*/

/**
   # Serializer
*/
pub trait Serializer {
    /// An object type.
    type Object: ?Sized;

    /**
       # Serializes an object.

       ## Arguments
       * `object` - An object.

       ## Returns
       * The serialized object.
    */
    fn serialize(&self, object: &Self::Object) -> Vec<u8>;
}

/**
   # Deserializer
*/
pub trait Deserializer {
    /// An object type.
    type Object: ?Sized;

    /**
       # Deserializes an object.

       ## Arguments
       * `serialized` - A serialized object.

       ## Returns
       * The deserialized object.
    */
    fn deserialize(&self, serialized: &[u8]) -> Self::Object;
}
