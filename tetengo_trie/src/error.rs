/*!
 * An error.
 */

/**
 * An error.
 */
#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    /**
     * An invalid serailzed bytes.
     */
    #[error("invalid serialized bytes: {0}")]
    InvalidSerializedBytes(String),
}
