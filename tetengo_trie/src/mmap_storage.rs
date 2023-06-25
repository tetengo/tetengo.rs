/*!
 * An mmap storage.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

use std::cell::RefCell;
use std::fs::File;
use std::io::Write;
use std::mem::size_of;

use hashlink::LinkedHashMap;
use memmap2::Mmap;
use once_cell::sync::Lazy;
use tempfile as _;

use crate::integer_serializer::IntegerDeserializer;
use crate::serializer::Deserializer;
use crate::storage::{Result, Storage, StorageError};
use crate::value_serializer::{ValueDeserializer, ValueSerializer};

/**
 * A file mapping.
 */
#[derive(Debug)]
pub struct FileMapping {
    _file: File,
    mmap: Mmap,
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
        Ok(Self { _file: file, mmap })
    }

    fn mmap(&self) -> &Mmap {
        &self.mmap
    }
}

#[derive(Debug)]
struct ValueCache<T> {
    cache_capacity: usize,
    map: LinkedHashMap<usize, Option<T>>,
}

impl<T> ValueCache<T> {
    fn new(cache_capacity: usize) -> Self {
        Self {
            cache_capacity,
            map: LinkedHashMap::new(),
        }
    }

    fn has(&self, index: usize) -> bool {
        self.map.contains_key(&index)
    }

    fn at(&mut self, index: usize) -> Option<&Option<T>> {
        let _ = self.map.to_back(&index);
        self.map.get(&index)
    }

    fn insert(&mut self, index: usize, value: Option<T>) {
        debug_assert!(!self.has(index));

        while self.map.len() >= self.cache_capacity {
            let _popped = self.map.pop_front();
        }

        let _inserted = self.map.insert(index, value);
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
    value_deserializer: ValueDeserializer<T>,
    value_cache: RefCell<ValueCache<T>>,
}

impl<'a, T> MmapStorage<'a, T> {
    /// A default value cache capacity.
    pub const DEFAULT_VALUE_CACHE_CAPACITY: usize = 10000;

    /**
     * Creates an mmap storage with a value cache capacity.
     *
     * `DEFAULT_VALUE_CACHE_CAPACITY` is used as the value cache capacity.
     *
     * # Arguments
     * * `file_mapping`         - A file mapping.
     * * `content_offset`       - A content offset in the file.
     * * `file_size`            - The file size.
     * * `value_deserializer`   - A deserializer for value objects.
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
    ) -> Result<Self> {
        Self::new_with_value_cache_capacity(
            file_mapping,
            content_offset,
            file_size,
            value_deserializer,
            Self::DEFAULT_VALUE_CACHE_CAPACITY,
        )
    }

    /**
     * Creates an mmap storage with a value cache capacity.
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
    pub fn new_with_value_cache_capacity(
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
            value_deserializer,
            value_cache: RefCell::new(ValueCache::new(value_cache_capacity)),
        };

        if self_.content_offset > self_.file_size {
            return Err(MmapStorageError::InvalidContentSize.into());
        }

        let base_check_count = self_.base_check_size()?;
        let fixed_value_size = self_.read_u32(size_of::<u32>() * (1 + base_check_count + 1))?;
        if fixed_value_size == 0 {
            return Err(MmapStorageError::ValueSizeNotFixed.into());
        }

        Ok(self_)
    }

    const UNINITIALIZED_BYTE: u8 = 0xFF;

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

impl<T> Storage<T> for MmapStorage<'_, T> {
    fn base_check_size(&self) -> Result<usize> {
        self.read_u32(0).map(|v| v as usize)
    }

    fn base_at(&self, base_check_index: usize) -> Result<i32> {
        let base_check = self.read_u32(size_of::<u32>() * (1 + base_check_index))?;
        Ok((base_check as i32) >> 8)
    }

    fn set_base_at(&mut self, _: usize, _: i32) -> Result<()> {
        unreachable!("Unsupported operation.");
    }

    fn check_at(&self, base_check_index: usize) -> Result<u8> {
        let base_check = self.read_u32(size_of::<u32>() * (1 + base_check_index))?;
        Ok((base_check & 0xFF) as u8)
    }

    fn set_check_at(&mut self, _: usize, _: u8) -> Result<()> {
        unreachable!("Unsupported operation.");
    }

    fn value_count(&self) -> Result<usize> {
        let base_check_count = self.base_check_size()?;
        self.read_u32(size_of::<u32>() * (1 + base_check_count))
            .map(|v| v as usize)
    }

    fn value_at(
        &self,
        value_index: usize,
        operation: fn(value: &Option<T>) -> Result<()>,
    ) -> Result<()> {
        if !self.value_cache.borrow().has(value_index) {
            let base_check_count = self.base_check_size()?;
            let fixed_value_size =
                self.read_u32(size_of::<u32>() * (1 + base_check_count + 1))? as usize;
            let offset =
                size_of::<u32>() * (1 + base_check_count + 2) + fixed_value_size * value_index;
            let serialized = self.read_bytes(offset, fixed_value_size)?;
            if serialized == vec![Self::UNINITIALIZED_BYTE; fixed_value_size] {
                self.value_cache.borrow_mut().insert(value_index, None);
            } else {
                let value = self.value_deserializer.deserialize(serialized)?;
                self.value_cache
                    .borrow_mut()
                    .insert(value_index, Some(value));
            }
        }
        operation(
            self.value_cache
                .borrow_mut()
                .at(value_index)
                .unwrap_or_else(|| unreachable!("The value must be cached.")),
        )
    }

    fn add_value_at(&mut self, _value_index: usize, _value: T) -> Result<()> {
        todo!()
    }

    fn filling_rate(&self) -> Result<f64> {
        todo!()
    }

    fn serialize(
        &self,
        _writer: &mut dyn Write,
        _value_serializer: &ValueSerializer<T>,
    ) -> Result<()> {
        todo!()
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

    #[rustfmt::skip]
    const SERIALIZED_BROKEN: [u8; 9] = [
        0x00u8, 0x00u8, 0x00u8, 0x02u8,
        0x00u8, 0x00u8, 0x2Au8, 0xFFu8,
        0x00u8,
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
                let storage = MmapStorage::new(&file_mapping, 0, file_size, deserializer);
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
                let storage = MmapStorage::new(&file_mapping, 5, file_size, deserializer);
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
                let storage = MmapStorage::new(&file_mapping, 0, file_size, deserializer);
                assert!(storage.is_err());
            }
            {
                let file = make_temporary_file(&SERIALIZED_BROKEN);
                let file_size = size_of(&file);
                let file_mapping = FileMapping::new(file).expect("Can't create a file mapping.");
                let deserializer = ValueDeserializer::<u32>::new(|serialized| {
                    static INTEGER_DESERIALIZER: Lazy<IntegerDeserializer<u32>> =
                        Lazy::new(|| IntegerDeserializer::new(false));
                    INTEGER_DESERIALIZER.deserialize(serialized)
                });
                let storage = MmapStorage::new(&file_mapping, 0, file_size, deserializer);
                assert!(storage.is_err());
            }
            {
                let file = make_temporary_file(&SERIALIZED_FIXED_VALUE_SIZE);
                let file_size = size_of(&file);
                let file_mapping = FileMapping::new(file).expect("Can't create a file mapping.");
                let deserializer = ValueDeserializer::<u32>::new(|serialized| {
                    static INTEGER_DESERIALIZER: Lazy<IntegerDeserializer<u32>> =
                        Lazy::new(|| IntegerDeserializer::new(false));
                    INTEGER_DESERIALIZER.deserialize(serialized)
                });
                let storage =
                    MmapStorage::new(&file_mapping, file_size + 1, file_size, deserializer);
                assert!(storage.is_err());
            }
        }

        #[test]
        fn new_with_value_cache_capacity() {
            let file = make_temporary_file(&SERIALIZED_FIXED_VALUE_SIZE);
            let file_size = size_of(&file);
            let file_mapping = FileMapping::new(file).expect("Can't create a file mapping.");
            let deserializer = ValueDeserializer::<u32>::new(|serialized| {
                static INTEGER_DESERIALIZER: Lazy<IntegerDeserializer<u32>> =
                    Lazy::new(|| IntegerDeserializer::new(false));
                INTEGER_DESERIALIZER.deserialize(serialized)
            });
            let storage = MmapStorage::new_with_value_cache_capacity(
                &file_mapping,
                0,
                file_size,
                deserializer,
                10000,
            );
            assert!(storage.is_ok());
        }

        #[test]
        fn base_check_size() {
            {
                let file = make_temporary_file(&SERIALIZED_FIXED_VALUE_SIZE);
                let file_size = size_of(&file);
                let file_mapping = FileMapping::new(file).expect("Can't create a file mapping.");
                let deserializer = ValueDeserializer::<u32>::new(|serialized| {
                    static INTEGER_DESERIALIZER: Lazy<IntegerDeserializer<u32>> =
                        Lazy::new(|| IntegerDeserializer::new(false));
                    INTEGER_DESERIALIZER.deserialize(serialized)
                });
                let storage = MmapStorage::new(&file_mapping, 0, file_size, deserializer)
                    .expect("Can't create a storage.");

                assert_eq!(storage.base_check_size().unwrap(), 2);
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
                let storage = MmapStorage::new(&file_mapping, 5, file_size, deserializer)
                    .expect("Can't create a storage.");

                assert_eq!(storage.base_check_size().unwrap(), 2);
            }
        }

        #[test]
        fn base_at() {
            {
                let file = make_temporary_file(&SERIALIZED_FIXED_VALUE_SIZE);
                let file_size = size_of(&file);
                let file_mapping = FileMapping::new(file).expect("Can't create a file mapping.");
                let deserializer = ValueDeserializer::<u32>::new(|serialized| {
                    static INTEGER_DESERIALIZER: Lazy<IntegerDeserializer<u32>> =
                        Lazy::new(|| IntegerDeserializer::new(false));
                    INTEGER_DESERIALIZER.deserialize(serialized)
                });
                let storage = MmapStorage::new(&file_mapping, 0, file_size, deserializer)
                    .expect("Can't create a storage.");

                assert_eq!(storage.base_at(0).unwrap(), 42);
                assert_eq!(storage.base_at(1).unwrap(), 0xFE);
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
                let storage = MmapStorage::new(&file_mapping, 5, file_size, deserializer)
                    .expect("Can't create a storage.");

                assert_eq!(storage.base_at(0).unwrap(), 42);
                assert_eq!(storage.base_at(1).unwrap(), 0xFE);
            }
        }

        #[test]
        #[should_panic]
        fn set_base_at() {
            let file = make_temporary_file(&SERIALIZED_FIXED_VALUE_SIZE);
            let file_size = size_of(&file);
            let file_mapping = FileMapping::new(file).expect("Can't create a file mapping.");
            let deserializer = ValueDeserializer::<u32>::new(|serialized| {
                static INTEGER_DESERIALIZER: Lazy<IntegerDeserializer<u32>> =
                    Lazy::new(|| IntegerDeserializer::new(false));
                INTEGER_DESERIALIZER.deserialize(serialized)
            });
            let mut storage = MmapStorage::new(&file_mapping, 0, file_size, deserializer)
                .expect("Can't create a storage.");

            let _result = storage.set_base_at(42, 4242);
        }

        #[test]
        fn check_at() {
            {
                let file = make_temporary_file(&SERIALIZED_FIXED_VALUE_SIZE);
                let file_size = size_of(&file);
                let file_mapping = FileMapping::new(file).expect("Can't create a file mapping.");
                let deserializer = ValueDeserializer::<u32>::new(|serialized| {
                    static INTEGER_DESERIALIZER: Lazy<IntegerDeserializer<u32>> =
                        Lazy::new(|| IntegerDeserializer::new(false));
                    INTEGER_DESERIALIZER.deserialize(serialized)
                });
                let storage = MmapStorage::new(&file_mapping, 0, file_size, deserializer)
                    .expect("Can't create a storage.");

                assert_eq!(
                    storage.check_at(0).unwrap(),
                    0xFF /* TODO: tetengo::trie::double_array::vacant_check_value() */
                );
                assert_eq!(storage.check_at(1).unwrap(), 24);
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
                let storage = MmapStorage::new(&file_mapping, 5, file_size, deserializer)
                    .expect("Can't create a storage.");

                assert_eq!(
                    storage.check_at(0).unwrap(),
                    0xFF /* TODO: tetengo::trie::double_array::vacant_check_value() */
                );
                assert_eq!(storage.check_at(1).unwrap(), 24);
            }
        }

        #[test]
        #[should_panic]
        fn set_check_at() {
            let file = make_temporary_file(&SERIALIZED_FIXED_VALUE_SIZE);
            let file_size = size_of(&file);
            let file_mapping = FileMapping::new(file).expect("Can't create a file mapping.");
            let deserializer = ValueDeserializer::<u32>::new(|serialized| {
                static INTEGER_DESERIALIZER: Lazy<IntegerDeserializer<u32>> =
                    Lazy::new(|| IntegerDeserializer::new(false));
                INTEGER_DESERIALIZER.deserialize(serialized)
            });
            let mut storage = MmapStorage::new(&file_mapping, 0, file_size, deserializer)
                .expect("Can't create a storage.");

            let _result = storage.set_check_at(24, 124);
        }

        #[test]
        fn value_count() {
            {
                let file = make_temporary_file(&SERIALIZED_FIXED_VALUE_SIZE);
                let file_size = size_of(&file);
                let file_mapping = FileMapping::new(file).expect("Can't create a file mapping.");
                let deserializer = ValueDeserializer::<u32>::new(|serialized| {
                    static INTEGER_DESERIALIZER: Lazy<IntegerDeserializer<u32>> =
                        Lazy::new(|| IntegerDeserializer::new(false));
                    INTEGER_DESERIALIZER.deserialize(serialized)
                });
                let storage = MmapStorage::new(&file_mapping, 0, file_size, deserializer)
                    .expect("Can't create a storage.");

                assert_eq!(storage.value_count().unwrap(), 5);
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
                let storage = MmapStorage::new(&file_mapping, 5, file_size, deserializer)
                    .expect("Can't create a storage.");

                assert_eq!(storage.value_count().unwrap(), 5);
            }
        }

        #[test]
        fn value_at() {
            {
                let file = make_temporary_file(&SERIALIZED_FIXED_VALUE_SIZE);
                let file_size = size_of(&file);
                let file_mapping = FileMapping::new(file).expect("Can't create a file mapping.");
                let deserializer = ValueDeserializer::<u32>::new(|serialized| {
                    static INTEGER_DESERIALIZER: Lazy<IntegerDeserializer<u32>> =
                        Lazy::new(|| IntegerDeserializer::new(false));
                    INTEGER_DESERIALIZER.deserialize(serialized)
                });
                let storage = MmapStorage::new(&file_mapping, 0, file_size, deserializer)
                    .expect("Can't create a storage.");

                storage
                    .value_at(0, |value| {
                        assert!(value.is_none());
                        Ok(())
                    })
                    .unwrap();
                storage
                    .value_at(1, |value| {
                        assert_eq!(value.unwrap(), 159);
                        Ok(())
                    })
                    .unwrap();
                storage
                    .value_at(2, |value| {
                        assert_eq!(value.unwrap(), 14);
                        Ok(())
                    })
                    .unwrap();
                storage
                    .value_at(3, |value| {
                        assert!(value.is_none());
                        Ok(())
                    })
                    .unwrap();
                storage
                    .value_at(4, |value| {
                        assert_eq!(value.unwrap(), 3);
                        Ok(())
                    })
                    .unwrap();
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
                let storage = MmapStorage::new(&file_mapping, 5, file_size, deserializer)
                    .expect("Can't create a storage.");

                storage
                    .value_at(0, |value| {
                        assert!(value.is_none());
                        Ok(())
                    })
                    .unwrap();
                storage
                    .value_at(1, |value| {
                        assert_eq!(value.unwrap(), 159);
                        Ok(())
                    })
                    .unwrap();
                storage
                    .value_at(2, |value| {
                        assert_eq!(value.unwrap(), 14);
                        Ok(())
                    })
                    .unwrap();
                storage
                    .value_at(3, |value| {
                        assert!(value.is_none());
                        Ok(())
                    })
                    .unwrap();
                storage
                    .value_at(4, |value| {
                        assert_eq!(value.unwrap(), 3);
                        Ok(())
                    })
                    .unwrap();
            }
        }
    }
}
