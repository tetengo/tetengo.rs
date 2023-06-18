/*!
 * An mmap storage.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

use tempfile as _;

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
    use std::fs::File;

    use tempfile::tempfile;

    use super::*;

    fn make_temporary_path(_initial_content: &[u8]) -> File {
        let file = tempfile().expect("Can't create a temporary file.");
        file
    }

    #[test]
    fn new() {
        let _file = make_temporary_path(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

        let _ = MmapStorage::<u32>::new();

        // {
        //     const auto file_path = temporary_file_path(serialized_fixed_value_size);
        //     BOOST_SCOPE_EXIT(&file_path)
        //     {
        //         std::filesystem::remove(file_path);
        //     }
        //     BOOST_SCOPE_EXIT_END;

        //     const boost::interprocess::file_mapping file_mapping{ file_path.c_str(), boost::interprocess::read_only };
        //     const auto                        file_size = static_cast<std::size_t>(std::filesystem::file_size(file_path));
        //     tetengo::trie::value_deserializer deserializer{ [](const std::vector<char>& serialized) {
        //         static const tetengo::trie::default_deserializer<std::uint32_t>uint32_deserializer{ false };
        //         return uint32_deserializer(serialized);
        //     } };
        //     const tetengo::trie::mmap_storage storage{ file_mapping, 0, file_size, std::move(deserializer) };
        // }
    }
}
