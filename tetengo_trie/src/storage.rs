/*!
 * A storage.
 *
 * Copyright (C) 2023-2025 kaoru  <https://www.tetengo.org/>
 */

use std::any::Any;
use std::fmt::Debug;
use std::io::Write;
use std::rc::Rc;

use crate::error::Error;
use crate::value_serializer::ValueSerializer;

/**
 * A storage.
 *
 * # Type Parameters
 * * `Value` - A value type.
 */
pub trait Storage<Value: 'static>: Debug + 'static {
    /**
     * Returns the base-check size.
     *
     * # Returns
     * The base-check size.
     *
     * # Errors
     * * When it fails to read the base-check size.
     */
    fn base_check_size(&self) -> Result<usize, Error>;

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
    fn base_at(&self, base_check_index: usize) -> Result<i32, Error>;

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
    fn set_base_at(&mut self, base_check_index: usize, base: i32) -> Result<(), Error>;

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
    fn check_at(&self, base_check_index: usize) -> Result<u8, Error>;

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
    fn set_check_at(&mut self, base_check_index: usize, check: u8) -> Result<(), Error>;

    /**
     * Returns the value count.
     *
     * # Returns
     * The value count.
     *
     * # Errors
     * * When it fails to read the value count.
     */
    fn value_count(&self) -> Result<usize, Error>;

    /**
     * Returns the value object.
     *
     * # Arguments
     * * `value_index` - A value index.
     *
     * # Returns
     * The value object. Or None when there is no corresponding value object.
     *
     * # Errors
     * * When it fails to read the value object.
     */
    fn value_at(&self, value_index: usize) -> Result<Option<Rc<Value>>, Error>;

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
    fn add_value_at(&mut self, value_index: usize, value: Value) -> Result<(), Error>;

    /**
     * Returns the filling rate.
     *
     * # Returns
     * The filling rate.
     *
     * # Errors
     * * When it fails to calculate the filling rate.
     */
    fn filling_rate(&self) -> Result<f64, Error>;

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
        value_serializer: &mut ValueSerializer<'_, Value>,
    ) -> Result<(), Error>;

    /**
     * Clones this storage as `Box`.
     *
     * # Returns
     * A Box of a clone of this storage.
     */
    fn clone_box(&self) -> Box<dyn Storage<Value>>;

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

impl<Value: 'static> dyn Storage<Value> {
    /**
     * Returns `true` if the concrete type of this input is `T`.
     *
     * # Returns
     * `true` if the concrete type of this input is `T`.
     */
    pub fn is<T: Storage<Value>>(&self) -> bool {
        self.as_any().is::<T>()
    }

    /**
     * Downcasts this object to a concrete type.
     *
     * # Returns
     * The object of the concrete type.
     */
    pub fn downcast_ref<T: Storage<Value>>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }

    /**
     * Downcasts this mutable object to a concrete type.
     *
     * # Returns
     * The mutable object of the concrete type.
     */
    pub fn downcast_mut<T: Storage<Value>>(&mut self) -> Option<&mut T> {
        self.as_any_mut().downcast_mut::<T>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct ConcreteStorage1;

    impl Storage<i32> for ConcreteStorage1 {
        fn base_check_size(&self) -> Result<usize, Error> {
            unimplemented!()
        }

        fn base_at(&self, _: usize) -> Result<i32, Error> {
            unimplemented!()
        }

        fn set_base_at(&mut self, _: usize, _: i32) -> Result<(), Error> {
            unimplemented!()
        }

        fn check_at(&self, _: usize) -> Result<u8, Error> {
            unimplemented!()
        }

        fn set_check_at(&mut self, _: usize, _: u8) -> Result<(), Error> {
            unimplemented!()
        }

        fn value_count(&self) -> Result<usize, Error> {
            unimplemented!()
        }

        fn value_at(&self, _: usize) -> Result<Option<Rc<i32>>, Error> {
            unimplemented!()
        }

        fn add_value_at(&mut self, _: usize, _: i32) -> Result<(), Error> {
            unimplemented!()
        }

        fn filling_rate(&self) -> Result<f64, Error> {
            unimplemented!()
        }

        fn serialize(
            &self,
            _: &mut dyn Write,
            _: &mut ValueSerializer<'_, i32>,
        ) -> Result<(), Error> {
            unimplemented!()
        }

        fn clone_box(&self) -> Box<dyn Storage<i32>> {
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

    impl Storage<i32> for ConcreteInput2 {
        fn base_check_size(&self) -> Result<usize, Error> {
            unimplemented!()
        }

        fn base_at(&self, _: usize) -> Result<i32, Error> {
            unimplemented!()
        }

        fn set_base_at(&mut self, _: usize, _: i32) -> Result<(), Error> {
            unimplemented!()
        }

        fn check_at(&self, _: usize) -> Result<u8, Error> {
            unimplemented!()
        }

        fn set_check_at(&mut self, _: usize, _: u8) -> Result<(), Error> {
            unimplemented!()
        }

        fn value_count(&self) -> Result<usize, Error> {
            unimplemented!()
        }

        fn value_at(&self, _: usize) -> Result<Option<Rc<i32>>, Error> {
            unimplemented!()
        }

        fn add_value_at(&mut self, _: usize, _: i32) -> Result<(), Error> {
            unimplemented!()
        }

        fn filling_rate(&self) -> Result<f64, Error> {
            unimplemented!()
        }

        fn serialize(
            &self,
            _: &mut dyn Write,
            _: &mut ValueSerializer<'_, i32>,
        ) -> Result<(), Error> {
            unimplemented!()
        }

        fn clone_box(&self) -> Box<dyn Storage<i32>> {
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
        let input = ConcreteStorage1;
        let input_ref: &dyn Storage<i32> = &input;

        assert!(input_ref.is::<ConcreteStorage1>());
        assert!(!input_ref.is::<ConcreteInput2>());
    }

    #[test]
    fn downcast_ref() {
        let input = ConcreteStorage1;
        let input_ref: &dyn Storage<i32> = &input;

        assert!(input_ref.downcast_ref::<ConcreteStorage1>().is_some());
        assert!(input_ref.downcast_ref::<ConcreteInput2>().is_none());
    }

    #[test]
    fn downcast_mut() {
        let mut input = ConcreteStorage1;
        let input_ref: &mut dyn Storage<i32> = &mut input;

        assert!(input_ref.downcast_mut::<ConcreteStorage1>().is_some());
        assert!(input_ref.downcast_mut::<ConcreteInput2>().is_none());
    }
}
