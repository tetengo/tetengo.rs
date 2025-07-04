/*!
 * An mmap storage.
 *
 * Copyright (C) 2023-2025 kaoru  <https://www.tetengo.org/>
 */

use std::any::Any;
use std::cell::RefCell;
use std::fmt::Debug;
use std::io::Write;
use std::rc::Rc;
use std::sync::LazyLock;

use hashlink::LinkedHashMap;
use tempfile as _;

use crate::error::Error;
use crate::file_mapping::FileMapping;
use crate::integer_serializer::IntegerDeserializer;
use crate::serializer::Deserializer;
use crate::storage::Storage;
use crate::value_serializer::{ValueDeserializer, ValueSerializer};

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
    #[must_use]
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
    pub fn build(self) -> Result<MmapStorage<Value>, Error> {
        let self_ = MmapStorage::<Value> {
            file_mapping: self.file_mapping,
            content_offset: self.content_offset,
            file_size: self.file_size,
            value_deserializer: Rc::new(RefCell::new(self.value_deserializer)),
            value_cache: RefCell::new(ValueCache::new(self.value_cache_capacity)),
        };

        if self_.content_offset > self_.file_size {
            return Err(Error::LargerContentOffsetThanFileSize);
        }

        let base_check_count = self_.base_check_size()?;
        let fixed_value_size = self_.read_u32(size_of::<u32>() * (1 + base_check_count + 1))?;
        if fixed_value_size == 0 {
            return Err(Error::ZeroFixedValueSize);
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

    fn ensure_value_cached(&self, value_index: usize) -> Result<(), Error> {
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

    fn read_bytes(&self, offset: usize, size: usize) -> Result<&[u8], Error> {
        if offset + size > self.file_size {
            return Err(Error::UnexpectedEof);
        }

        self.file_mapping
            .region(self.content_offset + offset..self.content_offset + offset + size)
    }
    fn read_u32(&self, offset: usize) -> Result<u32, Error> {
        static U32_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
            LazyLock::new(|| IntegerDeserializer::new(false));
        U32_DESERIALIZER.deserialize(self.read_bytes(offset, size_of::<u32>())?)
    }
}

impl<Value: Clone + Debug + 'static> Storage<Value> for MmapStorage<Value> {
    fn base_check_size(&self) -> Result<usize, Error> {
        self.read_u32(0).map(|v| v as usize)
    }

    fn base_at(&self, base_check_index: usize) -> Result<i32, Error> {
        let base_check = self.read_u32(size_of::<u32>() * (1 + base_check_index))?;
        #[allow(clippy::cast_possible_wrap)]
        let result = (base_check as i32) >> 8;
        Ok(result)
    }

    fn set_base_at(&mut self, _: usize, _: i32) -> Result<(), Error> {
        unreachable!("Unsupported operation.");
    }

    fn check_at(&self, base_check_index: usize) -> Result<u8, Error> {
        let base_check = self.read_u32(size_of::<u32>() * (1 + base_check_index))?;
        Ok((base_check & 0xFF) as u8)
    }

    fn set_check_at(&mut self, _: usize, _: u8) -> Result<(), Error> {
        unreachable!("Unsupported operation.");
    }

    fn value_count(&self) -> Result<usize, Error> {
        let base_check_count = self.base_check_size()?;
        self.read_u32(size_of::<u32>() * (1 + base_check_count))
            .map(|v| v as usize)
    }

    fn value_at(&self, value_index: usize) -> Result<Option<Rc<Value>>, Error> {
        self.ensure_value_cached(value_index)?;
        let mut cache_ref = self.value_cache.borrow_mut();
        let Some(value) = cache_ref.at(value_index) else {
            unreachable!("The value must be cached.")
        };
        Ok(value.clone())
    }

    fn add_value_at(&mut self, _: usize, _: Value) -> Result<(), Error> {
        unreachable!("Unsupported operation.");
    }

    fn filling_rate(&self) -> Result<f64, Error> {
        let base_check_count = self.base_check_size()?;
        let mut empty_count = 0usize;
        for i in 0..base_check_count {
            let base_check = self.read_u32(size_of::<u32>() * (1 + i))?;
            if base_check == 0x000000FF {
                empty_count += 1;
            }
        }
        let empty_count_f64 =
            f64::from(u32::try_from(empty_count).map_err(|e| Error::InternalError(e.into()))?);
        let base_check_count_f64 =
            f64::from(u32::try_from(base_check_count).map_err(|e| Error::InternalError(e.into()))?);
        Ok(1.0 - empty_count_f64 / base_check_count_f64)
    }

    fn serialize(
        &self,
        _: &mut dyn Write,
        _: &mut ValueSerializer<'_, Value>,
    ) -> Result<(), Error> {
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
    use std::{
        fs::File,
        io::{Seek, SeekFrom},
    };

    use tempfile::tempfile;

    use crate::serializer::Serializer;

    use super::*;

    #[rustfmt::skip]
    const SERIALIZED: &[u8] = &[
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
    const SERIALIZED_FIXED_VALUE_SIZE: &[u8] = &[
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
    const SERIALIZED_FIXED_VALUE_SIZE_WITH_HEADER: &[u8] = &[
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
    const SERIALIZED_FIXED_VALUE_SIZE_FOR_CALCULATING_FILLING_RATE: &[u8] = &[
            0x00u8, 0x00u8, 0x00u8, 0x02u8,
            0x00u8, 0x00u8, 0x00u8, 0xFFu8,
            0x00u8, 0x00u8, 0xFEu8, 0x18u8,
            0x00u8, 0x00u8, 0x00u8, 0x00u8,
            0x00u8, 0x00u8, 0x00u8, 0x04u8,
        ];

    #[rustfmt::skip]
    const SERIALIZED_BROKEN: &[u8] = &[
        0x00u8, 0x00u8, 0x00u8, 0x02u8,
        0x00u8, 0x00u8, 0x2Au8, 0xFFu8,
        0x00u8,
    ];

    fn base_check_array_of<Value: 'static>(storage: &dyn Storage<Value>) -> Vec<u32> {
        let size = storage.base_check_size().unwrap();
        let mut array = Vec::<u32>::with_capacity(size);
        for i in 0..size {
            #[allow(clippy::cast_sign_loss)]
            {
                array.push(
                    ((storage.base_at(i).unwrap() as u32) << 8u32)
                        | u32::from(storage.check_at(i).unwrap()),
                );
            }
        }
        array
    }

    fn make_temporary_file(initial_content: &[u8]) -> File {
        let mut file = tempfile().unwrap();
        file.write_all(initial_content).unwrap();
        let _ = file.seek(SeekFrom::Start(0)).unwrap();
        file
    }

    mod mmap_storage {
        use std::io::Cursor;

        use crate::double_array::VACANT_CHECK_VALUE;
        use crate::integer_serializer::IntegerSerializer;

        use super::*;

        fn file_size_of(file: &File) -> usize {
            usize::try_from(file.metadata().unwrap().len()).unwrap()
        }

        #[test]
        fn builder() {
            {
                let file = make_temporary_file(SERIALIZED_FIXED_VALUE_SIZE);
                let file_size = file_size_of(&file);
                let file_mapping = Rc::new(FileMapping::new(file).unwrap());
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
                let file = make_temporary_file(SERIALIZED_FIXED_VALUE_SIZE_WITH_HEADER);
                let file_size = file_size_of(&file);
                let file_mapping = Rc::new(FileMapping::new(file).unwrap());
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
                let file = make_temporary_file(SERIALIZED);
                let file_size = file_size_of(&file);
                let file_mapping = Rc::new(FileMapping::new(file).unwrap());
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
                let file = make_temporary_file(SERIALIZED_BROKEN);
                let file_size = file_size_of(&file);
                let file_mapping = Rc::new(FileMapping::new(file).unwrap());
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
                let file = make_temporary_file(SERIALIZED_FIXED_VALUE_SIZE);
                let file_size = file_size_of(&file);
                let file_mapping = Rc::new(FileMapping::new(file).unwrap());
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
                let file = make_temporary_file(SERIALIZED_FIXED_VALUE_SIZE);
                let file_size = file_size_of(&file);
                let file_mapping = Rc::new(FileMapping::new(file).unwrap());
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
                let file = make_temporary_file(SERIALIZED_FIXED_VALUE_SIZE);
                let file_size = file_size_of(&file);
                let file_mapping = Rc::new(FileMapping::new(file).unwrap());
                let deserializer = ValueDeserializer::<u32>::new(Box::new(|serialized| {
                    static INTEGER_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
                        LazyLock::new(|| IntegerDeserializer::new(false));
                    INTEGER_DESERIALIZER.deserialize(serialized)
                }));
                let storage = MmapStorage::builder(file_mapping, 0, file_size, deserializer)
                    .build()
                    .unwrap();

                assert_eq!(storage.base_check_size().unwrap(), 2);
            }
            {
                let file = make_temporary_file(SERIALIZED_FIXED_VALUE_SIZE_WITH_HEADER);
                let file_size = file_size_of(&file);
                let file_mapping = Rc::new(FileMapping::new(file).unwrap());
                let deserializer = ValueDeserializer::<u32>::new(Box::new(|serialized| {
                    static INTEGER_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
                        LazyLock::new(|| IntegerDeserializer::new(false));
                    INTEGER_DESERIALIZER.deserialize(serialized)
                }));
                let storage = MmapStorage::builder(file_mapping, 5, file_size, deserializer)
                    .build()
                    .unwrap();

                assert_eq!(storage.base_check_size().unwrap(), 2);
            }
        }

        #[test]
        fn base_at() {
            {
                let file = make_temporary_file(SERIALIZED_FIXED_VALUE_SIZE);
                let file_size = file_size_of(&file);
                let file_mapping = Rc::new(FileMapping::new(file).unwrap());
                let deserializer = ValueDeserializer::<u32>::new(Box::new(|serialized| {
                    static INTEGER_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
                        LazyLock::new(|| IntegerDeserializer::new(false));
                    INTEGER_DESERIALIZER.deserialize(serialized)
                }));
                let storage = MmapStorage::builder(file_mapping, 0, file_size, deserializer)
                    .build()
                    .unwrap();

                assert_eq!(storage.base_at(0).unwrap(), 42);
                assert_eq!(storage.base_at(1).unwrap(), 0xFE);
            }
            {
                let file = make_temporary_file(SERIALIZED_FIXED_VALUE_SIZE_WITH_HEADER);
                let file_size = file_size_of(&file);
                let file_mapping = Rc::new(FileMapping::new(file).unwrap());
                let deserializer = ValueDeserializer::<u32>::new(Box::new(|serialized| {
                    static INTEGER_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
                        LazyLock::new(|| IntegerDeserializer::new(false));
                    INTEGER_DESERIALIZER.deserialize(serialized)
                }));
                let storage = MmapStorage::builder(file_mapping, 5, file_size, deserializer)
                    .build()
                    .unwrap();

                assert_eq!(storage.base_at(0).unwrap(), 42);
                assert_eq!(storage.base_at(1).unwrap(), 0xFE);
            }
        }

        #[test]
        #[should_panic]
        fn set_base_at() {
            let file = make_temporary_file(SERIALIZED_FIXED_VALUE_SIZE);
            let file_size = file_size_of(&file);
            let file_mapping = Rc::new(FileMapping::new(file).unwrap());
            let deserializer = ValueDeserializer::<u32>::new(Box::new(|serialized| {
                static INTEGER_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
                    LazyLock::new(|| IntegerDeserializer::new(false));
                INTEGER_DESERIALIZER.deserialize(serialized)
            }));
            let mut storage = MmapStorage::builder(file_mapping, 0, file_size, deserializer)
                .build()
                .unwrap();

            let _result = storage.set_base_at(42, 4242);
        }

        #[test]
        fn check_at() {
            {
                let file = make_temporary_file(SERIALIZED_FIXED_VALUE_SIZE);
                let file_size = file_size_of(&file);
                let file_mapping = Rc::new(FileMapping::new(file).unwrap());
                let deserializer = ValueDeserializer::<u32>::new(Box::new(|serialized| {
                    static INTEGER_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
                        LazyLock::new(|| IntegerDeserializer::new(false));
                    INTEGER_DESERIALIZER.deserialize(serialized)
                }));
                let storage = MmapStorage::builder(file_mapping, 0, file_size, deserializer)
                    .build()
                    .unwrap();

                assert_eq!(storage.check_at(0).unwrap(), VACANT_CHECK_VALUE);
                assert_eq!(storage.check_at(1).unwrap(), 24);
            }
            {
                let file = make_temporary_file(SERIALIZED_FIXED_VALUE_SIZE_WITH_HEADER);
                let file_size = file_size_of(&file);
                let file_mapping = Rc::new(FileMapping::new(file).unwrap());
                let deserializer = ValueDeserializer::<u32>::new(Box::new(|serialized| {
                    static INTEGER_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
                        LazyLock::new(|| IntegerDeserializer::new(false));
                    INTEGER_DESERIALIZER.deserialize(serialized)
                }));
                let storage = MmapStorage::builder(file_mapping, 5, file_size, deserializer)
                    .build()
                    .unwrap();

                assert_eq!(storage.check_at(0).unwrap(), VACANT_CHECK_VALUE);
                assert_eq!(storage.check_at(1).unwrap(), 24);
            }
        }

        #[test]
        #[should_panic]
        fn set_check_at() {
            let file = make_temporary_file(SERIALIZED_FIXED_VALUE_SIZE);
            let file_size = file_size_of(&file);
            let file_mapping = Rc::new(FileMapping::new(file).unwrap());
            let deserializer = ValueDeserializer::<u32>::new(Box::new(|serialized| {
                static INTEGER_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
                    LazyLock::new(|| IntegerDeserializer::new(false));
                INTEGER_DESERIALIZER.deserialize(serialized)
            }));
            let mut storage = MmapStorage::builder(file_mapping, 0, file_size, deserializer)
                .build()
                .unwrap();

            let _result = storage.set_check_at(24, 124);
        }

        #[test]
        fn value_count() {
            {
                let file = make_temporary_file(SERIALIZED_FIXED_VALUE_SIZE);
                let file_size = file_size_of(&file);
                let file_mapping = Rc::new(FileMapping::new(file).unwrap());
                let deserializer = ValueDeserializer::<u32>::new(Box::new(|serialized| {
                    static INTEGER_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
                        LazyLock::new(|| IntegerDeserializer::new(false));
                    INTEGER_DESERIALIZER.deserialize(serialized)
                }));
                let storage = MmapStorage::builder(file_mapping, 0, file_size, deserializer)
                    .build()
                    .unwrap();

                assert_eq!(storage.value_count().unwrap(), 5);
            }
            {
                let file = make_temporary_file(SERIALIZED_FIXED_VALUE_SIZE_WITH_HEADER);
                let file_size = file_size_of(&file);
                let file_mapping = Rc::new(FileMapping::new(file).unwrap());
                let deserializer = ValueDeserializer::<u32>::new(Box::new(|serialized| {
                    static INTEGER_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
                        LazyLock::new(|| IntegerDeserializer::new(false));
                    INTEGER_DESERIALIZER.deserialize(serialized)
                }));
                let storage = MmapStorage::builder(file_mapping, 5, file_size, deserializer)
                    .build()
                    .unwrap();

                assert_eq!(storage.value_count().unwrap(), 5);
            }
        }

        #[test]
        fn value_at() {
            {
                let file = make_temporary_file(SERIALIZED_FIXED_VALUE_SIZE);
                let file_size = file_size_of(&file);
                let file_mapping = Rc::new(FileMapping::new(file).unwrap());
                let deserializer = ValueDeserializer::<u32>::new(Box::new(|serialized| {
                    static INTEGER_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
                        LazyLock::new(|| IntegerDeserializer::new(false));
                    INTEGER_DESERIALIZER.deserialize(serialized)
                }));
                let storage = MmapStorage::builder(file_mapping, 0, file_size, deserializer)
                    .build()
                    .unwrap();

                assert!(storage.value_at(0).unwrap().is_none());
                assert_eq!(*storage.value_at(1).unwrap().unwrap(), 159);
                assert_eq!(*storage.value_at(2).unwrap().unwrap(), 14);
                assert!(storage.value_at(3).unwrap().is_none());
                assert_eq!(*storage.value_at(4).unwrap().unwrap(), 3);
            }
            {
                let file = make_temporary_file(SERIALIZED_FIXED_VALUE_SIZE_WITH_HEADER);
                let file_size = file_size_of(&file);
                let file_mapping = Rc::new(FileMapping::new(file).unwrap());
                let deserializer = ValueDeserializer::<u32>::new(Box::new(|serialized| {
                    static INTEGER_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
                        LazyLock::new(|| IntegerDeserializer::new(false));
                    INTEGER_DESERIALIZER.deserialize(serialized)
                }));
                let storage = MmapStorage::builder(file_mapping, 5, file_size, deserializer)
                    .build()
                    .unwrap();

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
            let file = make_temporary_file(SERIALIZED_FIXED_VALUE_SIZE);
            let file_size = file_size_of(&file);
            let file_mapping = Rc::new(FileMapping::new(file).unwrap());
            let deserializer = ValueDeserializer::<u32>::new(Box::new(|serialized| {
                static INTEGER_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
                    LazyLock::new(|| IntegerDeserializer::new(false));
                INTEGER_DESERIALIZER.deserialize(serialized)
            }));
            let mut storage = MmapStorage::builder(file_mapping, 0, file_size, deserializer)
                .build()
                .unwrap();

            let _result = storage.add_value_at(24, 124);
        }

        #[test]
        fn filling_rate() {
            let file =
                make_temporary_file(SERIALIZED_FIXED_VALUE_SIZE_FOR_CALCULATING_FILLING_RATE);
            let file_size = file_size_of(&file);
            let file_mapping = Rc::new(FileMapping::new(file).unwrap());
            let deserializer = ValueDeserializer::<u32>::new(Box::new(|serialized| {
                static INTEGER_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
                    LazyLock::new(|| IntegerDeserializer::new(false));
                INTEGER_DESERIALIZER.deserialize(serialized)
            }));
            let storage = MmapStorage::builder(file_mapping, 0, file_size, deserializer)
                .build()
                .unwrap();

            assert!((storage.filling_rate().unwrap() - 1.0 / 2.0).abs() < 0.1);
        }

        #[test]
        #[should_panic]
        fn serialize() {
            let file = make_temporary_file(SERIALIZED_FIXED_VALUE_SIZE);
            let file_size = file_size_of(&file);
            let file_mapping = Rc::new(FileMapping::new(file).unwrap());
            let deserializer = ValueDeserializer::<u32>::new(Box::new(|serialized| {
                static INTEGER_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
                    LazyLock::new(|| IntegerDeserializer::new(false));
                INTEGER_DESERIALIZER.deserialize(serialized)
            }));
            let storage = MmapStorage::builder(file_mapping, 0, file_size, deserializer)
                .build()
                .unwrap();

            let mut writer = Cursor::new(Vec::new());
            let mut serializer = ValueSerializer::<u32>::new(
                Box::new(|value| {
                    static INTEGER_SERIALIZER: LazyLock<IntegerSerializer<u32>> =
                        LazyLock::new(|| IntegerSerializer::new(false));
                    Ok(INTEGER_SERIALIZER.serialize(value))
                }),
                size_of::<u32>(),
            );

            let _result = storage.serialize(&mut writer, &mut serializer);
        }

        #[test]
        fn clone() {
            {
                let file = make_temporary_file(SERIALIZED_FIXED_VALUE_SIZE);
                let file_size = file_size_of(&file);
                let file_mapping = Rc::new(FileMapping::new(file).unwrap());
                let deserializer = ValueDeserializer::<u32>::new(Box::new(|serialized| {
                    static INTEGER_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
                        LazyLock::new(|| IntegerDeserializer::new(false));
                    INTEGER_DESERIALIZER.deserialize(serialized)
                }));
                let storage = MmapStorage::builder(file_mapping, 0, file_size, deserializer)
                    .build()
                    .unwrap();

                let clone = storage.clone_box();
                assert_eq!(
                    base_check_array_of(clone.as_ref()),
                    base_check_array_of(&storage)
                );
                assert_eq!(clone.value_count().unwrap(), storage.value_count().unwrap());
            }
            {
                let file = make_temporary_file(SERIALIZED_FIXED_VALUE_SIZE_WITH_HEADER);
                let file_size = file_size_of(&file);
                let file_mapping = Rc::new(FileMapping::new(file).unwrap());
                let deserializer = ValueDeserializer::<u32>::new(Box::new(|serialized| {
                    static INTEGER_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
                        LazyLock::new(|| IntegerDeserializer::new(false));
                    INTEGER_DESERIALIZER.deserialize(serialized)
                }));
                let storage = MmapStorage::builder(file_mapping, 5, file_size, deserializer)
                    .build()
                    .unwrap();

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
                make_temporary_file(SERIALIZED_FIXED_VALUE_SIZE_FOR_CALCULATING_FILLING_RATE);
            let file_size = file_size_of(&file);
            let file_mapping = Rc::new(FileMapping::new(file).unwrap());
            let deserializer = ValueDeserializer::<u32>::new(Box::new(|serialized| {
                static INTEGER_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
                    LazyLock::new(|| IntegerDeserializer::new(false));
                INTEGER_DESERIALIZER.deserialize(serialized)
            }));
            let storage = MmapStorage::builder(file_mapping, 0, file_size, deserializer)
                .build()
                .unwrap();

            let _ = storage.as_any();
        }

        #[test]
        fn as_any_mut() {
            let file =
                make_temporary_file(SERIALIZED_FIXED_VALUE_SIZE_FOR_CALCULATING_FILLING_RATE);
            let file_size = file_size_of(&file);
            let file_mapping = Rc::new(FileMapping::new(file).unwrap());
            let deserializer = ValueDeserializer::<u32>::new(Box::new(|serialized| {
                static INTEGER_DESERIALIZER: LazyLock<IntegerDeserializer<u32>> =
                    LazyLock::new(|| IntegerDeserializer::new(false));
                INTEGER_DESERIALIZER.deserialize(serialized)
            }));
            let mut storage = MmapStorage::builder(file_mapping, 0, file_size, deserializer)
                .build()
                .unwrap();

            let _ = storage.as_any_mut();
        }
    }
}
