/*!
 * # Serializer
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

use std::error;

/**
 * # Serializer
 */
pub trait Serializer {
    /**
     * # An object type.
     */
    type Object: ?Sized;

    /**
     * # Serializes an object.
     *
     * ## Arguments
     * * `object` - An object.
     *
     * ## Returns
     * * The serialized object.
     */
    fn serialize(&self, object: &Self::Object) -> Vec<u8>;
}

/**
 * # Deserialization Error
 */
pub trait DeserializationError: error::Error {}

/**
 * # Result
 *
 * ## Type Parameters
 * * `T` - A type.
 */
pub type Result<T> = anyhow::Result<T>;

/**
 * # Deserializer
 */
pub trait Deserializer {
    /**
     * # An object type.
     */
    type Object;

    /**
     * # Deserializes an object.
     *
     * ## Arguments
     * * `serialized` - A serialized object.
     *
     * ## Returns
     * * The deserialized object.
     *
     * ## Errors
     * * `DeserializationError` - Failed to deserialize.
     */
    fn deserialize(&self, serialized: &[u8]) -> Result<Self::Object>;
}
