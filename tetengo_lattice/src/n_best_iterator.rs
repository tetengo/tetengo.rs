/*!
 * An N-best lattice path iterator.
 *
 * Copyright (C) 2023-2024 kaoru  <https://www.tetengo.org/>
 */

/**
 * An N-best lattice path iterator.
 */
#[derive(Debug)]
pub struct NBestIterator {
    current: i32,
    max: i32,
}

impl NBestIterator {
    /**
     * Creates an N-best lattice path iterator.
     *
     * # Arguments
     * * `max` - The maximum number of paths.
     */
    pub fn new(max: i32) -> Self {
        NBestIterator { current: 0, max }
    }
}

impl Iterator for NBestIterator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.max {
            self.current += 1;
            Some(self.current)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let iter = NBestIterator::new(3);
        let mut values = Vec::new();
        iter.for_each(|e| values.push(e));
        assert_eq!(values, vec![1, 2, 3]);
    }
}
