/*!
 * A double array iterator.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

use std::fmt::{self, Debug, Formatter};

use crate::double_array;
use crate::storage::Storage;

#[derive(Clone)]
pub(super) struct DoubleArrayIterator<'a, T> {
    storage: &'a dyn Storage<T>,
    base_check_index_key_stack: Vec<(usize, Vec<u8>)>,
}

impl<'a, T> DoubleArrayIterator<'a, T> {
    pub(super) fn new(storage: &'a dyn Storage<T>, root_base_check_index: usize) -> Self {
        Self {
            storage,
            base_check_index_key_stack: vec![(root_base_check_index, Vec::new())],
        }
    }
}

impl<T> Iterator for DoubleArrayIterator<'_, T> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let (base_check_index, key) = self.base_check_index_key_stack.pop()?;

        let base = match self.storage.base_at(base_check_index) {
            Ok(base) => base,
            Err(e) => {
                debug_assert!(false, "{}", e);
                return None;
            }
        };
        let check = match self.storage.check_at(base_check_index) {
            Ok(check) => check,
            Err(e) => {
                debug_assert!(false, "{}", e);
                return None;
            }
        };

        if check == double_array::KEY_TERMINATOR {
            return Some(base);
        }

        for char_code in (0..=0xFE).rev() {
            let char_code_as_uint8 = char_code as u8;
            let next_index = base + char_code_as_uint8 as i32;
            if next_index < 0 {
                continue;
            }
            let check_at_next_index = match self.storage.check_at(next_index as usize) {
                Ok(check) => check,
                Err(e) => {
                    debug_assert!(false, "{}", e);
                    return None;
                }
            };
            if check_at_next_index == char_code_as_uint8 {
                let mut next_key_tail = if char_code_as_uint8 != double_array::KEY_TERMINATOR {
                    vec![char_code_as_uint8]
                } else {
                    Vec::new()
                };
                let next_key = {
                    let mut next_key = key.clone();
                    next_key.append(&mut next_key_tail);
                    next_key
                };
                self.base_check_index_key_stack
                    .push((next_index as usize, next_key));
            }
        }

        self.next()
    }
}

impl<T> Debug for DoubleArrayIterator<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("DoubleArrayIterator")
            .field("storage", &" &'a dyn Storage<T>")
            .field(
                "base_check_index_key_stack",
                &self.base_check_index_key_stack,
            )
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::double_array::{DoubleArray, DoubleArrayElement};

    #[rustfmt::skip]
    const EXPECTED_VALUES3 : [DoubleArrayElement<'_>; 3] = [
        (b"UTIGOSI", 24),
        (b"UTO", 2424),
        (b"SETA", 42),
    ];

    #[rustfmt::skip]
    const EXPECTED_VALUES4 : [DoubleArrayElement<'_>; 2] = [
        ("赤瀬".as_bytes(), 24),
        ("赤水".as_bytes(), 42),
    ];

    #[test]
    fn new() {
        {
            let double_array = DoubleArray::<i32>::builder()
                .elements(EXPECTED_VALUES3.to_vec())
                .build()
                .unwrap();

            let _iterator = double_array.iter();
        }
        {
            let double_array = DoubleArray::<i32>::builder()
                .elements(EXPECTED_VALUES3.to_vec())
                .build()
                .unwrap();
            let mut iterator = double_array.iter();

            let _ = iterator.next();

            let mut iterator2 = iterator.clone();

            let element = iterator2.next().unwrap();

            assert_eq!(element, 24);
        }
    }

    #[test]
    fn next() {
        {
            let double_array = DoubleArray::<i32>::builder().build().unwrap();
            let mut iterator = double_array.iter();

            {
                let element = iterator.next();
                assert!(element.is_none());
            }
        }
        {
            let double_array = DoubleArray::<i32>::builder()
                .elements(EXPECTED_VALUES3.to_vec())
                .build()
                .unwrap();
            let mut iterator = double_array.iter();

            {
                let element = iterator.next().unwrap();
                assert_eq!(element, 42);
            }
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
        {
            let double_array = DoubleArray::<i32>::builder()
                .elements(EXPECTED_VALUES4.to_vec())
                .build()
                .unwrap();
            let mut iterator = double_array.iter();

            {
                let element = iterator.next().unwrap();
                assert_eq!(element, 42);
            }
            {
                let element = iterator.next().unwrap();
                assert_eq!(element, 24);
            }
            {
                let element = iterator.next();
                assert!(element.is_none());
            }
        }
        {
            let double_array = DoubleArray::<i32>::builder()
                .elements(EXPECTED_VALUES3.to_vec())
                .build()
                .unwrap();
            let values = double_array
                .iter()
                .filter(|&e| e < 100)
                .map(|e| e * 2)
                .collect::<Vec<_>>();

            assert_eq!(values, vec![84, 48]);
        }
    }
}
