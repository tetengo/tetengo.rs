/*!
 * An error.
 */

/**
 * An error.
 */
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /**
     * Range out of bounds.
     */
    #[error("range out of bounds")]
    RangeOutOfBounds,

    /**
     * Mismatch concrete type.
     */
    #[error("mismatch concrete type")]
    MismatchConcreteType,

    /**
     * An I/O error.
     */
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
}
