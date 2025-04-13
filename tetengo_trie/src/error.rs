/*!
 * An error.
 */

use std::io;

/**
 * An error.
 */
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /**
     * Out of mmap.
     */
    #[error("out of mmap")]
    OutOfMmap,

    /**
     * An invalid serialized bytes.
     */
    #[error("invalid serialized bytes: {0}")]
    InvalidSerializedBytes(String),

    /**
     * An invalid density factor.
     */
    #[error("invalid density factor")]
    InvalidDensityFactor,

    /**
     * An I/O error.
     */
    #[error("io error: {0}")]
    IoError(#[from] io::Error),
}
