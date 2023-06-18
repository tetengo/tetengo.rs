/*!
 * An mmap storage.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

/**
 * An mmap storage.
 *
 * # Type Parameters
 * * `T` - A value type.
 */
#[derive(Clone, Debug, Default)]
pub struct MmapStorage<T> {
    _dummy: T,
}

impl<T: Default> MmapStorage<T> {
    /**
     * Creates an mmap storage.
     *
     * # Returns
     * An mmap storage.
     */
    pub fn new() -> Self {
        Self {
            _dummy: T::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let _ = MmapStorage::<u32>::new();
    }
}
