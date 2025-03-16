/*!
 * An error.
 */

use std::io;

type Type = io::Error;

/**
 * An error.
 */
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /**
     * An invalid serailzed bytes.
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
    IoError(#[from] Type),
}
