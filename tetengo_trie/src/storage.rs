/*!
 * A storage.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

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
 * A storage.
 *
 * # Type Parameters
 * * `T` - A value type.
 */
pub trait Storage<T>: Clone {
    /**
     * Returns the base-check size.
     *
     * # Returns
     * The base-check size.
     */
    fn base_check_size(&self) -> usize;

    /**
     * Returns the base value.
     *
     * # Arguments
     * * `base_check_index` - A base-check index.
     *
     * # Returns
     * The base balue.
     */
    fn base_at(&self, base_check_index: usize) -> i32;

    /**
     * Sets a base value.
     *
     * # Arguments
     * * `base_check_index` - A base-check index.
     * * `base`             - A base value.
     */
    fn set_base_at(&mut self, base_check_index: usize, base: i32);

    /**
     * Return the check value.
     *
     * # Arguments
     * * `base_check_index` - A base-check index.
     *
     * # Returns
     * The check value.
     */
    fn check_at(&self, base_check_index: usize) -> u8;

    /**
     * Sets a check value.
     *
     * # Arguments
     * * `base_check_index` - A base-check index.
     * * `check`            - A check value.
     */
    fn set_check_at(&mut self, base_check_index: usize, check: u8);

    /**
     * Returns the value count.
     *
     * # Returns
     * The value count.
     */
    fn value_count(&self) -> usize;

    /**
     * Returns the value object.
     *
     * # Arguments
     * * `value_index` - A value index.
     *
     * # Returns
     * The value object. Or `None` if there is no corresponding value object.
     */
    fn value_at(&self, value_index: usize) -> Option<T>;

    /**
     * Adds a value object.
     *
     * # Arguments
     * * `value_index` - A value index.
     * * `value`       - A value object.
     */
    fn add_value_at(&mut self, value_index: usize, value: T);

    /**
     * Returns the filling rate.
     *
     * # Returns
     * The filling rate.
     */
    fn filling_rate(&self) -> f64;

    /**
     * Serializes this storage.
     *
     * # Arguments
     * * `writer`           - A writer.
     * * `value_serializer` - A serializer for value objects.
     */
    fn serialize(&self, writer: &dyn Write, value_serializer: &ValueSerializer<T>);
}
