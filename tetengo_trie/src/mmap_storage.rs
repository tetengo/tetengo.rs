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
