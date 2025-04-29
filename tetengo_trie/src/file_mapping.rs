/*!
 * A file mapping.
 */

use std::fs::File;
use std::ops::Range;

use memmap2::Mmap;

use crate::error::Error;

/**
 * A file mapping.
 */
#[derive(Debug)]
pub struct FileMapping {
    file: File,
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
    pub fn new(file: File) -> Result<Self, Error> {
        let mmap = unsafe { Mmap::map(&file).map_err(|e| Error::InternalError(e.into()))? };
        Ok(Self { file, mmap })
    }

    /**
     * Returns the file.
     *
     * # Returns
     * The file.
     */
    pub fn file(&self) -> &File {
        &self.file
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
    pub fn region(&self, range: Range<usize>) -> Result<&[u8], Error> {
        self.mmap.get(range).ok_or(Error::OutOfMmap)
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Seek, SeekFrom, Write};

    use tempfile::tempfile;

    use super::*;

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

    fn make_temporary_file(initial_content: &[u8]) -> File {
        let mut file = tempfile().unwrap();
        file.write_all(initial_content).unwrap();
        let _ = file.seek(SeekFrom::Start(0)).unwrap();
        file
    }

    #[test]
    fn new() {
        let file = make_temporary_file(SERIALIZED_FIXED_VALUE_SIZE);
        let file_mapping = FileMapping::new(file);
        assert!(file_mapping.is_ok());
    }

    #[test]
    fn file() {
        let file = make_temporary_file(SERIALIZED_FIXED_VALUE_SIZE);
        let file_mapping = FileMapping::new(file).unwrap();

        assert_eq!(
            file_mapping.file().metadata().unwrap().len(),
            SERIALIZED_FIXED_VALUE_SIZE.len() as u64
        );
    }

    #[test]
    fn size() {
        let file = make_temporary_file(SERIALIZED_FIXED_VALUE_SIZE);
        let file_mapping = FileMapping::new(file).unwrap();

        assert_eq!(file_mapping.size(), SERIALIZED_FIXED_VALUE_SIZE.len());
    }

    #[test]
    fn region() {
        let file = make_temporary_file(SERIALIZED_FIXED_VALUE_SIZE);
        let file_mapping = FileMapping::new(file).unwrap();

        {
            let region = file_mapping.region(3..24).unwrap();
            assert_eq!(region, &SERIALIZED_FIXED_VALUE_SIZE[3..24]);
        }
        {
            let region = file_mapping.region(0..file_mapping.size()).unwrap();
            assert_eq!(region, SERIALIZED_FIXED_VALUE_SIZE);
        }
        {
            let region = file_mapping.region(0..file_mapping.size() + 1);
            assert!(region.is_err());
        }
    }
}
