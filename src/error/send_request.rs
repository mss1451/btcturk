use std::time::SystemTimeError;

use surf::StatusCode;
use thiserror::Error;

use super::{Parameter, Response};

/// Occurs when there is an error sending a request.
#[derive(Error, Debug)]
pub enum SendRequest {
    /// Endpoint requires authentication.
    #[error("endpoint requires authentication")]
    AuthenticationRequired,
    /// Received a status code other than 200 OK.
    #[error("received a status code `{status_code}` which is not 200 OK with \
    response `{response_string}` with code `{code:?}` and message `{message:?}`")]
    BadStatusCode {
        /// HTTP status code.
        status_code: StatusCode,
        /// JSON string of the response.
        response_string: String,
        /// Deserialized response's code, if any.
        code: Option<i64>,
        /// Deserialized response's message, if any.
        message: Option<String>,
    },
    /// System time error occurred.
    #[error(transparent)]
    SystemTimeError {
        /// Source of the error.
        #[from]
        source: SystemTimeError,
    },
    /// Surf error occurred.
    #[error(
        "surf error occurred. status code: `{status_code:?}`,
    type name: `{type_name:?}`"
    )]
    SurfError {
        /// Source of the error.
        #[source]
        source: anyhow::Error,
        /// HTTP status code associated with the error.
        status_code: StatusCode,
        /// Type name of the error, if any.
        type_name: Option<String>,
    },
    /// Serde JSON error occurred.
    #[error(transparent)]
    SerdeJsonError {
        /// Source of the error.
        #[from]
        source: serde_json::Error,
    },
    /// Response error occurred.
    #[error(transparent)]
    ResponseError {
        /// Source of the error.
        #[from]
        source: Response,
    },
    /// Parameter error occurred.
    #[error(transparent)]
    ParameterError {
        /// Source of the error.
        #[from]
        source: Parameter,
    },
}

impl From<surf::Error> for SendRequest {
    fn from(error: surf::Error) -> Self {
        Self::SurfError {
            status_code: error.status(),
            type_name: error.type_name().map(ToOwned::to_owned),
            source: error.into_inner(),
        }
    }
}
