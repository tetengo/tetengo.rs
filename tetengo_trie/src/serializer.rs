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
  # Deserialization Error
*/
#[derive(Clone, Debug, thiserror::Error)]
pub enum DeserializationError {
    /// Invalid length of the serialized object.
    #[error("Invalid length of the serialized object.")]
    InvalidSeralizedLength,

    /// Invalid UTF-8 sequence.
    #[error("Invalid length of the serialized object.")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
}

/// Result
pub type Result<T> = anyhow::Result<T>;

/**
   # Deserializer
*/
pub trait Deserializer {
    /// An object type.
    type Object;

    /**
       # Deserializes an object.

       ## Arguments
       * `serialized` - A serialized object.

       ## Returns
       * The deserialized object.
    */
    fn deserialize(&self, serialized: &[u8]) -> Result<Self::Object>;
}
