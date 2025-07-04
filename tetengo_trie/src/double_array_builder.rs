/*!
 * A double array builder.
 *
 * Copyright (C) 2023-2025 kaoru  <https://www.tetengo.org/>
 */

use std::collections::HashSet;
use std::fmt::Debug;

use crate::double_array::{
    BuildingObserverSet, DoubleArrayElement, KEY_TERMINATOR, VACANT_CHECK_VALUE,
};
use crate::error::Error;
use crate::memory_storage::MemoryStorage;
use crate::storage::Storage;

pub(super) fn build<T: Clone + Debug + 'static>(
    mut elements: Vec<DoubleArrayElement<'_>>,
    observer: &mut BuildingObserverSet<'_>,
    density_factor: usize,
) -> Result<Box<dyn Storage<T>>, Error> {
    if density_factor == 0 {
        return Err(Error::InvalidDensityFactor);
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

fn build_iter<T: 'static>(
    elements: &[DoubleArrayElement<'_>],
    key_offset: usize,
    storage: &mut dyn Storage<T>,
    base_check_index: usize,
    base_uniquer: &mut HashSet<i32>,
    observer: &mut BuildingObserverSet<'_>,
    density_factor: usize,
) -> Result<(), Error> {
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

    for children_first in children_firsts.iter().take(children_firsts.len() - 1) {
        let (element_key, _) = elements[*children_first];
        let char_code = char_code_at(element_key, key_offset);
        let base_plus_char = base + i32::from(char_code);
        #[allow(clippy::cast_sign_loss)]
        let next_base_check_index: usize = base_plus_char as usize;
        storage.set_check_at(next_base_check_index, char_code)?;
    }
    for i in 0..children_firsts.len() - 1 {
        let children_first = children_firsts[i];
        let children_last = children_firsts[i + 1];
        let (element_key, value) = elements[children_first];
        let char_code = char_code_at(element_key, key_offset);
        let base_plus_char = base + i32::from(char_code);
        #[allow(clippy::cast_sign_loss)]
        let next_base_check_index: usize = base_plus_char as usize;
        if char_code == KEY_TERMINATOR {
            observer.adding(&elements[children_first]);
            storage.set_base_at(next_base_check_index, value)?;
            continue;
        }
        build_iter(
            &elements[children_first..children_last],
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

fn calc_base<T: 'static>(
    firsts: &[usize],
    elements: &[DoubleArrayElement<'_>],
    key_offset: usize,
    storage: &dyn Storage<T>,
    base_check_index: usize,
    density_factor: usize,
    base_uniquer: &mut HashSet<i32>,
) -> Result<i32, Error> {
    let (element_key, _) = elements[0];
    let base_first = i32::try_from(base_check_index - (base_check_index / density_factor))
        .expect("Base check calculation should fit in i32")
        - i32::from(char_code_at(element_key, key_offset))
        + 1;
    for base in base_first.. {
        let first_last = firsts[firsts.len() - 1];
        let occupied = elements
            .iter()
            .take(first_last)
            .skip(firsts[0])
            .find_map(|&(key, _)| {
                let base_plus_char = base + i32::from(char_code_at(key, key_offset));
                #[allow(clippy::cast_sign_loss)]
                let next_base_check_index: usize = base_plus_char as usize;
                match storage.check_at(next_base_check_index) {
                    Ok(check) => {
                        if check == VACANT_CHECK_VALUE {
                            None
                        } else {
                            Some(Ok(()))
                        }
                    }
                    Err(e) => Some(Err(e)),
                }
            });
        if occupied.is_none() && !base_uniquer.contains(&base) {
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
            .unwrap_or(elements.len() - child_first)
            + child_first;

        debug_assert!(firsts.last().unwrap() < &child_last);
        firsts.push(child_last);

        child_first = child_last;
    }
    debug_assert!(!firsts.is_empty());
    firsts
}

const fn char_code_at(bytes: &[u8], index: usize) -> u8 {
    if index < bytes.len() {
        bytes[index]
    } else {
        KEY_TERMINATOR
    }
}
