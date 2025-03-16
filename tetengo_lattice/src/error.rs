/*!
 * An error.
 */

/**
 * An error.
 */
#[derive(Debug, thiserror::Error)]
pub enum Error {
    // /**
    //  * An invalid density factor.
    //  */
    // #[error("invalid density factor")]
    // InvalidDensityFactor,
    /**
     * An I/O error.
     */
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
}
