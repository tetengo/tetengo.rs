/*!
 * An input.
 *
 * Copyright (C) 2023-2025 kaoru  <https://www.tetengo.org/>
 */

use std::any::Any;
use std::fmt::Debug;

use anyhow::Result;

/**
 * An input error.
 */
#[derive(Clone, Copy, Debug, thiserror::Error)]
pub enum InputError {
    /**
     * The range is out of the bounds.
     */
    #[error("range out of bounds")]
    RangeOutOfBounds,

    /**
     * Mismatch concrete type.
     */
    #[error("mismatch concrete type")]
    MismatchConcreteType,
}

/**
 * An input.
 */
pub trait Input: Debug + 'static {
    /**
     * Returns `true` if this input is equal to the other.
     *
     * # Arguments
     * * `other` - The other input.
     *
     * # Returns
     * `true` if this input is equal to the other.
     */
    fn equal_to(&self, other: &dyn Input) -> bool;

    /**
     * Returns the hash value.
     *
     * # Returns
     * The hash value.
     */
    fn hash_value(&self) -> u64;

    /**
     * Returns the length.
     *
     * # Returns
     * * The length.
     */
    fn length(&self) -> usize;

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

impl dyn Input {
    /**
     * Returns `true` if the concrete type of this input is `T`.
     *
     * # Returns
     * `true` if the concrete type of this input is `T`.
     */
    pub fn is<T: Input>(&self) -> bool {
        self.as_any().is::<T>()
    }

    /**
     * Downcasts this object to a concrete type.
     *
     * # Returns
     * The object of the concrete type.
     */
    pub fn downcast_ref<T: Input>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }

    /**
     * Downcasts this mutable object to a concrete type.
     *
     * # Returns
     * The mutable object of the concrete type.
     */
    pub fn downcast_mut<T: Input>(&mut self) -> Option<&mut T> {
        self.as_any_mut().downcast_mut::<T>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct ConcreteInput1;

    impl Input for ConcreteInput1 {
        fn equal_to(&self, _: &dyn Input) -> bool {
            unimplemented!()
        }

        fn hash_value(&self) -> u64 {
            unimplemented!()
        }

        fn length(&self) -> usize {
            unimplemented!()
        }

        fn create_subrange(&self, _: usize, _: usize) -> Result<Box<dyn Input>> {
            unimplemented!()
        }

        fn append(&mut self, _: Box<dyn Input>) -> Result<()> {
            unimplemented!()
        }

        fn as_any(&self) -> &dyn Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    #[derive(Debug)]
    struct ConcreteInput2;

    impl Input for ConcreteInput2 {
        fn equal_to(&self, _: &dyn Input) -> bool {
            unimplemented!()
        }

        fn hash_value(&self) -> u64 {
            unimplemented!()
        }

        fn length(&self) -> usize {
            unimplemented!()
        }

        fn create_subrange(&self, _: usize, _: usize) -> Result<Box<dyn Input>> {
            unimplemented!()
        }

        fn append(&mut self, _: Box<dyn Input>) -> Result<()> {
            unimplemented!()
        }

        fn as_any(&self) -> &dyn Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    #[test]
    fn is() {
        let input = ConcreteInput1;
        let input_ref: &dyn Input = &input;

        assert!(input_ref.is::<ConcreteInput1>());
        assert!(!input_ref.is::<ConcreteInput2>());
    }

    #[test]
    fn downcast_ref() {
        let input = ConcreteInput1;
        let input_ref: &dyn Input = &input;

        assert!(input_ref.downcast_ref::<ConcreteInput1>().is_some());
        assert!(input_ref.downcast_ref::<ConcreteInput2>().is_none());
    }

    #[test]
    fn downcast_mut() {
        let mut input = ConcreteInput1;
        let input_ref: &mut dyn Input = &mut input;

        assert!(input_ref.downcast_mut::<ConcreteInput1>().is_some());
        assert!(input_ref.downcast_mut::<ConcreteInput2>().is_none());
    }
}
