/*!
 * A trie.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

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
