use thiserror::Error;

/// Occurs when there is an error in the received response such as
/// server-side error value or empty data field.
///
/// JSON parsing error is not included.
#[derive(Error, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Response {
    /// False `success` field.
    #[error("`false `success` field. code: '{code}'. message: {message:?}")]
    Unsuccessful {
        /// `message` field of the response.
        message: Option<String>,
        /// `code` field of the response.
        code: i64,
    },
    /// Null `data` field
    #[error("null `data` field")]
    NullData,
    /// Empty `data` field
    #[error("empty `data` field")]
    EmptyData,
}
