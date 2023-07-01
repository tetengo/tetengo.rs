/*!
 * A double array.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

use std::fmt::{self, Debug, Formatter};

use crate::double_array_builder;
use crate::storage::Storage;

/**
 * A result type.
 *
 * # Type Parameters
 * * `T` - A type.
 */
pub type Result<T> = anyhow::Result<T>;

/**
 * A double array error.
 */
#[derive(Clone, Copy, Debug, thiserror::Error)]
pub enum DoubleArrayError {
    /**
     * density_factor must be greater than 0.
     */
    #[error("density_factor must be greater than 0.")]
    InvalidDensityFactor,
}

/// The double array element type.
pub type DoubleArrayElement<'a> = (&'a str, i32);
/**
 * A building observer set.
 */
pub struct BuldingObserverSet {
    pub(crate) adding: Box<dyn Fn(&DoubleArrayElement<'_>)>,
    pub(crate) done: Box<dyn Fn()>,
}

impl BuldingObserverSet {
    /**
     * Creates a building observer set.
     *
     * # Parameters
     * * `adding` - An adding observer.
     * * `done` - A done observer.
     */
    pub fn new(adding: Box<dyn Fn(&DoubleArrayElement<'_>)>, done: Box<dyn Fn()>) -> Self {
        Self { adding, done }
    }

    /**
     * Calls `adding`.
     *
     * # Arguments
     * * `element` - An element.
     */
    pub fn adding(&self, element: &DoubleArrayElement<'_>) {
        (self.adding)(element);
    }

    /**
     * Calls `done`.
     */
    pub fn done(&self) {
        (self.done)();
    }
}

impl Debug for BuldingObserverSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("BuldingObserverSet")
            .field("adding", &"Box<dyn FnOnce(&DoubleArrayElement<'_>)>")
            .finish()
    }
}

/**
 * A double array.
 */
pub struct DoubleArray<'a, V> {
    storage: Box<dyn Storage<V> + 'a>,
    _root_base_check_index: usize,
}

impl<'a, V: 'a> DoubleArray<'a, V> {
    /**
     * Creates a double array.
     */
    pub fn new() -> Result<Self> {
        Ok(Self {
            storage: double_array_builder::build::<V>(
                vec![],
                &BuldingObserverSet::new(Box::new(|_| {}), Box::new(|| {})),
                double_array_builder::DEFAULT_DENSITY_FACTOR,
            )?,
            _root_base_check_index: 0,
        })
    }

    /**
     * Returns the storage.
     *
     * # Returns
     * The storage.
     */
    pub fn storage(&self) -> &dyn Storage<V> {
        &*self.storage
    }

    /**
     * Returns the mutable storage.
     *
     * # Returns
     * The mutable storage.
     */
    pub fn storage_mut(&mut self) -> &mut dyn Storage<V> {
        &mut *self.storage
    }
}

impl<V> Debug for DoubleArray<'_, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("DoubleArray")
            .field("storage", &"Box<dyn Storage<V>")
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    const _EXPECTED_EMPTY_BASE_CHECK_ARRAY_EMPTY: [u32; 1] = [
    //                  BASE  CHECK  BYTECHECK
    0x000000FF, // [ 0]    0,    -1,        -1
    ];

    fn _base_check_array_of<T>(storage: &dyn Storage<T>) -> Result<Vec<u32>> {
        let size = storage.base_check_size()?;
        let mut array = Vec::<u32>::with_capacity(size);
        for i in 0..size {
            array.push(((storage.base_at(i)? as u32) << 8) | storage.check_at(i)? as u32);
        }
        Ok(array)
    }

    #[test]
    fn new() {
        let _double_array = DoubleArray::<i32>::new().unwrap();

        // assert_eq!(base_check_array_of(_double_array.storage()), EXPECTED_EMPTY_BASE_CHECK_ARRAY_EMPTY);
    }

    #[test]
    fn storage() {
        // TODO: Implement it.
        // {
        //     let double_array = DoubleArray::<i32>::new().unwrap();

        //     let base_check_array = _base_check_array_of(double_array.storage()).unwrap();

        //     assert_eq!(base_check_array, _EXPECTED_EMPTY_BASE_CHECK_ARRAY3);
        // }
    }

    #[test]
    fn storage_mut() {
        // {
        //     let mut double_array = DoubleArray::<i32>::new().unwrap();

        //     let base_check_array = _base_check_array_of(double_array.storage_mut()).unwrap();

        //     assert_eq!(base_check_array, _EXPECTED_EMPTY_BASE_CHECK_ARRAY3);
        // }
    }
}
