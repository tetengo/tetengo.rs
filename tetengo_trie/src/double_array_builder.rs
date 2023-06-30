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

    let mut storage = Box::new(MemoryStorage::<T>::new());

    if !elements.is_empty() {
        let mut base_uniquer = HashSet::new();
        _build_iter(
            &elements[..],
            0,
            storage.as_mut(),
            0,
            &mut base_uniquer,
            observer,
            density_factor,
        );
    }

    observer.done();
    Ok(storage)
}

fn _build_iter<T>(
    _elements: &[(&str, i32)],
    _key_offset: usize,
    _storage: &mut dyn Storage<T>,
    _base_check_index: usize,
    _base_uniquer: &mut HashSet<i32>,
    _observer: &BuldingObserverSet,
    _density_factor: usize,
) {
    // let children_firsts = children_firsts(elements, key_offset);
}

// fn children_firsts(elements: &[(&str, i32)], key_offset: usize) -> Vec<usize> {
//     let mut firsts = vec![0];
//     {
//         let mut child_first = 0;
//         while child_first < elements.len() {
//             let child_last = elements[children_first..].iter().find(|(k, _)| k.len() > key_offset);
//         }
//     }
// }
