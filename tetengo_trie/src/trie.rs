/*!
 * A trie.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

use crate::double_array::DEFAULT_DENSITY_FACTOR;

/// The default double array density factor.
const _DEFAULT_DOUBLE_ARRAY_DENSITY_FACTOR: usize = DEFAULT_DENSITY_FACTOR;

/**
 * A trie.
 */
#[derive(Debug, Clone, Copy, Default)]
pub struct Trie {}

impl Trie {
    /**
     * Creates a trie.
     */
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::Trie;

    #[test]
    fn test_new() {
        let _ = Trie::new();
    }
}
