use std::fmt::Display;

use serde::Deserialize;

use crate::error::Parse;

#[allow(missing_docs)]
#[derive(
    Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[serde(try_from = "String")]
pub enum OrderMethod {
    #[allow(missing_docs)]
    Market,
    #[allow(missing_docs)]
    Limit,
    #[allow(missing_docs)]
    StopLimit,
    #[allow(missing_docs)]
    StopMarket,
}

impl Display for OrderMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            OrderMethod::Market => "market",
            OrderMethod::Limit => "limit",
            OrderMethod::StopLimit => "stoplimit",
            OrderMethod::StopMarket => "stopmarket",
        })
    }
}

impl From<OrderMethod> for String {
    fn from(value: OrderMethod) -> Self {
        value.to_string()
    }
}

impl TryFrom<String> for OrderMethod {
    type Error = Parse;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_ref() {
            "market" | "Market" | "MARKET" => Ok(Self::Market),
            "limit" | "Limit" | "LIMIT" => Ok(Self::Limit),
            "stoplimit" | "stopLimit" | "StopLimit" | "STOP_LIMIT" => {
                Ok(Self::StopLimit)
            }
            "stopmarket" | "stopMarket" | "StopMarket" | "STOP_MARKET" => {
                Ok(Self::StopMarket)
            }
            other => Err(Parse::new(other, "&str", "OrderMethod")),
        }
    }
}
