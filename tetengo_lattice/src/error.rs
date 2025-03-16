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
     * A BOS or EOS entry is not allowed.
     */
    #[error("BOS or EOS entry is not allowed")]
    BosOrEosEntryNotAllowed,

    /**
     * The step is too large.
     */
    #[error("the step is too large")]
    StepIsTooLarge,

    /**
     * No node is found for the input.
     */
    #[error("no node is found for the input")]
    NoNodeIsFoundForTheInput,

    /**
     * No input.
     */
    #[error("no input")]
    NoInput,
    /**
     * An I/O error.
     */
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
}
