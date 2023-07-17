/*!
 * A double array builder.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

use anyhow::Result;
use std::collections::HashSet;

use crate::double_array::{
    BuldingObserverSet, DoubleArrayElement, DoubleArrayError, KEY_TERMINATOR, VACANT_CHECK_VALUE,
};
use crate::memory_storage::MemoryStorage;
use crate::storage::Storage;

pub(super) fn build<T: Clone + 'static>(
    mut elements: Vec<DoubleArrayElement<'_>>,
    observer: &mut BuldingObserverSet<'_>,
    density_factor: usize,
) -> Result<Box<dyn Storage<T>>> {
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
        children_firsts.as_slice(),
        elements,
        key_offset,
        storage,
        base_check_index,
        density_factor,
        base_uniquer,
    )?;
    storage.set_base_at(base_check_index, base)?;

    for i in &children_firsts[0..children_firsts.len() - 1] {
        let (element_key, _) = elements[*i];
        let char_code = char_code_at(element_key, key_offset);
        let next_base_check_index = (base + char_code as i32) as usize;
        storage.set_check_at(next_base_check_index, char_code)?;
    }
    for i in &children_firsts[0..children_firsts.len() - 1] {
        let (element_key, _) = elements[*i];
        let char_code = char_code_at(element_key, key_offset);
        let next_base_check_index = (base + char_code as i32) as usize;
        if char_code == KEY_TERMINATOR {
            observer.adding(&elements[*i]);
            storage.set_base_at(next_base_check_index, elements[*i].1)?;
            continue;
        }
        build_iter(
            &elements[children_firsts[*i]..children_firsts[*i + 1]],
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
    firsts: &[usize],
    elements: &[DoubleArrayElement<'_>],
    key_offset: usize,
    storage: &dyn Storage<T>,
    base_check_index: usize,
    density_factor: usize,
    base_uniquer: &mut HashSet<i32>,
) -> Result<i32> {
    let (element_key, _) = elements[0];
    let base_first = (base_check_index - (base_check_index / density_factor)) as i32
        - char_code_at(element_key, key_offset) as i32
        + 1;
    for base in base_first.. {
        let first_last = firsts[firsts.len() - 1];
        let occupied = elements
            .iter()
            .take(first_last)
            .skip(firsts[0])
            .find_map(|&(key, _)| {
                let next_base_check_index = (base + char_code_at(key, key_offset) as i32) as usize;
                match storage.check_at(next_base_check_index) {
                    Ok(check) => {
                        if check != VACANT_CHECK_VALUE {
                            Some(Ok(()))
                        } else {
                            None
                        }
                    }
                    Err(e) => Some(Err(e)),
                }
            });
        if let Some(occupied) = occupied {
            occupied?
        } else {
            let _ = base_uniquer.insert(base);
            return Ok(base);
        }
    }
    unreachable!()
}

fn children_firsts(elements: &[DoubleArrayElement<'_>], key_offset: usize) -> Vec<usize> {
    let mut firsts = vec![0];
    let mut child_first = 0;
    while child_first < elements.len() {
        let (child_first_element_key, _) = elements[child_first];
        let child_last = elements
            .iter()
            .skip(child_first)
            .position(|&(key, _)| {
                char_code_at(key, key_offset) != char_code_at(child_first_element_key, key_offset)
            })
            .unwrap_or(elements.len());

        firsts.push(child_last);

        child_first = child_last;
    }
    debug_assert!(!firsts.is_empty());
    firsts
}

fn char_code_at(bytes: &[u8], index: usize) -> u8 {
    if index < bytes.len() {
        bytes[index]
    } else {
        KEY_TERMINATOR
    }
}
