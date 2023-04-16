/*!
    # Serializer

    Copyright 2023 kaoru  <https://www.tetengo.org/>
*/

/**
   # Serializer
*/
pub trait Serializer {
    /// An object type.
    type Object;

    /// A serialized object type.
    type Serialized;

    /**
       # Serializes an object.

       ## Arguments
       * `object` - An object.

       ## Returns
       * The serialized object.
    */
    fn serialize(&self, object: Self::Object) -> Self::Serialized;
}
