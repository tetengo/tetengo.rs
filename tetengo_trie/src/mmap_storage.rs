/*!
 * An mmap storage.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

use std::fs::File;
use std::mem::size_of;

use memmap2::Mmap;
use once_cell::sync::Lazy;
use tempfile as _;

use crate::integer_serializer::IntegerDeserializer;
use crate::serializer::Deserializer;
use crate::storage::{Result, StorageError};
use crate::value_serializer::ValueDeserializer;

/**
 * A file mapping.
 */
#[derive(Debug)]
pub struct FileMapping {
    _file: File,
    _mmap: Mmap,
}

impl FileMapping {
    /**
     * Creates a file mapping.
     *
     * # Arguments
     * * `file` - A file.
     *
     * # Errors
     * When it fails to memory-map the file.
     */
    pub fn new(file: File) -> Result<Self> {
        let mmap = unsafe { Mmap::map(&file)? };
        Ok(Self {
            _file: file,
            _mmap: mmap,
        })
    }

    fn mmap(&self) -> &Mmap {
        &self._mmap
    }
}

/**
 * An mmap storage error.
 */
#[derive(Clone, Copy, Debug, thiserror::Error)]
pub enum MmapStorageError {
    /**
     * content_size is greater than file_size.
     */
    #[error("content_offset is greater than file_size.")]
    InvalidContentSize,

    /**
     * The value size is not fixed.
     */
    #[error("the value size is not fixed.")]
    ValueSizeNotFixed,

    /**
     * The mmap region is out of the file size.
     */
    #[error("the mmap region is out of the file size.")]
    MmapRegionOutOfFileSize,
}

impl StorageError for MmapStorageError {}

/**
 * An mmap storage.
 *
 * # Type Parameters
 * * `T` - A value type.
 */
#[derive(Debug)]
pub struct MmapStorage<'a, T> {
    file_mapping: &'a FileMapping,
    content_offset: usize,
    file_size: usize,
    _value_deserializer: ValueDeserializer<T>,
    _value_cache_capacity: usize,
}

impl<'a, T> MmapStorage<'a, T> {
    /**
     * Creates an mmap storage.
     *
     * # Arguments
     * * `file_mapping`         - A file mapping.
     * * `content_offset`       - A content offset in the file.
     * * `file_size`            - The file size.
     * * `value_deserializer`   - A deserializer for value objects.
     * * `value_cache_capacity` - A value cache capacity.
     *
     * # Returns
     * An mmap storage.
     *
     * # Errors
     * * When the argument(s) is/are invalid.
     * * When it fails to read the file.
     */
    pub fn new(
        file_mapping: &'a FileMapping,
        content_offset: usize,
        file_size: usize,
        value_deserializer: ValueDeserializer<T>,
        value_cache_capacity: usize,
    ) -> Result<Self> {
        let self_ = Self {
            file_mapping,
            content_offset,
            file_size,
            _value_deserializer: value_deserializer,
            _value_cache_capacity: value_cache_capacity,
        };

        if self_.content_offset > self_.file_size {
            return Err(MmapStorageError::InvalidContentSize.into());
        }

        let base_check_count = self_.base_check_size();
        let fixed_value_size = self_.read_u32(size_of::<u32>() * (1 + base_check_count + 1))?;
        if fixed_value_size == 0 {
            return Err(MmapStorageError::ValueSizeNotFixed.into());
        }

        Ok(self_)
    }

    fn base_check_size(&self) -> usize {
        self.read_u32(0).unwrap() as usize
    }

    fn read_bytes(&self, offset: usize, size: usize) -> Result<&[u8]> {
        if offset + size > self.file_size {
            return Err(MmapStorageError::MmapRegionOutOfFileSize.into());
        }

        Ok(&self.file_mapping.mmap()
            [self.content_offset + offset..self.content_offset + offset + size])
    }
    fn read_u32(&self, offset: usize) -> Result<u32> {
        static U32_DESERIALIZER: Lazy<IntegerDeserializer<u32>> =
            Lazy::new(|| IntegerDeserializer::new(false));
        U32_DESERIALIZER.deserialize(self.read_bytes(offset, size_of::<u32>())?)
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Seek, SeekFrom, Write};
    use tempfile::tempfile;

    use super::*;

    #[rustfmt::skip]
    const SERIALIZED: [u8; 52] = [
        0x00u8, 0x00u8, 0x00u8, 0x02u8,
        0x00u8, 0x00u8, 0x2Au8, 0xFFu8,
        0x00u8, 0x00u8, 0xFEu8, 0x18u8,
        0x00u8, 0x00u8, 0x00u8, 0x05u8,
        0x00u8, 0x00u8, 0x00u8, 0x00u8,
        0x00u8, 0x00u8, 0x00u8, 0x00u8,
        0x00u8, 0x00u8, 0x00u8, 0x04u8,
        0x70u8, 0x69u8, 0x79u8, 0x6Fu8,
        0x00u8, 0x00u8, 0x00u8, 0x04u8,
        0x66u8, 0x75u8, 0x67u8, 0x61u8,
        0x00u8, 0x00u8, 0x00u8, 0x00u8,
        0x00u8, 0x00u8, 0x00u8, 0x04u8,
        0x68u8, 0x6Fu8, 0x67u8, 0x65u8,
    ];

    #[rustfmt::skip]
    const SERIALIZED_FIXED_VALUE_SIZE: [u8; 40] = [
        0x00u8, 0x00u8, 0x00u8, 0x02u8,
        0x00u8, 0x00u8, 0x2Au8, 0xFFu8,
        0x00u8, 0x00u8, 0xFEu8, 0x18u8,
        0x00u8, 0x00u8, 0x00u8, 0x05u8,
        0x00u8, 0x00u8, 0x00u8, 0x04u8,
        0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
        0x00u8, 0x00u8, 0x00u8, 0x9Fu8,
        0x00u8, 0x00u8, 0x00u8, 0x0Eu8,
        0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
        0x00u8, 0x00u8, 0x00u8, 0x03u8,
    ];

    #[rustfmt::skip]
    const SERIALIZED_FIXED_VALUE_SIZE_WITH_HEADER: [u8; 45] = [
        // header
        0x01u8, 0x23u8, 0x45u8, 0x67u8, 0x89u8,

        // content
        0x00u8, 0x00u8, 0x00u8, 0x02u8,
        0x00u8, 0x00u8, 0x2Au8, 0xFFu8,
        0x00u8, 0x00u8, 0xFEu8, 0x18u8,
        0x00u8, 0x00u8, 0x00u8, 0x05u8,
        0x00u8, 0x00u8, 0x00u8, 0x04u8,
        0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
        0x00u8, 0x00u8, 0x00u8, 0x9Fu8,
        0x00u8, 0x00u8, 0x00u8, 0x0Eu8,
        0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
        0x00u8, 0x00u8, 0x00u8, 0x03u8,
    ];

    fn make_temporary_file(initial_content: &[u8]) -> File {
        let mut file = tempfile().expect("Can't create a temporary file.");
        file.write_all(initial_content)
            .expect("Can't write to the temporary file.");
        let _ = file
            .seek(SeekFrom::Start(0))
            .expect("Can't seek the temporary file.");
        file
    }

    mod file_mapping {
        use super::*;

        #[test]
        fn new() {
            let file = make_temporary_file(&SERIALIZED_FIXED_VALUE_SIZE);
            let file_mapping = FileMapping::new(file);
            assert!(file_mapping.is_ok());
        }

        #[test]
        fn mmap() {
            let file = make_temporary_file(&SERIALIZED_FIXED_VALUE_SIZE);
            let file_mapping = FileMapping::new(file).expect("Can't create a file mapping.");
            let mmap = file_mapping.mmap();
            assert_eq!(&mmap[..], &SERIALIZED_FIXED_VALUE_SIZE);
        }
    }

    mod mmap_storage {
        use crate::integer_serializer::IntegerDeserializer;
        use crate::serializer::Deserializer;
        use crate::value_serializer::ValueDeserializer;

        use super::*;

        fn size_of(file: &File) -> usize {
            file.metadata().expect("Can't get the file size.").len() as usize
        }

        #[test]
        fn new() {
            {
                let file = make_temporary_file(&SERIALIZED_FIXED_VALUE_SIZE);
                let file_size = size_of(&file);
                let file_mapping = FileMapping::new(file).expect("Can't create a file mapping.");
                let deserializer = ValueDeserializer::<u32>::new(|serialized| {
                    static INTEGER_DESERIALIZER: Lazy<IntegerDeserializer<u32>> =
                        Lazy::new(|| IntegerDeserializer::new(false));
                    INTEGER_DESERIALIZER.deserialize(serialized)
                });
                let storage = MmapStorage::new(&file_mapping, 0, file_size, deserializer, 10000);
                assert!(storage.is_ok());
            }
            {
                let file = make_temporary_file(&SERIALIZED_FIXED_VALUE_SIZE_WITH_HEADER);
                let file_size = size_of(&file);
                let file_mapping = FileMapping::new(file).expect("Can't create a file mapping.");
                let deserializer = ValueDeserializer::<u32>::new(|serialized| {
                    static INTEGER_DESERIALIZER: Lazy<IntegerDeserializer<u32>> =
                        Lazy::new(|| IntegerDeserializer::new(false));
                    INTEGER_DESERIALIZER.deserialize(serialized)
                });
                let storage = MmapStorage::new(&file_mapping, 5, file_size, deserializer, 10000);
                assert!(storage.is_ok());
            }
            {
                let file = make_temporary_file(&SERIALIZED);
                let file_size = size_of(&file);
                let file_mapping = FileMapping::new(file).expect("Can't create a file mapping.");
                let deserializer = ValueDeserializer::<u32>::new(|serialized| {
                    static INTEGER_DESERIALIZER: Lazy<IntegerDeserializer<u32>> =
                        Lazy::new(|| IntegerDeserializer::new(false));
                    INTEGER_DESERIALIZER.deserialize(serialized)
                });
                let storage = MmapStorage::new(&file_mapping, 0, file_size, deserializer, 10000);
                assert!(storage.is_err());
            }
            // {
            //     const auto file_path = temporary_file_path(serialized_broken);
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
            //     BOOST_CHECK_THROW(
            //         const tetengo::trie::mmap_storage storage(file_mapping, 0, file_size, std::move(deserializer)),
            //         std::ios_base::failure);
            // }
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
            //     BOOST_CHECK_THROW(
            //         const tetengo::trie::mmap_storage storage(file_mapping, file_size + 1, file_size, std::move(deserializer)),
            //         std::invalid_argument);
            // }
        }
    }
}
