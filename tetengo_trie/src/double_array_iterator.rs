/*!
 * A double array iterator.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

use std::fmt::{self, Debug, Formatter};

use crate::storage::Storage;

/**
 * A double array iterator.
 */
#[derive(Clone, Copy)]
pub struct DoubleArrayIterator<'a, T> {
    _storage: &'a dyn Storage<T>,
    _root_base_check_index: usize,
}

impl<'a, T> DoubleArrayIterator<'a, T> {
    /**
     * Creates a double array iterator.
     *
     * # Arguments
     * * `storage`               - A storage.
     * * `root_base_check_index` - A root base-check index.
     */
    pub fn new(storage: &'a dyn Storage<T>, root_base_check_index: usize) -> Self {
        Self {
            _storage: storage,
            _root_base_check_index: root_base_check_index,
        }
    }
}

impl<T> Debug for DoubleArrayIterator<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("DoubleArrayIterator")
            .field("storage", &" &'a dyn Storage<T>")
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::double_array::{DoubleArray, DoubleArrayElement};

    // use super::*;

    #[rustfmt::skip]
    const EXPECTED_VALUES3 : [DoubleArrayElement<'_>; 3] = [
        ("UTIGOSI", 24),
        ("UTO", 2424),
        ("SETA", 42),
    ];

    #[rustfmt::skip]
    const _EXPECTED_VALUES4 : [DoubleArrayElement<'_>; 2] = [
        ("赤瀬", 24), // "Akase" in Kanji
        ("赤水", 42), // "Akamizu" in Kanji
    ];

    #[test]
    fn new() {
        let double_array =
            DoubleArray::<i32>::new_with_elements(EXPECTED_VALUES3.to_vec()).unwrap();

        let _ = double_array.iter();
    }
}
