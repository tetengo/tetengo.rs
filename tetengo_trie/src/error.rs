/*!
 * An error.
 */

use std::error;

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
     * The content offset is larger than the file size.
     */
    #[error("content offset is larger than the file size")]
    LargerContentOffsetThanFileSize,

    /**
     * The fixed value size is 0.
     */
    #[error("fixed value size is 0")]
    ZeroFixedValueSize,

    /**
     * An unexpected EOF.
     */
    #[error("unexpected EOF")]
    UnexpectedEof,

    /**
     * An error returned from an internal crate.
     */
    #[error("internal error: {0}")]
    InternalError(#[from] Box<dyn error::Error>),
}
