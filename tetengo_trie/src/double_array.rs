/*!
 * A double array.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

use anyhow::Result;
use std::fmt::{self, Debug, Formatter};

use crate::double_array_builder;
use crate::double_array_iterator::DoubleArrayIterator;
use crate::storage::Storage;

/**
 * A double array error.
 */
#[derive(Clone, Copy, Debug, thiserror::Error)]
pub(super) enum DoubleArrayError {
    /**
     * density_factor must be greater than 0.
     */
    #[error("density_factor must be greater than 0.")]
    InvalidDensityFactor,
}

/// The double array element type.
pub(super) type DoubleArrayElement<'a> = (&'a [u8], i32);

/**
 * A building observer set.
 */
pub(super) struct BuldingObserverSet<'a> {
    adding: &'a mut dyn FnMut(&DoubleArrayElement<'_>),
    done: &'a mut dyn FnMut(),
}

impl<'a> BuldingObserverSet<'a> {
    /**
     * Creates a building observer set.
     *
     * # Parameters
     * * `adding` - An adding observer.
     * * `done` - A done observer.
     */
    pub(super) fn new(
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
    pub(super) fn adding(&mut self, element: &DoubleArrayElement<'_>) {
        (self.adding)(element);
    }

    /**
     * Calls `done`.
     */
    pub(super) fn done(&mut self) {
        (self.done)();
    }
}

impl Debug for BuldingObserverSet<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("BuldingObserverSet")
            .field("adding", &"Box<dyn FnOnce(&DoubleArrayElement<'_>)>")
            .field("done", &"Box<dyn FnOnce()>")
            .finish()
    }
}

/// The default density factor.
pub(super) const DEFAULT_DENSITY_FACTOR: usize = 1000;

/// The key terminator.
pub(super) const KEY_TERMINATOR: u8 = 0;

/// The check value for a vacant element.
pub(super) const VACANT_CHECK_VALUE: u8 = 0xFF;

/**
 * A double array.
 *
 * # Type Parameters
 * * `Value` - A value type.
 */
pub(super) struct DoubleArray<Value> {
    storage: Box<dyn Storage<Value>>,
    root_base_check_index: usize,
}

impl<Value: Clone + 'static> DoubleArray<Value> {
    /**
     * Creates a double array.
     *
     * # Errors
     * * When it fails to build a double array.
     */
    pub(super) fn new() -> Result<Self> {
        Self::new_with_elements(vec![])
    }

    /**
     * Creates a double array.
     *
     * # Arguments
     * * `elements` - Initial elements.
     *
     * # Errors
     * * When it fails to build a double array.
     */
    pub(super) fn new_with_elements(elements: Vec<DoubleArrayElement<'_>>) -> Result<Self> {
        Self::new_with_elements_buldingobserverset(
            elements,
            &mut BuldingObserverSet::new(&mut |_| {}, &mut || {}),
        )
    }

    /**
     * Creates a double array.
     *
     * # Arguments
     * * `elements`              - Initial elements.
     * * `building_observer_set` - A building observer set.
     *
     * # Errors
     * * When it fails to build a double array.
     */
    pub(super) fn new_with_elements_buldingobserverset(
        elements: Vec<DoubleArrayElement<'_>>,
        building_observer_set: &mut BuldingObserverSet<'_>,
    ) -> Result<Self> {
        Self::new_with_elements_buldingobserverset_densityfactor(
            elements,
            building_observer_set,
            DEFAULT_DENSITY_FACTOR,
        )
    }

    /**
     * Creates a double array.
     *
     * # Arguments
     * * `elements`              - Initial elements.
     * * `building_observer_set` - A building observer set.
     * * `density_factor`        - A density factor. Must be greater than 0.
     *
     * # Errors
     * * When it fails to build a double array.
     */
    pub(super) fn new_with_elements_buldingobserverset_densityfactor(
        elements: Vec<DoubleArrayElement<'_>>,
        building_observer_set: &mut BuldingObserverSet<'_>,
        density_factor: usize,
    ) -> Result<Self> {
        Ok(Self::new_with_storage(
            double_array_builder::build::<Value>(elements, building_observer_set, density_factor)?,
            0,
        ))
    }

    /**
     * Creates a double array.
     *
     * # Arguments
     * * `storage`               - A storage.
     * * `root_base_check_index` - A root base-check index.
     */
    pub(super) fn new_with_storage(
        storage: Box<dyn Storage<Value>>,
        root_base_check_index: usize,
    ) -> Self {
        Self {
            storage,
            root_base_check_index,
        }
    }

    /**
     * Finds the value correspoinding the given key.
     *
     * # Arguments
     * * `key` - A key.
     *
     * # Returns
     * The value. Or None when the double array does not have the given key.
     *
     * # Errors
     * * When it fails to access the storage.
     */
    pub(super) fn find(&self, key: &[u8]) -> Result<Option<i32>> {
        let mut terminated_key: Vec<u8>;
        let index = self.traverse({
            terminated_key = Vec::from(key);
            terminated_key.push(KEY_TERMINATOR);
            &terminated_key
        })?;
        match index {
            Some(index) => Ok(Some(self.storage.base_at(index)?)),
            None => Ok(None),
        }
    }

    /**
     * Returns an iterator.
     *
     * # Returns
     * A double array iterator.
     */
    pub(super) fn iter(&self) -> DoubleArrayIterator<'_, Value> {
        DoubleArrayIterator::new(self.storage.as_ref(), self.root_base_check_index)
    }

    /**
     * Returns a subtrie.
     *
     * # Arguments
     * * `key_prefix` - A key prefix.
     *
     * # Returns
     * A double array of the subtrie. Or None when the double array does not have the given key prefix.
     *
     * # Errors
     * * When it fails to access the storage.
     */
    pub(super) fn subtrie(&self, key_prefix: &[u8]) -> Result<Option<Self>> {
        let index = self.traverse(key_prefix)?;
        let Some(index) = index else {
            return Ok(None);
        };
        Ok(Some(Self::new_with_storage(
            self.storage().clone_box(),
            index,
        )))
    }

    fn traverse(&self, key: &[u8]) -> Result<Option<usize>> {
        let mut base_check_index = self.root_base_check_index;
        for c in key {
            let next_base_check_index =
                (self.storage.base_at(base_check_index)? + *c as i32) as usize;
            if next_base_check_index >= self.storage.base_check_size()?
                || self.storage.check_at(next_base_check_index)? != *c
            {
                return Ok(None);
            }
            base_check_index = next_base_check_index;
        }

        Ok(Some(base_check_index))
    }

    /**
     * Returns the storage.
     *
     * # Returns
     * The storage.
     */
    pub(super) fn storage(&self) -> &dyn Storage<Value> {
        self.storage.as_ref()
    }

    /**
     * Returns the mutable storage.
     *
     * # Returns
     * The mutable storage.
     */
    pub(super) fn storage_mut(&mut self) -> &mut dyn Storage<Value> {
        &mut *self.storage
    }
}

impl<Value> Debug for DoubleArray<Value> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("DoubleArray")
            .field("storage", &"Box<dyn Storage<Value>")
            .field("root_base_check_index", &self.root_base_check_index)
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
              \0
        [ 0]+---[ 1]
            |
            |' '      \0
            +---[ 2]----[ 3]
    */

    #[rustfmt::skip]
    const EXPECTED_VALUES0: [DoubleArrayElement<'_>; 2] = [
        (b"", 42),
        (b" ", 24),
    ];

    #[rustfmt::skip]
    const EXPECTED_BASE_CHECK_ARRAY0: [u32; 35] = [
        //                  BASE  CHECK  BYTECHECK
        0x000001FF, // [ 0]    1,    -1,        -1
        0x00002A00, // [ 1]   42,     0,         0
        0x000000FF, // [ 2]    0,    -1,        -1
        0x000000FF, // [ 3]    0,    -1,        -1
        0x000000FF, // [ 4]    0,    -1,        -1
        0x000000FF, // [ 5]    0,    -1,        -1
        0x000000FF, // [ 6]    0,    -1,        -1
        0x000000FF, // [ 7]    0,    -1,        -1
        0x000000FF, // [ 8]    0,    -1,        -1
        0x000000FF, // [ 9]    0,    -1,        -1
        0x000000FF, // [10]    0,    -1,        -1
        0x000000FF, // [11]    0,    -1,        -1
        0x000000FF, // [12]    0,    -1,        -1
        0x000000FF, // [13]    0,    -1,        -1
        0x000000FF, // [14]    0,    -1,        -1
        0x000000FF, // [15]    0,    -1,        -1
        0x000000FF, // [16]    0,    -1,        -1
        0x000000FF, // [17]    0,    -1,        -1
        0x000000FF, // [18]    0,    -1,        -1
        0x000000FF, // [19]    0,    -1,        -1
        0x000000FF, // [20]    0,    -1,        -1
        0x000000FF, // [21]    0,    -1,        -1
        0x000000FF, // [22]    0,    -1,        -1
        0x000000FF, // [23]    0,    -1,        -1
        0x000000FF, // [24]    0,    -1,        -1
        0x000000FF, // [25]    0,    -1,        -1
        0x000000FF, // [26]    0,    -1,        -1
        0x000000FF, // [27]    0,    -1,        -1
        0x000000FF, // [28]    0,    -1,        -1
        0x000000FF, // [29]    0,    -1,        -1
        0x000000FF, // [30]    0,    -1,        -1
        0x000000FF, // [31]    0,    -1,        -1
        0x000000FF, // [32]    0,    -1,        -1
        0x00002220, // [33]   34,     0,        32
        0x00001800, // [34]   24,    33,         0
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
        (b"UTIGOSI", 24),
        (b"UTO", 2424),
        (b"SETA", 42),
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

    /*
            0xE8    0xB5    0xA4    0xE7    0x80    0xAC      \0
        [ 0]----[ 1]----[ 2]----[ 3]+---[ 5]----[ 9]----[10]----[11]
                                    |
                                    |0xE6   0xB0    0xB4      \0
                                    +---[ 4]----[ 6]----[ 7]----[ 8]
    */

    #[rustfmt::skip]
    const EXPECTED_VALUES4 : [DoubleArrayElement<'_>; 2] = [
        ("赤瀬".as_bytes(), 24),
        ("赤水".as_bytes(), 42),
    ];

    #[rustfmt::skip]
    const EXPECTED_BASE_CHECK_ARRAY4: [u32; 12] = [
        //                  BASE  CHECK  BYTECHECK
        0xFFFF19FF, // [ 0] -231,    -1,        -1
        0xFFFF4DE8, // [ 1] -179,     0,       232
        0xFFFF5FB5, // [ 2] -161,     1,       181
        0xFFFF1EA4, // [ 3] -226,     2,       164
        0xFFFF56E6, // [ 4] -170,     3,       230
        0xFFFF89E7, // [ 5] -119,     3,       231
        0xFFFF53B0, // [ 6] -173,     4,       176
        0x000008B4, // [ 7]    8,     6,       180
        0x00002A00, // [ 8]   42,     7,         0
        0xFFFF5E80, // [ 9] -162,     5,       128
        0x00000BAC, // [10]   11,     9,       172
        0x00001800, // [11]   24,    10,         0
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
            let mut adding = |&(k, v): &DoubleArrayElement<'_>| added = Some((k.to_vec(), v));
            let mut done = || {};
            let mut observer_set = BuldingObserverSet::new(&mut adding, &mut done);

            observer_set.adding(&(b"hoge", 42));

            assert_eq!(added.unwrap(), (b"hoge".to_vec(), 42));
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
            {
                let double_array =
                    DoubleArray::<i32>::new_with_elements(EXPECTED_VALUES0.to_vec()).unwrap();

                assert_eq!(
                    base_check_array_of(double_array.storage()).unwrap(),
                    EXPECTED_BASE_CHECK_ARRAY0
                );
            }
            {
                let double_array =
                    DoubleArray::<i32>::new_with_elements(EXPECTED_VALUES3.to_vec()).unwrap();

                assert_eq!(
                    base_check_array_of(double_array.storage()).unwrap(),
                    EXPECTED_BASE_CHECK_ARRAY3
                );
            }
            {
                let double_array =
                    DoubleArray::<i32>::new_with_elements(EXPECTED_VALUES4.to_vec()).unwrap();

                assert_eq!(
                    base_check_array_of(double_array.storage()).unwrap(),
                    EXPECTED_BASE_CHECK_ARRAY4
                );
            }
        }

        #[test]
        fn new_with_elements_buldingobserverset() {
            let mut adding_called = false;
            let mut done_called = false;
            let double_array = DoubleArray::<i32>::new_with_elements_buldingobserverset(
                EXPECTED_VALUES3.to_vec(),
                &mut BuldingObserverSet::new(&mut |_| adding_called = true, &mut || {
                    done_called = true
                }),
            )
            .unwrap();

            assert_eq!(
                base_check_array_of(double_array.storage()).unwrap(),
                EXPECTED_BASE_CHECK_ARRAY3
            );
            assert!(adding_called);
            assert!(done_called);
        }

        #[test]
        fn new_with_elements_buldingobserverset_densityfactor() {
            {
                let mut adding_called = false;
                let mut done_called = false;
                let double_array =
                    DoubleArray::<i32>::new_with_elements_buldingobserverset_densityfactor(
                        EXPECTED_VALUES3.to_vec(),
                        &mut BuldingObserverSet::new(&mut |_| adding_called = true, &mut || {
                            done_called = true
                        }),
                        DEFAULT_DENSITY_FACTOR,
                    )
                    .unwrap();

                assert_eq!(
                    base_check_array_of(double_array.storage()).unwrap(),
                    EXPECTED_BASE_CHECK_ARRAY3
                );
                assert!(adding_called);
                assert!(done_called);
            }
            {
                let double_array =
                    DoubleArray::<i32>::new_with_elements_buldingobserverset_densityfactor(
                        EXPECTED_VALUES3.to_vec(),
                        &mut BuldingObserverSet::new(&mut |_| {}, &mut || {}),
                        0,
                    );

                assert!(double_array.is_err());
            }
        }

        #[test]
        fn new_with_storage() {
            let double_array0 =
                DoubleArray::<i32>::new_with_elements(EXPECTED_VALUES3.to_vec()).unwrap();
            let storage = double_array0.storage();

            let double_array1 = DoubleArray::<i32>::new_with_storage(storage.clone_box(), 8);

            assert_eq!(
                base_check_array_of(double_array1.storage()).unwrap(),
                EXPECTED_BASE_CHECK_ARRAY3
            );

            let found = double_array1.find(b"GOSI").unwrap().unwrap();
            assert_eq!(found, 24);
        }

        #[test]
        fn find() {
            {
                let double_array = DoubleArray::<i32>::new().unwrap();

                {
                    let found = double_array.find(b"SETA").unwrap();
                    assert!(found.is_none());
                }
            }
            {
                let double_array =
                    DoubleArray::<i32>::new_with_elements(EXPECTED_VALUES3.to_vec()).unwrap();

                {
                    let found = double_array.find(b"SETA").unwrap().unwrap();
                    assert_eq!(found, 42);
                }
                {
                    let found = double_array.find(b"UTIGOSI").unwrap().unwrap();
                    assert_eq!(found, 24);
                }
                {
                    let found = double_array.find(b"UTO").unwrap().unwrap();
                    assert_eq!(found, 2424);
                }
                {
                    let found = double_array.find(b"SUIZENJI").unwrap();
                    assert!(found.is_none());
                }
            }
            {
                let double_array =
                    DoubleArray::<i32>::new_with_elements(EXPECTED_VALUES4.to_vec()).unwrap();

                {
                    let found = double_array.find("赤瀬".as_bytes()).unwrap().unwrap(); // "Akase" in Kanji
                    assert_eq!(found, 24);
                }
                {
                    let found = double_array.find("赤水".as_bytes()).unwrap().unwrap(); // "Akamizu" in Kanji
                    assert_eq!(found, 42);
                }
                {
                    let found = double_array.find("水前寺".as_bytes()).unwrap(); // "Suizenji" in Kanji
                    assert!(found.is_none());
                }
            }
        }

        #[test]
        fn iter() {
            {
                let double_array = DoubleArray::<i32>::new().unwrap();

                let _iterator = double_array.iter();
            }
            {
                let double_array =
                    DoubleArray::<i32>::new_with_elements(EXPECTED_VALUES3.to_vec()).unwrap();

                let _iterator = double_array.iter();
            }
            {
                let double_array =
                    DoubleArray::<i32>::new_with_elements(EXPECTED_VALUES4.to_vec()).unwrap();

                let _iterator = double_array.iter();
            }
        }

        #[test]
        fn subtrie() {
            {
                let double_array =
                    DoubleArray::<i32>::new_with_elements(EXPECTED_VALUES3.to_vec()).unwrap();

                {
                    let subtrie = double_array.subtrie(b"U").unwrap().unwrap();
                    {
                        let found = subtrie.find(b"TIGOSI").unwrap().unwrap();
                        assert_eq!(found, 24);
                    }
                    {
                        let found = subtrie.find(b"TO").unwrap().unwrap();
                        assert_eq!(found, 2424);
                    }
                    {
                        let found = subtrie.find(b"SETA").unwrap();
                        assert!(found.is_none());
                    }
                    {
                        let found = subtrie.find(b"UTIGOSI").unwrap();
                        assert!(found.is_none());
                    }
                    {
                        let found = subtrie.find(b"SETA").unwrap();
                        assert!(found.is_none());
                    }
                    {
                        let mut iterator = subtrie.iter();

                        {
                            let element = iterator.next().unwrap();
                            assert_eq!(element, 24);
                        }
                        {
                            let element = iterator.next().unwrap();
                            assert_eq!(element, 2424);
                        }
                        {
                            let element = iterator.next();
                            assert!(element.is_none());
                        }
                    }

                    let subtrie2 = subtrie.subtrie(b"TI").unwrap().unwrap();
                    {
                        let found = subtrie2.find(b"GOSI").unwrap().unwrap();
                        assert_eq!(found, 24);
                    }
                }
                {
                    let subtrie = double_array.subtrie(b"T").unwrap();
                    assert!(subtrie.is_none());
                }
            }
            {
                let double_array =
                    DoubleArray::<i32>::new_with_elements(EXPECTED_VALUES4.to_vec()).unwrap();

                let subtrie = double_array.subtrie("赤".as_bytes()).unwrap().unwrap();
                {
                    let found = subtrie.find("水".as_bytes()).unwrap().unwrap();
                    assert_eq!(found, 42);
                }
            }
        }

        #[test]
        fn storage() {
            let double_array =
                DoubleArray::<i32>::new_with_elements(EXPECTED_VALUES3.to_vec()).unwrap();

            let base_check_array = base_check_array_of(double_array.storage()).unwrap();

            assert_eq!(base_check_array, EXPECTED_BASE_CHECK_ARRAY3);
        }

        #[test]
        fn storage_mut() {
            let mut double_array =
                DoubleArray::<i32>::new_with_elements(EXPECTED_VALUES3.to_vec()).unwrap();

            let base_check_array = base_check_array_of(double_array.storage_mut()).unwrap();

            assert_eq!(base_check_array, EXPECTED_BASE_CHECK_ARRAY3);
        }
    }
}
