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
