use base64::DecodeError;
use crypto_common::InvalidLength;
use thiserror::Error;

/// Occurs when there is an error sending a request.
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
