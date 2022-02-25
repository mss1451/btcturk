use std::fmt::Debug;
use thiserror::Error;

/// Occurs when one of the request parameters is invalid.
#[derive(Error, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[error("invalid parameter `{name}` = `{value}`")]
pub struct Parameter {
    name: &'static str,
    value: String,
}

impl Parameter {
    /// Constructs a new error with textual representations of
    /// both parameter and value.
    #[must_use]
    pub const fn new(name: &'static str, value: String) -> Self {
        Self { name, value }
    }
    /// Get a reference to the name.
    #[must_use]
    pub const fn name(&self) -> &str {
        self.name
    }

    /// Get a reference to the value.
    #[must_use]
    pub fn value(&self) -> &str {
        self.value.as_ref()
    }
}
