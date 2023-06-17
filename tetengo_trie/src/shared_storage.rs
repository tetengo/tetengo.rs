/*!
 * A shared storage.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

use std::fmt::{self, Debug, Formatter};
use std::rc::Rc;

use crate::storage::Storage;

/**
 * A shared storage.
 *
 * # Type Parameters
 * * `T` - A value type.
 */
#[derive(Clone, Default)]
pub struct SharedStorage<T> {
    _entity: Option<Rc<dyn Storage<T>>>,
}

impl<T> SharedStorage<T> {
    /**
     * Creates a shared storage.
     */
    pub fn new() -> Self {
        Self { _entity: None }
    }
}

impl<T> Debug for SharedStorage<T> {
    fn fmt(&self, _f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
        //write!(f, "SharedStorage")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let _storage = SharedStorage::<u32>::new();
    }
}
