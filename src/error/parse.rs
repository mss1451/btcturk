use std::fmt::Debug;
use thiserror::Error;

/// Occurs when there is a parsing error.
#[derive(Error, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[error(
    "content `{source_content}` with type `{source_type}` couldn't be \
parsed as type `{destination_type}`"
)]
pub struct Parse {
    source_content: String,
    source_type: String,
    destination_type: String,
}

impl Parse {
    /// Constructs a new error with string representations of both source and
    /// destination.
    #[must_use]
    pub fn new(
        source_content: impl Into<String>,
        source_type: impl Into<String>,
        destination_type: impl Into<String>,
    ) -> Self {
        Self {
            source_content: source_content.into(),
            source_type: source_type.into(),
            destination_type: destination_type.into(),
        }
    }
    /// Get a reference to the source content.
    #[must_use]
    pub fn source_content(&self) -> &str {
        self.source_content.as_ref()
    }

    /// Get a reference to the source type.
    #[must_use]
    pub fn source_type(&self) -> &str {
        self.source_type.as_ref()
    }

    /// Get a reference to the destination type.
    #[must_use]
    pub fn destination_type(&self) -> &str {
        self.destination_type.as_ref()
    }
}
