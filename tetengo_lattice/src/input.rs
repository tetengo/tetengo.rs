/*!
 * An input.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

use std::any::Any;

use anyhow::Result;

/**
 * An input.
 */
pub trait Input {
    /**
     * Returns the length.
     *
     * # Returns
     * * The length.
     */
    fn length(&self) -> usize;

    /**
     * Clones this storage as `Box`.
     *
     * # Returns
     * A Box of a clone of this input.
     */
    fn clone_box(&self) -> Box<dyn Input>;

    /**
     * Creates a subrange.
     *
     * # Arguments
     * * `offset` - An offset.
     * * `length` - A length.
     *
     * # Returns
     * * A box of a subrange.
     *
     * # Errors
     * * When `offset` and/or `length` are out of the range of the input.
     */
    fn create_subrange(&self, offset: usize, length: usize) -> Result<Box<dyn Input>>;

    /**
     * Appends another input.
     *
     * # Arguments
     * * `another` - A box of another input.
     *
     * # Errors
     * * When `another` is `None` or its type does not match.
     */
    fn append(&mut self, another: Box<dyn Input>) -> Result<()>;

    /**
     * Returns this object as 'Any'.
     *
     * # Returns
     * This object as 'Any'.
     */
    fn as_any(&self) -> &dyn Any;

    /**
     * Returns this mutable object as 'Any'.
     *
     * # Returns
     * This mutable object as 'Any'.
     */
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
