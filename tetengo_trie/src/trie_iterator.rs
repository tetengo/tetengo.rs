/*!
 * A trie iterator.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

use std::fmt::{self, Debug, Formatter};
use std::rc::Rc;

use crate::double_array_iterator::DoubleArrayIterator;
use crate::storage::Storage;

/**
 * A trie iterator.
 */
#[derive(Clone)]
pub struct TrieIterator<'a, T> {
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
    pub(super) fn new(
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

impl<T> Debug for TrieIterator<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("TrieIterator")
            .field("double_array_iterator", &self.double_array_iterator)
            .field("storage", &"&'a dyn Storage<T>")
            .finish()
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
            let trie = Trie::<&str, String>::new().unwrap();

            let _iterator = trie.iter();
        }
        {
            let trie = Trie::<&str, String>::new_with_elements(vec![
                (KUMAMOTO, KUMAMOTO.to_string()),
                (TAMANA, TAMANA.to_string()),
            ])
            .unwrap();

            let _iterator = trie.iter();
        }
        {
            let trie = Trie::<&str, String>::new_with_elements(vec![
                (KUMAMOTO, KUMAMOTO.to_string()),
                (TAMANA, TAMANA.to_string()),
            ])
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
            let trie = Trie::<&str, String>::new().unwrap();
            let mut iterator = trie.iter();

            assert!(iterator.next().is_none());
        }
        {
            let trie = Trie::<&str, String>::new_with_elements(vec![
                (KUMAMOTO, KUMAMOTO.to_string()),
                (TAMANA, TAMANA.to_string()),
            ])
            .unwrap();
            let mut iterator = trie.iter();

            assert_eq!(*iterator.next().unwrap().as_ref(), KUMAMOTO.to_string());
            assert_eq!(*iterator.next().unwrap().as_ref(), TAMANA.to_string());
            assert!(iterator.next().is_none());
        }
    }
}
