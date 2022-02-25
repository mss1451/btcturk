use std::fmt::Display;

use serde::Deserialize;

use crate::error::Parse;

#[allow(missing_docs)]
#[derive(
    Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[serde(try_from = "String")]
pub enum OrderType {
    #[allow(missing_docs)]
    Buy,
    #[allow(missing_docs)]
    Sell,
}

impl Display for OrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            OrderType::Buy => "buy",
            OrderType::Sell => "sell",
        })
    }
}

impl From<OrderType> for String {
    fn from(value: OrderType) -> Self {
        value.to_string()
    }
}

impl TryFrom<String> for OrderType {
    type Error = Parse;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_ref() {
            "buy" | "Buy" | "BUY" => Ok(Self::Buy),
            "sell" | "Sell" | "SELL" => Ok(Self::Sell),
            other => Err(Parse::new(other, "&str", "OrderType")),
        }
    }
}
