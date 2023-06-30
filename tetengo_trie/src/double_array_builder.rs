/*!
 * A double array builder.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

// use crate::storage::Storage;

use std::collections::HashSet;

use crate::double_array::{BuldingObserverSet, DoubleArrayError, Result};
use crate::memory_storage::MemoryStorage;
use crate::storage::Storage;

pub(crate) const _DEFAULT_DENSITY_FACTOR: usize = 1000;

pub(crate) fn _build<'a, T: 'a>(
    mut elements: Vec<(&str, i32)>,
    observer: &BuldingObserverSet,
    density_factor: usize,
) -> Result<Box<dyn Storage<T> + 'a>> {
    if density_factor == 0 {
        return Err(DoubleArrayError::InvalidDensityFactor.into());
    }

    elements.sort_by_key(|(k, _)| *k);

    let storage = Box::new(MemoryStorage::<T>::new());

    if !elements.is_empty() {
        let _base_uniquer = HashSet::<i32>::new();
        // build_iter()
    }

    observer.done();
    Ok(storage)
}
