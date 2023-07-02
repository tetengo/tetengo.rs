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
pub struct BuldingObserverSet<'a> {
    pub(crate) adding: &'a mut dyn FnMut(&DoubleArrayElement<'_>),
    pub(crate) done: &'a mut dyn FnMut(),
}

impl<'a> BuldingObserverSet<'a> {
    /**
     * Creates a building observer set.
     *
     * # Parameters
     * * `adding` - An adding observer.
     * * `done` - A done observer.
     */
    pub fn new(
        adding: &'a mut dyn FnMut(&DoubleArrayElement<'_>),
        done: &'a mut dyn FnMut(),
    ) -> Self {
        Self { adding, done }
    }

    /**
     * Calls `adding`.
     *
     * # Arguments
     * * `element` - An element.
     */
    pub fn adding(&mut self, element: &DoubleArrayElement<'_>) {
        (self.adding)(element);
    }

    /**
     * Calls `done`.
     */
    pub fn done(&mut self) {
        (self.done)();
    }
}

impl Debug for BuldingObserverSet<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("BuldingObserverSet")
            .field("adding", &"Box<dyn FnOnce(&DoubleArrayElement<'_>)>")
            .finish()
    }
}

/// The default density factor.
pub const DEFAULT_DENSITY_FACTOR: usize = 1000;

/// The key terminator.
pub const KEY_TERMINATOR: u8 = 0;

/// The check value for a vacant element.
pub const VACANT_CHECK_VALUE: u8 = 0xFF;

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
                &mut BuldingObserverSet::new(&mut |_| {}, &mut || {}),
                DEFAULT_DENSITY_FACTOR,
            )?,
            _root_base_check_index: 0,
        })
    }

    /**
     * Creates a double array.
     *
     * # Arguments
     * * `elements` - Initial elements.
     * * `building_observer_set` - A building observer set.
     * * `density_factor` - A density factor. Must be greater than 0.
     */
    pub fn new_with_elements(
        elements: Vec<DoubleArrayElement<'_>>,
        building_observer_set: &mut BuldingObserverSet<'_>,
        density_factor: usize,
    ) -> Result<Self> {
        Ok(Self {
            storage: double_array_builder::build::<V>(
                elements,
                building_observer_set,
                density_factor,
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
    const EXPECTED_EMPTY_BASE_CHECK_ARRAY_EMPTY: [u32; 1] = [
    //                  BASE  CHECK  BYTECHECK
    0x000000FF, // [ 0]    0,    -1,        -1
    ];

    /*
              S       E       T       A       \0
        [ 0]+---[ 1]----[ 2]----[ 4]----[ 5]----[ 6]
            |
            | U       T       I       G       O       S       I       \0
            +---[ 3]----[ 7]+---[ 8]----[ 9]----[10]----[11]----[12]----[13]
                            |
                            | O       \0
                            +---[14]----[15]
    */

    #[rustfmt::skip]
    const EXPECTED_VALUES3 : [DoubleArrayElement<'_>; 3] = [
        ("UTIGOSI", 24),
        ("UTO", 2424),
        ("SETA", 42),
    ];

    #[rustfmt::skip]
    const EXPECTED_BASE_CHECK_ARRAY3: [u32; 16] = [
        //                  BASE  CHECK  BYTECHECK
        0xFFFFAEFF, // [ 0]  -82,    -1,        -1
        0xFFFFBD53, // [ 1]  -67,     0,        83
        0xFFFFB045, // [ 2]  -80,     1,        69
        0xFFFFB355, // [ 3]  -77,     0,        85
        0xFFFFC454, // [ 4]  -60,     2,        84
        0x00000641, // [ 5]    6,     4,        65
        0x00002A00, // [ 6]   42,     5,         0
        0xFFFFBF54, // [ 7]  -65,     3,        84
        0xFFFFC249, // [ 8]  -62,     7,        73
        0xFFFFBB47, // [ 9]  -69,     8,        71
        0xFFFFB84F, // [10]  -72,     9,        79
        0xFFFFC353, // [11]  -61,    10,        83
        0x00000D49, // [12]   13,    11,        73
        0x00001800, // [13]   24,    12,         0
        0x00000F4F, // [14]   15,     7,        79
        0x00097800, // [15] 2424,    14,         0
    ];

    fn base_check_array_of<T>(storage: &dyn Storage<T>) -> Result<Vec<u32>> {
        let size = storage.base_check_size()?;
        let mut array = Vec::<u32>::with_capacity(size);
        for i in 0..size {
            array.push(((storage.base_at(i)? as u32) << 8) | storage.check_at(i)? as u32);
        }
        Ok(array)
    }

    mod building_observer_set {
        use super::*;

        #[test]
        fn new() {
            let _observer_set = BuldingObserverSet::new(&mut |_| {}, &mut || {});
        }

        #[test]
        fn adding() {
            let mut added = None;
            let mut adding = |e: &DoubleArrayElement<'_>| added = Some((e.0.to_string(), e.1));
            let mut done = || {};
            let mut observer_set = BuldingObserverSet::new(&mut adding, &mut done);

            observer_set.adding(&("hoge", 42));

            assert_eq!(added.unwrap(), (String::from("hoge"), 42));
        }

        #[test]
        fn done() {
            let mut adding = |_e: &DoubleArrayElement<'_>| {};
            let mut done_called = false;
            let mut done = || done_called = true;
            let mut observer_set = BuldingObserverSet::new(&mut adding, &mut done);

            observer_set.done();

            assert!(done_called);
        }
    }

    mod double_array {
        use super::*;

        #[test]
        fn new() {
            let double_array = DoubleArray::<i32>::new().unwrap();

            assert_eq!(
                base_check_array_of(double_array.storage()).unwrap(),
                EXPECTED_EMPTY_BASE_CHECK_ARRAY_EMPTY
            );
        }

        #[test]
        fn new_with_elements() {
            let double_array = DoubleArray::<i32>::new_with_elements(
                EXPECTED_VALUES3.to_vec(),
                &mut BuldingObserverSet::new(&mut |_| {}, &mut || {}),
                DEFAULT_DENSITY_FACTOR,
            )
            .unwrap();

            assert_eq!(
                base_check_array_of(double_array.storage()).unwrap(),
                EXPECTED_BASE_CHECK_ARRAY3
            );
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
}
