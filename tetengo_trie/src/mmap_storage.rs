/*!
 * An mmap storage.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

use std::fs::File;

use tempfile as _;

use crate::storage::Result;
use crate::value_serializer::ValueDeserializer;

/**
 * An mmap storage.
 *
 * # Type Parameters
 * * `T` - A value type.
 */
#[derive(Debug)]
pub struct MmapStorage<T> {
    _file: File,
    _content_offset: usize,
    _file_size: usize,
    _value_deserializer: ValueDeserializer<T>,
    _value_cache_capacity: usize,
}

impl<T> MmapStorage<T> {
    /**
     * Creates an mmap storage.
     *
     * # Arguments
     * * `file`                 - A file.
     * * `content_offset`       - A content offset in the file.
     * * `file_size`            - The file size.
     * * `value_deserializer`   - A deserializer for value objects.
     * * `value_cache_capacity` - A value cache capacity.
     *
     * # Returns
     * An mmap storage.
     *
     * # Errors
     * * `std::io::Error`   - If fails to read the file.
     * * `DeserializeError` - If fails to deserialize a value.
     * * `StorageError`     - If `content_offset` is greater than `file_size`, or the value size is not fixed.
     */
    pub fn new(
        file: File,
        content_offset: usize,
        file_size: usize,
        value_deserializer: ValueDeserializer<T>,
        value_cache_capacity: usize,
    ) -> Result<Self> {
        Ok(Self {
            _file: file,
            _content_offset: content_offset,
            _file_size: file_size,
            _value_deserializer: value_deserializer,
            _value_cache_capacity: value_cache_capacity,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Seek, SeekFrom, Write};

    use once_cell::sync::Lazy;
    use tempfile;

    use crate::integer_serializer::IntegerDeserializer;
    use crate::serializer::Deserializer;
    use crate::value_serializer::ValueDeserializer;

    use super::*;

    const SERIALIZED__FIXED_VALUE_SIZE: [u8; 40] = [
        0x00u8, 0x00u8, 0x00u8, 0x02u8, 0x00u8, 0x00u8, 0x2Au8, 0xFFu8, 0x00u8, 0x00u8, 0xFEu8,
        0x18u8, 0x00u8, 0x00u8, 0x00u8, 0x05u8, 0x00u8, 0x00u8, 0x00u8, 0x04u8, 0xFFu8, 0xFFu8,
        0xFFu8, 0xFFu8, 0x00u8, 0x00u8, 0x00u8, 0x9Fu8, 0x00u8, 0x00u8, 0x00u8, 0x0Eu8, 0xFFu8,
        0xFFu8, 0xFFu8, 0xFFu8, 0x00u8, 0x00u8, 0x00u8, 0x03u8,
    ];

    fn make_temporary_path(initial_content: &[u8]) -> File {
        let mut file = tempfile::tempfile().expect("Can't create a temporary file.");
        file.write_all(initial_content)
            .expect("Can't write to the temporary file.");
        let _ = file
            .seek(SeekFrom::Start(0))
            .expect("Can't seek the temporary file.");
        file
    }

    fn size_of(file: &File) -> usize {
        file.metadata().expect("Can't get the file size.").len() as usize
    }

    #[test]
    fn new() {
        {
            let file = make_temporary_path(&SERIALIZED__FIXED_VALUE_SIZE);
            let file_size = size_of(&file);
            let deserializer = ValueDeserializer::<u32>::new(|serialized| {
                static INTEGER_DESERIALIZER: Lazy<IntegerDeserializer<u32>> =
                    Lazy::new(|| IntegerDeserializer::new(false));
                INTEGER_DESERIALIZER.deserialize(serialized)
            });
            let _storage = MmapStorage::new(file, 0, file_size, deserializer, 10000);
        }
    }
}
