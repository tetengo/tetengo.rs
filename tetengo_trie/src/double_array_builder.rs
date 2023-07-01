/*!
 * A double array builder.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

use std::collections::HashSet;

use crate::double_array::{BuldingObserverSet, DoubleArrayElement, DoubleArrayError, Result};
use crate::memory_storage::MemoryStorage;
use crate::storage::Storage;

pub(super) const DEFAULT_DENSITY_FACTOR: usize = 1000;

pub(super) fn build<'a, T: 'a>(
    mut elements: Vec<DoubleArrayElement<'_>>,
    observer: &mut BuldingObserverSet<'_>,
    density_factor: usize,
) -> Result<Box<dyn Storage<T> + 'a>> {
    if density_factor == 0 {
        return Err(DoubleArrayError::InvalidDensityFactor.into());
    }

    elements.sort_by_key(|(k, _)| *k);

    let mut storage = Box::new(MemoryStorage::<T>::new());

    if !elements.is_empty() {
        let mut base_uniquer = HashSet::new();
        build_iter(
            &elements[..],
            0,
            storage.as_mut(),
            0,
            &mut base_uniquer,
            observer,
            density_factor,
        )?;
    }

    observer.done();
    Ok(storage)
}

fn build_iter<T>(
    elements: &[DoubleArrayElement<'_>],
    key_offset: usize,
    storage: &mut dyn Storage<T>,
    base_check_index: usize,
    base_uniquer: &mut HashSet<i32>,
    observer: &mut BuldingObserverSet<'_>,
    density_factor: usize,
) -> Result<()> {
    let children_firsts = children_firsts(elements, key_offset);

    let base = calc_base(
        elements,
        key_offset,
        storage,
        base_check_index,
        density_factor,
        base_uniquer,
    )?;
    storage.set_base_at(base_check_index, base)?;

    for &(key, _) in elements.iter().take(children_firsts.len() - 1) {
        let char_code = char_code_at(key, key_offset);
        let next_base_check_index = (base + char_code as i32) as usize;
        storage.set_check_at(next_base_check_index, char_code)?;
    }
    for (i, &(key, value)) in elements.iter().enumerate().take(children_firsts.len() - 1) {
        let char_code = char_code_at(key, key_offset);
        let next_base_check_index = (base + char_code as i32) as usize;
        if char_code == 0
        /* TODO: double_array::key_terminator() */
        {
            observer.adding(&(key, value));
            storage.set_base_at(next_base_check_index, value)?;
            continue;
        }
        build_iter(
            &elements[children_firsts[i]..children_firsts[i + 1]],
            key_offset + 1,
            storage,
            next_base_check_index,
            base_uniquer,
            observer,
            density_factor,
        )?;
    }
    Ok(())
}

fn calc_base<T>(
    elements: &[DoubleArrayElement<'_>],
    key_offset: usize,
    storage: &dyn Storage<T>,
    base_check_index: usize,
    density_factor: usize,
    base_uniquer: &mut HashSet<i32>,
) -> Result<i32> {
    let base_first = (base_check_index - (base_check_index / density_factor)) as i32
        - char_code_at(elements[0].0, key_offset) as i32
        + 1;
    for base in base_first.. {
        let first_last = elements.len() - 1;
        let mut occupied = false;
        for &(key, _) in elements.iter().take(first_last) {
            let next_base_check_index = (base + char_code_at(key, key_offset) as i32) as usize;
            let check = storage.check_at(next_base_check_index)?;
            if check != 0 {
                occupied = true;
                break;
            }
        }
        if !occupied && !base_uniquer.contains(&base) {
            let _ = base_uniquer.insert(base);
            return Ok(base);
        }
    }
    unreachable!()
}

fn children_firsts(elements: &[DoubleArrayElement<'_>], key_offset: usize) -> Vec<usize> {
    let mut firsts = vec![0];
    for &(child_key, _) in elements {
        let child_last = elements
            .iter()
            .skip_while(|(key, _)| {
                char_code_at(key, key_offset) == char_code_at(child_key, key_offset)
            })
            .count();
        firsts.push(child_last);
    }
    debug_assert!(!firsts.is_empty());
    firsts
}

fn char_code_at(string: &str, index: usize) -> u8 {
    if index < string.len() {
        string.as_bytes()[index]
    } else {
        0 /* TODO: double_array::key_terminator() */
    }
}
