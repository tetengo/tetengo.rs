/*!
 * A storage.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

use std::error;
use std::io::Write;

use crate::value_serializer::ValueSerializer;

/**
 * A result type.
 *
 * # Type Parameters
 * * `T` - A type.
 */
pub type Result<T> = anyhow::Result<T>;

/**
 * A storage error.
 */
pub trait StorageError: error::Error {}

/**
 * A storage.
 *
 * # Type Parameters
 * * `T` - A value type.
 */
pub trait Storage<T> {
    /**
     * Returns the base-check size.
     *
     * # Returns
     * The base-check size.
     *
     * # Errors
     * * When it fails to read the base-check size.
     */
    fn base_check_size(&self) -> Result<usize>;

    /**
     * Returns the base value.
     *
     * # Arguments
     * * `base_check_index` - A base-check index.
     *
     * # Returns
     * The base value.
     *
     * # Errors
     * * When it fails to read the base value.
     */
    fn base_at(&self, base_check_index: usize) -> Result<i32>;

    /**
     * Sets a base value.
     *
     * # Arguments
     * * `base_check_index` - A base-check index.
     * * `base`             - A base value.
     *
     * # Errors
     * * When it fails to write the base value.
     */
    fn set_base_at(&mut self, base_check_index: usize, base: i32) -> Result<()>;

    /**
     * Return the check value.
     *
     * # Arguments
     * * `base_check_index` - A base-check index.
     *
     * # Returns
     * The check value.
     *
     * # Errors
     * * When it fails to read the check value.
     */
    fn check_at(&self, base_check_index: usize) -> Result<u8>;

    /**
     * Sets a check value.
     *
     * # Arguments
     * * `base_check_index` - A base-check index.
     * * `check`            - A check value.
     *
     * # Errors
     * * When it fails to write the check value.
     */
    fn set_check_at(&mut self, base_check_index: usize, check: u8) -> Result<()>;

    /**
     * Returns the value count.
     *
     * # Returns
     * The value count.
     *
     * # Errors
     * * When it fails to read the value count.
     */
    fn value_count(&self) -> Result<usize>;

    /**
     * Returns the value object.
     *
     * # Arguments
     * * `value_index` - A value index.
     *
     * # Returns
     * The value object. Or `None` if there is no corresponding value object.
     *
     * # Errors
     * * When it fails to read the value object.
     */
    fn value_at(
        &self,
        value_index: usize,
        operation: fn(value: &Option<T>) -> Result<()>,
    ) -> Result<()>;

    /**
     * Adds a value object.
     *
     * # Arguments
     * * `value_index` - A value index.
     * * `value`       - A value object.
     *
     * # Errors
     * * When it fails to write the value object.
     */
    fn add_value_at(&mut self, value_index: usize, value: T) -> Result<()>;

    /**
     * Returns the filling rate.
     *
     * # Returns
     * The filling rate.
     *
     * # Errors
     * * When it fails to calculate the filling rate.
     */
    fn filling_rate(&self) -> Result<f64>;

    /**
     * Serializes this storage.
     *
     * # Arguments
     * * `writer`           - A writer.
     * * `value_serializer` - A serializer for value objects.
     *
     * # Errors
     * * When it fails to serialize the content.
     */
    fn serialize(
        &self,
        writer: &mut dyn Write,
        value_serializer: &ValueSerializer<T>,
    ) -> Result<()>;
}
