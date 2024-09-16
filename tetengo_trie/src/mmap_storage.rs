/*!
 * An mmap storage.
 *
 * Copyright (C) 2023-2024 kaoru  <https://www.tetengo.org/>
 */

use std::any::Any;
use std::cell::RefCell;
use std::fmt::Debug;
use std::fs::File;
use std::io::Write;
use std::ops::Range;
use std::rc::Rc;
use std::sync::LazyLock;

use anyhow::Result;
use hashlink::LinkedHashMap;
use memmap2::Mmap;
use tempfile as _;

use crate::integer_serializer::IntegerDeserializer;
use crate::serializer::Deserializer;
use crate::storage::{Storage, StorageError};
use crate::value_serializer::{ValueDeserializer, ValueSerializer};

/**
 * A file mapping error.
 */
#[derive(Clone, Copy, Debug, thiserror::Error)]
pub enum FileMappingError {
    /**
     * The range is out of the mmap.
     */
    #[error("the range is out of the mmap")]
    RangeOutOfMmap,
}

/**
 * A file mapping.
 */
#[derive(Debug)]
pub struct FileMapping {
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
     * * When it fails to memory-map the file.
     */
    pub fn new(file: &File) -> Result<Self> {
        let mmap = unsafe { Mmap::map(file)? };
        Ok(Self { mmap })
    }

    /**
     * Returns the size.
     *
     * # Returns
     * The size.
     */
    pub fn size(&self) -> usize {
        self.mmap.len()
    }

    /**
     * Returns the region.
     *
     * # Arguments
     * * `range` - A range.
     *
     * # Returns
     * The region.
     *
     * # Errors
     * * When the range is out of the mmap.
     */
    pub fn region(&self, range: Range<usize>) -> Result<&[u8]> {
        self.mmap
            .get(range)
            .ok_or(FileMappingError::RangeOutOfMmap.into())
    }
}

#[derive(Clone, Debug)]
struct ValueCache<Value> {
    cache_capacity: usize,
    map: LinkedHashMap<usize, Option<Rc<Value>>>,
}

impl<Value> ValueCache<Value> {
    fn new(cache_capacity: usize) -> Self {
        Self {
            cache_capacity,
            map: LinkedHashMap::new(),
        }
    }

    fn has(&self, index: usize) -> bool {
        self.map.contains_key(&index)
    }

    fn at(&mut self, index: usize) -> Option<&Option<Rc<Value>>> {
        let _ = self.map.to_back(&index);
        self.map.get(&index)
    }

    fn insert(&mut self, index: usize, value: Option<Rc<Value>>) {
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
    #[error("content_offset is greater than file_size")]
    InvalidContentSize,

    /**
     * The value size is not fixed.
     */
    #[error("the value size is not fixed")]
    ValueSizeNotFixed,

    /**
     * The mmap region is out of the file size.
     */
    #[error("the mmap region is out of the file size")]
    MmapRegionOutOfFileSize,
}

impl StorageError for MmapStorageError {}

/**
 * An mmap storage builder.
 *
 * # Type Parameters
 * * `Value` - A value type.
*/
#[derive(Debug)]
pub struct MmapStorageBuilder<Value: Clone + Debug> {
    file_mapping: Rc<FileMapping>,
    content_offset: usize,
    file_size: usize,
    value_deserializer: ValueDeserializer<Value>,
    value_cache_capacity: usize,
}

impl<Value: Clone + Debug + 'static> MmapStorageBuilder<Value> {
    /**
     * Sets a value cache capacity.
     *
     * # Arguments
     * * `value` - A value cache capacity.
     */
    pub const fn value_cache_capacity(mut self, value: usize) -> Self {
        self.value_cache_capacity = value;
        self
    }

    /**
     * Builds a mmap storage.
     *
     * # Returns
     * An mmap storage.
     *
     * # Errors
     * * When the argument(s) is/are invalid.
     * * When it fails to read the file.
     */
    pub fn build(self) -> Result<MmapStorage<Value>> {
        let self_ = MmapStorage::<Value> {
            file_mapping: self.file_mapping,
            content_offset: self.content_offset,
            file_size: self.file_size,
            value_deserializer: Rc::new(RefCell::new(self.value_deserializer)),
            value_cache: RefCell::new(ValueCache::new(self.value_cache_capacity)),
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
}

/**
 * An mmap storage.
 *
 * # Type Parameters
 * * `Value` - A value type.
 */
#[derive(Debug)]
pub struct MmapStorage<Value: Clone + Debug> {
    file_mapping: Rc<FileMapping>,
    content_offset: usize,
    file_size: usize,
    value_deserializer: Rc<RefCell<ValueDeserializer<Value>>>,
    value_cache: RefCell<ValueCache<Value>>,
}

impl<Value: Clone + Debug + 'static> MmapStorage<Value> {
    /// A default value cache capacity.
    pub const DEFAULT_VALUE_CACHE_CAPACITY: usize = 10000;

    /**
     * Creates an mmap storage builder.
     *
     * # Arguments
     * * `file_mapping`         - A file mapping.
     * * `content_offset`       - A content offset in the file.
     * * `file_size`            - The file size.
     * * `value_deserializer`   - A deserializer for value objects.
     *
     * # Returns
     * An mmap storage builder.
     */
    pub const fn builder(
        file_mapping: Rc<FileMapping>,
        content_offset: usize,
        file_size: usize,
        value_deserializer: ValueDeserializer<Value>,
    ) -> MmapStorageBuilder<Value> {
        MmapStorageBuilder::<Value> {
            file_mapping,
            content_offset,
            file_size,
            value_deserializer,
            value_cache_capacity: Self::DEFAULT_VALUE_CACHE_CAPACITY,
        }
    }

    fn ensure_value_cached(&self, value_index: usize) -> Result<()> {
        if self.value_cache.borrow().has(value_index) {
            return Ok(());
        }

        let base_check_count = self.base_check_size()?;
        let fixed_value_size =
            self.read_u32(size_of::<u32>() * (1 + base_check_count + 1))? as usize;
        let offset = size_of::<u32>() * (1 + base_check_count + 2) + fixed_value_size * value_index;
        let serialized = self.read_bytes(offset, fixed_value_size)?;
        if serialized == vec![Self::UNINITIALIZED_BYTE; fixed_value_size] {
            self.value_cache.borrow_mut().insert(value_index, None);
        } else {
            let value = self
                .value_deserializer
                .borrow_mut()
                .deserialize(serialized)?;
            self.value_cache
                .borrow_mut()
                .insert(value_index, Some(Rc::new(value)));
        }
        Ok(())
    }

    const UNINITIALIZED_BYTE: u8 = 0xFF;

    fn read_bytes(&self, offset: usize, size: usize) -> Result<&[u8]> {
        if offset + size > self.file_size {
            return Err(MmapStorageError::MmapRegionOutOfFileSize.into());
        }

        self.file_mapping
            .region(self.content_offset + offset..self.content_offset + offset + size)
    }
    fn read_u32(&self, offset: usize) -> Result<u32> {
        static U32_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
            LazyLock::new(|| IntegerDeserializer::new(false));
        U32_DESERIALIZER.deserialize(self.read_bytes(offset, size_of::<u32>())?)
    }
}

impl<Value: Clone + Debug + 'static> Storage<Value> for MmapStorage<Value> {
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

    fn value_at(&self, value_index: usize) -> Result<Option<Rc<Value>>> {
        self.ensure_value_cached(value_index)?;
        let mut cache_ref = self.value_cache.borrow_mut();
        let Some(value) = cache_ref.at(value_index) else {
            unreachable!("The value must be cached.")
        };
        Ok(value.clone())
    }

    fn add_value_at(&mut self, _: usize, _: Value) -> Result<()> {
        unreachable!("Unsupported operation.");
    }

    fn filling_rate(&self) -> Result<f64> {
        let base_check_count = self.base_check_size()?;
        let mut empty_count = 0usize;
        for i in 0..base_check_count {
            let base_check = self.read_u32(size_of::<u32>() * (1 + i))?;
            if base_check == 0x000000FF {
                empty_count += 1;
            }
        }
        Ok(1.0 - (empty_count as f64) / (base_check_count as f64))
    }

    fn serialize(&self, _: &mut dyn Write, _: &mut ValueSerializer<'_, Value>) -> Result<()> {
        unreachable!("Unsupported operation.");
    }

    fn clone_box(&self) -> Box<dyn Storage<Value>> {
        Box::new(Self {
            file_mapping: self.file_mapping.clone(),
            file_size: self.file_size,
            content_offset: self.content_offset,
            value_deserializer: self.value_deserializer.clone(),
            value_cache: RefCell::new(self.value_cache.borrow().clone()),
        })
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Seek, SeekFrom, Write};

    use tempfile::tempfile;

    use crate::serializer::Serializer;

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
    const SERIALIZED_FIXED_VALUE_SIZE_FOR_CALCULATING_FILLING_RATE: [u8; 20] = [
            0x00u8, 0x00u8, 0x00u8, 0x02u8,
            0x00u8, 0x00u8, 0x00u8, 0xFFu8,
            0x00u8, 0x00u8, 0xFEu8, 0x18u8,
            0x00u8, 0x00u8, 0x00u8, 0x00u8,
            0x00u8, 0x00u8, 0x00u8, 0x04u8,
        ];

    #[rustfmt::skip]
    const SERIALIZED_BROKEN: [u8; 9] = [
        0x00u8, 0x00u8, 0x00u8, 0x02u8,
        0x00u8, 0x00u8, 0x2Au8, 0xFFu8,
        0x00u8,
    ];

    fn base_check_array_of<Value>(storage: &dyn Storage<Value>) -> Vec<u32> {
        let size = storage.base_check_size().unwrap();
        let mut array = Vec::<u32>::with_capacity(size);
        for i in 0..size {
            array.push(
                ((storage.base_at(i).unwrap() as u32) << 8u32)
                    | storage.check_at(i).unwrap() as u32,
            );
        }
        array
    }

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
            let file_mapping = FileMapping::new(&file);
            assert!(file_mapping.is_ok());
        }

        #[test]
        fn size() {
            let file = make_temporary_file(&SERIALIZED_FIXED_VALUE_SIZE);
            let file_mapping = FileMapping::new(&file).expect("Can't create a file mapping.");

            assert_eq!(file_mapping.size(), SERIALIZED_FIXED_VALUE_SIZE.len());
        }

        #[test]
        fn region() {
            let file = make_temporary_file(&SERIALIZED_FIXED_VALUE_SIZE);
            let file_mapping = FileMapping::new(&file).expect("Can't create a file mapping.");

            {
                let region = file_mapping.region(3..24).unwrap();
                assert_eq!(region, &SERIALIZED_FIXED_VALUE_SIZE[3..24]);
            }
            {
                let region = file_mapping.region(0..file_mapping.size()).unwrap();
                assert_eq!(region, &SERIALIZED_FIXED_VALUE_SIZE);
            }
            {
                let region = file_mapping.region(0..file_mapping.size() + 1);
                assert!(region.is_err());
            }
        }
    }

    mod mmap_storage {
        use std::io::Cursor;

        use crate::double_array::VACANT_CHECK_VALUE;
        use crate::integer_serializer::{IntegerDeserializer, IntegerSerializer};
        use crate::serializer::Deserializer;
        use crate::value_serializer::ValueDeserializer;

        use super::*;

        fn file_size_of(file: &File) -> usize {
            file.metadata().expect("Can't get the file size.").len() as usize
        }

        #[test]
        fn builder() {
            {
                let file = make_temporary_file(&SERIALIZED_FIXED_VALUE_SIZE);
                let file_size = file_size_of(&file);
                let file_mapping =
                    Rc::new(FileMapping::new(&file).expect("Can't create a file mapping."));
                let deserializer = ValueDeserializer::<u32>::new(Box::new(|serialized| {
                    static INTEGER_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
                        LazyLock::new(|| IntegerDeserializer::new(false));
                    INTEGER_DESERIALIZER.deserialize(serialized)
                }));
                let storage =
                    MmapStorage::builder(file_mapping, 0, file_size, deserializer).build();
                assert!(storage.is_ok());
            }
            {
                let file = make_temporary_file(&SERIALIZED_FIXED_VALUE_SIZE_WITH_HEADER);
                let file_size = file_size_of(&file);
                let file_mapping =
                    Rc::new(FileMapping::new(&file).expect("Can't create a file mapping."));
                let deserializer = ValueDeserializer::<u32>::new(Box::new(|serialized| {
                    static INTEGER_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
                        LazyLock::new(|| IntegerDeserializer::new(false));
                    INTEGER_DESERIALIZER.deserialize(serialized)
                }));
                let storage =
                    MmapStorage::builder(file_mapping, 5, file_size, deserializer).build();
                assert!(storage.is_ok());
            }
            {
                let file = make_temporary_file(&SERIALIZED);
                let file_size = file_size_of(&file);
                let file_mapping =
                    Rc::new(FileMapping::new(&file).expect("Can't create a file mapping."));
                let deserializer = ValueDeserializer::<u32>::new(Box::new(|serialized| {
                    static INTEGER_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
                        LazyLock::new(|| IntegerDeserializer::new(false));
                    INTEGER_DESERIALIZER.deserialize(serialized)
                }));
                let storage =
                    MmapStorage::builder(file_mapping, 0, file_size, deserializer).build();
                assert!(storage.is_err());
            }
            {
                let file = make_temporary_file(&SERIALIZED_BROKEN);
                let file_size = file_size_of(&file);
                let file_mapping =
                    Rc::new(FileMapping::new(&file).expect("Can't create a file mapping."));
                let deserializer = ValueDeserializer::<u32>::new(Box::new(|serialized| {
                    static INTEGER_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
                        LazyLock::new(|| IntegerDeserializer::new(false));
                    INTEGER_DESERIALIZER.deserialize(serialized)
                }));
                let storage =
                    MmapStorage::builder(file_mapping, 0, file_size, deserializer).build();
                assert!(storage.is_err());
            }
            {
                let file = make_temporary_file(&SERIALIZED_FIXED_VALUE_SIZE);
                let file_size = file_size_of(&file);
                let file_mapping =
                    Rc::new(FileMapping::new(&file).expect("Can't create a file mapping."));
                let deserializer = ValueDeserializer::<u32>::new(Box::new(|serialized| {
                    static INTEGER_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
                        LazyLock::new(|| IntegerDeserializer::new(false));
                    INTEGER_DESERIALIZER.deserialize(serialized)
                }));
                let storage =
                    MmapStorage::builder(file_mapping, file_size + 1, file_size, deserializer)
                        .build();
                assert!(storage.is_err());
            }
            {
                let file = make_temporary_file(&SERIALIZED_FIXED_VALUE_SIZE);
                let file_size = file_size_of(&file);
                let file_mapping =
                    Rc::new(FileMapping::new(&file).expect("Can't create a file mapping."));
                let deserializer = ValueDeserializer::<u32>::new(Box::new(|serialized| {
                    static INTEGER_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
                        LazyLock::new(|| IntegerDeserializer::new(false));
                    INTEGER_DESERIALIZER.deserialize(serialized)
                }));
                let storage = MmapStorage::builder(file_mapping, 0, file_size, deserializer)
                    .value_cache_capacity(10000)
                    .build();
                assert!(storage.is_ok());
            }
        }

        #[test]
        fn base_check_size() {
            {
                let file = make_temporary_file(&SERIALIZED_FIXED_VALUE_SIZE);
                let file_size = file_size_of(&file);
                let file_mapping =
                    Rc::new(FileMapping::new(&file).expect("Can't create a file mapping."));
                let deserializer = ValueDeserializer::<u32>::new(Box::new(|serialized| {
                    static INTEGER_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
                        LazyLock::new(|| IntegerDeserializer::new(false));
                    INTEGER_DESERIALIZER.deserialize(serialized)
                }));
                let storage = MmapStorage::builder(file_mapping, 0, file_size, deserializer)
                    .build()
                    .expect("Can't create a storage.");

                assert_eq!(storage.base_check_size().unwrap(), 2);
            }
            {
                let file = make_temporary_file(&SERIALIZED_FIXED_VALUE_SIZE_WITH_HEADER);
                let file_size = file_size_of(&file);
                let file_mapping =
                    Rc::new(FileMapping::new(&file).expect("Can't create a file mapping."));
                let deserializer = ValueDeserializer::<u32>::new(Box::new(|serialized| {
                    static INTEGER_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
                        LazyLock::new(|| IntegerDeserializer::new(false));
                    INTEGER_DESERIALIZER.deserialize(serialized)
                }));
                let storage = MmapStorage::builder(file_mapping, 5, file_size, deserializer)
                    .build()
                    .expect("Can't create a storage.");

                assert_eq!(storage.base_check_size().unwrap(), 2);
            }
        }

        #[test]
        fn base_at() {
            {
                let file = make_temporary_file(&SERIALIZED_FIXED_VALUE_SIZE);
                let file_size = file_size_of(&file);
                let file_mapping =
                    Rc::new(FileMapping::new(&file).expect("Can't create a file mapping."));
                let deserializer = ValueDeserializer::<u32>::new(Box::new(|serialized| {
                    static INTEGER_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
                        LazyLock::new(|| IntegerDeserializer::new(false));
                    INTEGER_DESERIALIZER.deserialize(serialized)
                }));
                let storage = MmapStorage::builder(file_mapping, 0, file_size, deserializer)
                    .build()
                    .expect("Can't create a storage.");

                assert_eq!(storage.base_at(0).unwrap(), 42);
                assert_eq!(storage.base_at(1).unwrap(), 0xFE);
            }
            {
                let file = make_temporary_file(&SERIALIZED_FIXED_VALUE_SIZE_WITH_HEADER);
                let file_size = file_size_of(&file);
                let file_mapping =
                    Rc::new(FileMapping::new(&file).expect("Can't create a file mapping."));
                let deserializer = ValueDeserializer::<u32>::new(Box::new(|serialized| {
                    static INTEGER_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
                        LazyLock::new(|| IntegerDeserializer::new(false));
                    INTEGER_DESERIALIZER.deserialize(serialized)
                }));
                let storage = MmapStorage::builder(file_mapping, 5, file_size, deserializer)
                    .build()
                    .expect("Can't create a storage.");

                assert_eq!(storage.base_at(0).unwrap(), 42);
                assert_eq!(storage.base_at(1).unwrap(), 0xFE);
            }
        }

        #[test]
        #[should_panic]
        fn set_base_at() {
            let file = make_temporary_file(&SERIALIZED_FIXED_VALUE_SIZE);
            let file_size = file_size_of(&file);
            let file_mapping =
                Rc::new(FileMapping::new(&file).expect("Can't create a file mapping."));
            let deserializer = ValueDeserializer::<u32>::new(Box::new(|serialized| {
                static INTEGER_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
                    LazyLock::new(|| IntegerDeserializer::new(false));
                INTEGER_DESERIALIZER.deserialize(serialized)
            }));
            let mut storage = MmapStorage::builder(file_mapping, 0, file_size, deserializer)
                .build()
                .expect("Can't create a storage.");

            let _result = storage.set_base_at(42, 4242);
        }

        #[test]
        fn check_at() {
            {
                let file = make_temporary_file(&SERIALIZED_FIXED_VALUE_SIZE);
                let file_size = file_size_of(&file);
                let file_mapping =
                    Rc::new(FileMapping::new(&file).expect("Can't create a file mapping."));
                let deserializer = ValueDeserializer::<u32>::new(Box::new(|serialized| {
                    static INTEGER_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
                        LazyLock::new(|| IntegerDeserializer::new(false));
                    INTEGER_DESERIALIZER.deserialize(serialized)
                }));
                let storage = MmapStorage::builder(file_mapping, 0, file_size, deserializer)
                    .build()
                    .expect("Can't create a storage.");

                assert_eq!(storage.check_at(0).unwrap(), VACANT_CHECK_VALUE);
                assert_eq!(storage.check_at(1).unwrap(), 24);
            }
            {
                let file = make_temporary_file(&SERIALIZED_FIXED_VALUE_SIZE_WITH_HEADER);
                let file_size = file_size_of(&file);
                let file_mapping =
                    Rc::new(FileMapping::new(&file).expect("Can't create a file mapping."));
                let deserializer = ValueDeserializer::<u32>::new(Box::new(|serialized| {
                    static INTEGER_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
                        LazyLock::new(|| IntegerDeserializer::new(false));
                    INTEGER_DESERIALIZER.deserialize(serialized)
                }));
                let storage = MmapStorage::builder(file_mapping, 5, file_size, deserializer)
                    .build()
                    .expect("Can't create a storage.");

                assert_eq!(storage.check_at(0).unwrap(), VACANT_CHECK_VALUE);
                assert_eq!(storage.check_at(1).unwrap(), 24);
            }
        }

        #[test]
        #[should_panic]
        fn set_check_at() {
            let file = make_temporary_file(&SERIALIZED_FIXED_VALUE_SIZE);
            let file_size = file_size_of(&file);
            let file_mapping =
                Rc::new(FileMapping::new(&file).expect("Can't create a file mapping."));
            let deserializer = ValueDeserializer::<u32>::new(Box::new(|serialized| {
                static INTEGER_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
                    LazyLock::new(|| IntegerDeserializer::new(false));
                INTEGER_DESERIALIZER.deserialize(serialized)
            }));
            let mut storage = MmapStorage::builder(file_mapping, 0, file_size, deserializer)
                .build()
                .expect("Can't create a storage.");

            let _result = storage.set_check_at(24, 124);
        }

        #[test]
        fn value_count() {
            {
                let file = make_temporary_file(&SERIALIZED_FIXED_VALUE_SIZE);
                let file_size = file_size_of(&file);
                let file_mapping =
                    Rc::new(FileMapping::new(&file).expect("Can't create a file mapping."));
                let deserializer = ValueDeserializer::<u32>::new(Box::new(|serialized| {
                    static INTEGER_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
                        LazyLock::new(|| IntegerDeserializer::new(false));
                    INTEGER_DESERIALIZER.deserialize(serialized)
                }));
                let storage = MmapStorage::builder(file_mapping, 0, file_size, deserializer)
                    .build()
                    .expect("Can't create a storage.");

                assert_eq!(storage.value_count().unwrap(), 5);
            }
            {
                let file = make_temporary_file(&SERIALIZED_FIXED_VALUE_SIZE_WITH_HEADER);
                let file_size = file_size_of(&file);
                let file_mapping =
                    Rc::new(FileMapping::new(&file).expect("Can't create a file mapping."));
                let deserializer = ValueDeserializer::<u32>::new(Box::new(|serialized| {
                    static INTEGER_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
                        LazyLock::new(|| IntegerDeserializer::new(false));
                    INTEGER_DESERIALIZER.deserialize(serialized)
                }));
                let storage = MmapStorage::builder(file_mapping, 5, file_size, deserializer)
                    .build()
                    .expect("Can't create a storage.");

                assert_eq!(storage.value_count().unwrap(), 5);
            }
        }

        #[test]
        fn value_at() {
            {
                let file = make_temporary_file(&SERIALIZED_FIXED_VALUE_SIZE);
                let file_size = file_size_of(&file);
                let file_mapping =
                    Rc::new(FileMapping::new(&file).expect("Can't create a file mapping."));
                let deserializer = ValueDeserializer::<u32>::new(Box::new(|serialized| {
                    static INTEGER_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
                        LazyLock::new(|| IntegerDeserializer::new(false));
                    INTEGER_DESERIALIZER.deserialize(serialized)
                }));
                let storage = MmapStorage::builder(file_mapping, 0, file_size, deserializer)
                    .build()
                    .expect("Can't create a storage.");

                assert!(storage.value_at(0).unwrap().is_none());
                assert_eq!(*storage.value_at(1).unwrap().unwrap(), 159);
                assert_eq!(*storage.value_at(2).unwrap().unwrap(), 14);
                assert!(storage.value_at(3).unwrap().is_none());
                assert_eq!(*storage.value_at(4).unwrap().unwrap(), 3);
            }
            {
                let file = make_temporary_file(&SERIALIZED_FIXED_VALUE_SIZE_WITH_HEADER);
                let file_size = file_size_of(&file);
                let file_mapping =
                    Rc::new(FileMapping::new(&file).expect("Can't create a file mapping."));
                let deserializer = ValueDeserializer::<u32>::new(Box::new(|serialized| {
                    static INTEGER_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
                        LazyLock::new(|| IntegerDeserializer::new(false));
                    INTEGER_DESERIALIZER.deserialize(serialized)
                }));
                let storage = MmapStorage::builder(file_mapping, 5, file_size, deserializer)
                    .build()
                    .expect("Can't create a storage.");

                assert!(storage.value_at(0).unwrap().is_none());
                assert_eq!(*storage.value_at(1).unwrap().unwrap(), 159);
                assert_eq!(*storage.value_at(2).unwrap().unwrap(), 14);
                assert!(storage.value_at(3).unwrap().is_none());
                assert_eq!(*storage.value_at(4).unwrap().unwrap(), 3);
            }
        }

        #[test]
        #[should_panic]
        fn add_value_at() {
            let file = make_temporary_file(&SERIALIZED_FIXED_VALUE_SIZE);
            let file_size = file_size_of(&file);
            let file_mapping =
                Rc::new(FileMapping::new(&file).expect("Can't create a file mapping."));
            let deserializer = ValueDeserializer::<u32>::new(Box::new(|serialized| {
                static INTEGER_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
                    LazyLock::new(|| IntegerDeserializer::new(false));
                INTEGER_DESERIALIZER.deserialize(serialized)
            }));
            let mut storage = MmapStorage::builder(file_mapping, 0, file_size, deserializer)
                .build()
                .expect("Can't create a storage.");

            let _result = storage.add_value_at(24, 124);
        }

        #[test]
        fn filling_rate() {
            let file =
                make_temporary_file(&SERIALIZED_FIXED_VALUE_SIZE_FOR_CALCULATING_FILLING_RATE);
            let file_size = file_size_of(&file);
            let file_mapping =
                Rc::new(FileMapping::new(&file).expect("Can't create a file mapping."));
            let deserializer = ValueDeserializer::<u32>::new(Box::new(|serialized| {
                static INTEGER_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
                    LazyLock::new(|| IntegerDeserializer::new(false));
                INTEGER_DESERIALIZER.deserialize(serialized)
            }));
            let storage = MmapStorage::builder(file_mapping, 0, file_size, deserializer)
                .build()
                .expect("Can't create a storage.");

            assert!((storage.filling_rate().unwrap() - 1.0 / 2.0).abs() < 0.1);
        }

        #[test]
        #[should_panic]
        fn serialize() {
            let file = make_temporary_file(&SERIALIZED_FIXED_VALUE_SIZE);
            let file_size = file_size_of(&file);
            let file_mapping =
                Rc::new(FileMapping::new(&file).expect("Can't create a file mapping."));
            let deserializer = ValueDeserializer::<u32>::new(Box::new(|serialized| {
                static INTEGER_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
                    LazyLock::new(|| IntegerDeserializer::new(false));
                INTEGER_DESERIALIZER.deserialize(serialized)
            }));
            let storage = MmapStorage::builder(file_mapping, 0, file_size, deserializer)
                .build()
                .expect("Can't create a storage.");

            let mut writer = Cursor::new(Vec::new());
            let mut serializer = ValueSerializer::<u32>::new(
                Box::new(|value| {
                    static INTEGER_SERIALIZER: LazyLock<IntegerSerializer<u32>> =
                        LazyLock::new(|| IntegerSerializer::new(false));
                    INTEGER_SERIALIZER.serialize(value)
                }),
                size_of::<u32>(),
            );

            let _result = storage.serialize(&mut writer, &mut serializer);
        }

        #[test]
        fn clone() {
            {
                let file = make_temporary_file(&SERIALIZED_FIXED_VALUE_SIZE);
                let file_size = file_size_of(&file);
                let file_mapping =
                    Rc::new(FileMapping::new(&file).expect("Can't create a file mapping."));
                let deserializer = ValueDeserializer::<u32>::new(Box::new(|serialized| {
                    static INTEGER_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
                        LazyLock::new(|| IntegerDeserializer::new(false));
                    INTEGER_DESERIALIZER.deserialize(serialized)
                }));
                let storage = MmapStorage::builder(file_mapping, 0, file_size, deserializer)
                    .build()
                    .expect("Can't create a storage.");

                let clone = storage.clone_box();
                assert_eq!(
                    base_check_array_of(clone.as_ref()),
                    base_check_array_of(&storage)
                );
                assert_eq!(clone.value_count().unwrap(), storage.value_count().unwrap());
            }
            {
                let file = make_temporary_file(&SERIALIZED_FIXED_VALUE_SIZE_WITH_HEADER);
                let file_size = file_size_of(&file);
                let file_mapping =
                    Rc::new(FileMapping::new(&file).expect("Can't create a file mapping."));
                let deserializer = ValueDeserializer::<u32>::new(Box::new(|serialized| {
                    static INTEGER_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
                        LazyLock::new(|| IntegerDeserializer::new(false));
                    INTEGER_DESERIALIZER.deserialize(serialized)
                }));
                let storage = MmapStorage::builder(file_mapping, 5, file_size, deserializer)
                    .build()
                    .expect("Can't create a storage.");

                let clone = storage.clone_box();
                assert_eq!(
                    base_check_array_of(clone.as_ref()),
                    base_check_array_of(&storage)
                );
                assert_eq!(clone.value_count().unwrap(), storage.value_count().unwrap());
            }
        }

        #[test]
        fn as_any() {
            let file =
                make_temporary_file(&SERIALIZED_FIXED_VALUE_SIZE_FOR_CALCULATING_FILLING_RATE);
            let file_size = file_size_of(&file);
            let file_mapping =
                Rc::new(FileMapping::new(&file).expect("Can't create a file mapping."));
            let deserializer = ValueDeserializer::<u32>::new(Box::new(|serialized| {
                static INTEGER_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
                    LazyLock::new(|| IntegerDeserializer::new(false));
                INTEGER_DESERIALIZER.deserialize(serialized)
            }));
            let storage = MmapStorage::builder(file_mapping, 0, file_size, deserializer)
                .build()
                .expect("Can't create a storage.");

            let _ = storage.as_any();
        }

        #[test]
        fn as_any_mut() {
            let file =
                make_temporary_file(&SERIALIZED_FIXED_VALUE_SIZE_FOR_CALCULATING_FILLING_RATE);
            let file_size = file_size_of(&file);
            let file_mapping =
                Rc::new(FileMapping::new(&file).expect("Can't create a file mapping."));
            let deserializer = ValueDeserializer::<u32>::new(Box::new(|serialized| {
                static INTEGER_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
                    LazyLock::new(|| IntegerDeserializer::new(false));
                INTEGER_DESERIALIZER.deserialize(serialized)
            }));
            let mut storage = MmapStorage::builder(file_mapping, 0, file_size, deserializer)
                .build()
                .expect("Can't create a storage.");

            let _ = storage.as_any_mut();
        }
    }
}
