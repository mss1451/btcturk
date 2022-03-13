use base64::DecodeError;
use crypto_common::InvalidLength;
use thiserror::Error;

/// Occurs when there is an error with the private API key.
#[derive(Error, Debug, Clone)]
pub enum PrivateKey {
    /// Invalid length error occurred.
    #[error(transparent)]
    InvalidLengthError {
        /// Source of the error.
        #[from]
        source: InvalidLength,
    },
    /// Decode error occurred.
    #[error(transparent)]
    DecodeError {
        /// Source of the error.
        #[from]
        source: DecodeError,
    },
}
