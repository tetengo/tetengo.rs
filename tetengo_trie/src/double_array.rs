/*!
 * A double array.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

use std::fmt::{self, Debug, Formatter};

use crate::storage::Storage;

/**
 * A double array.
 *
 * # Type Parameters
 * * `V` - A value type.
 */
pub struct DoubleArray<V> {
    _storage: Box<dyn Storage<V>>,
    _root_base_check_index: usize,
}

impl<V> Debug for DoubleArray<V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("DoubleArray")
            .field("storage", &"Box<dyn Storage<V>")
            .finish()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn new() {
        // TODO: Implement it.
    }
}
