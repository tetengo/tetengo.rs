/*!
 * A trie iterator.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

use std::fmt::{self, Debug, Formatter};

use crate::double_array_iterator::DoubleArrayIterator;
use crate::storage::Storage;

/**
 * A trie iterator.
 */
#[derive(Clone)]
pub struct TrieIterator<'a, T> {
    double_array_iterator: DoubleArrayIterator<'a, T>,
    _storage: &'a dyn Storage<T>,
}

impl<'a, T> TrieIterator<'a, T> {
    /**
     * Creates an iterator.
     *
     * # Arguments
     * * `double_array_iterator` - A double array iterator.
     * * `storage`               - A storage.
     */
    pub fn new(
        double_array_iterator: DoubleArrayIterator<'a, T>,
        storage: &'a dyn Storage<T>,
    ) -> Self {
        Self {
            double_array_iterator,
            _storage: storage,
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

    static TAMANA: &str = "玉名";

    #[test]
    fn new() {
        let trie = Trie::<&str, String>::new_with_elements(vec![
            (KUMAMOTO, KUMAMOTO.to_string()),
            (TAMANA, TAMANA.to_string()),
        ])
        .unwrap();

        let _iterator = trie.iter();
    }
}
