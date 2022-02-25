use std::fmt::Display;

use serde::Deserialize;

use crate::error::Parse;

#[allow(missing_docs)]
#[derive(
    Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[serde(try_from = "String")]
pub enum OrderStatus {
    #[allow(missing_docs)]
    Canceled,
    #[allow(missing_docs)]
    Filled,
    #[allow(missing_docs)]
    Untouched,
}

impl Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            OrderStatus::Canceled => "Canceled",
            OrderStatus::Filled => "Filled",
            OrderStatus::Untouched => "Untouched",
        })
    }
}

impl TryFrom<String> for OrderStatus {
    type Error = Parse;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_ref() {
            "canceled" | "Canceled" | "CANCELED" => Ok(Self::Canceled),
            "filled" | "Filled" | "FILLED" => Ok(Self::Filled),
            "untouched" | "Untouched" | "UNTOUCHED" => Ok(Self::Untouched),
            other => Err(Parse::new(other, "&str", "OrderStatus")),
        }
    }
}
