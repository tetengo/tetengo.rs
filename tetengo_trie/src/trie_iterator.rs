/*!
 * A trie iterator.
 *
 * Copyright (C) 2023-2025 kaoru  <https://www.tetengo.org/>
 */

use std::fmt::Debug;
use std::rc::Rc;

use crate::double_array_iterator::DoubleArrayIterator;
use crate::storage::Storage;

/**
 * A trie iterator.
 */
#[derive(Clone, Debug)]
pub struct TrieIterator<'a, T: 'static> {
    double_array_iterator: DoubleArrayIterator<'a, T>,
    storage: &'a dyn Storage<T>,
}

impl<'a, T> TrieIterator<'a, T> {
    /**
     * Creates an iterator.
     *
     * # Arguments
     * * `double_array_iterator` - A double array iterator.
     * * `storage`               - A storage.
     */
    pub(super) const fn new(
        double_array_iterator: DoubleArrayIterator<'a, T>,
        storage: &'a dyn Storage<T>,
    ) -> Self {
        Self {
            double_array_iterator,
            storage,
        }
    }
}

impl<T> Iterator for TrieIterator<'_, T> {
    type Item = Rc<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let value_index = self.double_array_iterator.next()?;
        match self.storage.value_at(value_index as usize) {
            Ok(value) => value,
            Err(e) => {
                debug_assert!(false, "{}", e);
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::trie::Trie;

    const KUMAMOTO: &str = "熊本";

    const TAMANA: &str = "玉名";

    #[test]
    fn new() {
        {
            let trie = Trie::<&str, String>::builder().build().unwrap();

            let _iterator = trie.iter();
        }
        {
            let trie = Trie::<&str, String>::builder()
                .elements(vec![
                    (KUMAMOTO, KUMAMOTO.to_string()),
                    (TAMANA, TAMANA.to_string()),
                ])
                .build()
                .unwrap();

            let _iterator = trie.iter();
        }
        {
            let trie = Trie::<&str, String>::builder()
                .elements(vec![
                    (KUMAMOTO, KUMAMOTO.to_string()),
                    (TAMANA, TAMANA.to_string()),
                ])
                .build()
                .unwrap();

            let mut iterator = trie.iter();
            let mut clone = iterator.clone();
            assert_eq!(clone.next(), iterator.next());
            assert_eq!(clone.next(), iterator.next());
            assert_eq!(clone.next(), iterator.next());
        }
    }

    #[test]
    fn next() {
        {
            let trie = Trie::<&str, String>::builder().build().unwrap();
            let mut iterator = trie.iter();

            assert!(iterator.next().is_none());
        }
        {
            let trie = Trie::<&str, String>::builder()
                .elements(vec![
                    (KUMAMOTO, KUMAMOTO.to_string()),
                    (TAMANA, TAMANA.to_string()),
                ])
                .build()
                .unwrap();
            let mut iterator = trie.iter();

            assert_eq!(*iterator.next().unwrap().as_ref(), KUMAMOTO.to_string());
            assert_eq!(*iterator.next().unwrap().as_ref(), TAMANA.to_string());
            assert!(iterator.next().is_none());
        }
    }
}
