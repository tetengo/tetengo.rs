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
    fn serialize<'a>(&self, object: &'a Self::Object) -> &'a [u8];
}
