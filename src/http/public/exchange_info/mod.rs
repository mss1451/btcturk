//! Implementation of the exchange info endpoint.

use rust_decimal::Decimal;
use serde::Deserialize;
use surf::http::Method;

use crate::{
    error::{Parse, SendRequest},
    http::{request::Parameters, Client, OrderMethod, Request},
};

impl Client<'_> {
    /// Gets a list of all known currencies.
    /// You can use this endpoint to get all tradable pairs and their quantity
    /// or price scales.
    /// # Errors
    /// [`SendRequest`] if there is an error sending the request or there
    /// is an error or a malformation in the received response.
    ///
    /// See also <https://docs.btcturk.com/public-endpoints/exchange-info>.
    pub async fn exchange_info(&self) -> Result<ExchangeInfo, SendRequest> {
        self.send(
            Request {
                endpoint: self.url_cache().exchange_info(),
                method: Method::Get,
                parameters: Parameters::new(),
                requires_auth: false,
            },
            false,
        )
        .await
    }
}

/// **Sample**:
/// ```json
#[doc = include_str!("sample.json")]
/// ```
/// See also <https://docs.btcturk.com/public-endpoints/exchange-info>
#[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInfo {
    #[allow(missing_docs)]
    #[serde(rename = "timeZone")]
    pub timezone: String,
    #[allow(missing_docs)]
    pub server_time: u64,
    #[allow(missing_docs)]
    pub symbols: Vec<Symbol>,
    #[allow(missing_docs)]
    pub currencies: Vec<Currency>,
    #[allow(missing_docs)]
    pub currency_operation_blocks: Vec<CurrencyOperationBlock>,
}

#[allow(clippy::struct_excessive_bools)]
#[allow(missing_docs)]
#[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    #[allow(missing_docs)]
    pub id: i64,
    #[allow(missing_docs)]
    pub name: String,
    #[allow(missing_docs)]
    pub name_normalized: String,
    #[allow(missing_docs)]
    pub status: String,
    #[allow(missing_docs)]
    pub numerator: String,
    #[allow(missing_docs)]
    pub denominator: String,
    #[allow(missing_docs)]
    pub numerator_scale: u64,
    #[allow(missing_docs)]
    pub denominator_scale: u64,
    #[allow(missing_docs)]
    pub has_fraction: bool,
    #[allow(missing_docs)]
    pub filters: Vec<Filter>,
    #[allow(missing_docs)]
    pub order_methods: Vec<OrderMethod>,
    #[allow(missing_docs)]
    pub display_format: String,
    #[allow(missing_docs)]
    pub commission_from_numerator: bool,
    #[allow(missing_docs)]
    pub order: i64,
    #[allow(missing_docs)]
    pub price_rounding: bool,
    #[allow(missing_docs)]
    pub is_new: bool,
    #[allow(missing_docs)]
    pub market_price_warning_threshold_percentage: Decimal,
    #[allow(missing_docs)]
    pub maximum_order_amount: Option<Decimal>,
}

#[allow(missing_docs)]
#[derive(
    Deserialize, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[serde(tag = "filterType")]
pub enum Filter {
    #[serde(rename = "PRICE_FILTER")]
    #[serde(rename_all = "camelCase")]
    PriceFilter {
        #[allow(missing_docs)]
        min_price: Decimal,
        #[allow(missing_docs)]
        max_price: Decimal,
        #[allow(missing_docs)]
        tick_size: Decimal,
        #[allow(missing_docs)]
        min_exchange_value: Decimal,
        #[allow(missing_docs)]
        min_amount: Option<Decimal>,
        #[allow(missing_docs)]
        max_amount: Option<Decimal>,
    },
}

#[allow(clippy::struct_excessive_bools)]
#[allow(missing_docs)]
#[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Currency {
    #[allow(missing_docs)]
    pub id: i64,
    #[allow(missing_docs)]
    pub symbol: String,
    #[allow(missing_docs)]
    pub min_withdrawal: Decimal,
    #[allow(missing_docs)]
    pub min_deposit: Decimal,
    #[allow(missing_docs)]
    pub precision: u64,
    #[allow(missing_docs)]
    pub address: Address,
    #[allow(missing_docs)]
    pub currency_type: CurrencyType,
    #[allow(missing_docs)]
    pub tag: Tag,
    #[allow(missing_docs)]
    pub color: String,
    #[allow(missing_docs)]
    pub name: String,
    #[allow(missing_docs)]
    pub is_address_renewable: bool,
    #[allow(missing_docs)]
    pub get_auto_address_disabled: bool,
    #[allow(missing_docs)]
    pub is_partial_withdrawal_enabled: bool,
    #[allow(missing_docs)]
    pub is_new: bool,
}

#[allow(missing_docs)]
#[derive(
    Deserialize, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    #[allow(missing_docs)]
    pub min_len: Option<u64>,
    #[allow(missing_docs)]
    pub max_len: Option<u64>,
}

#[allow(missing_docs)]
#[derive(
    Deserialize, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[serde(rename_all = "camelCase")]
#[serde(try_from = "String")]
pub enum CurrencyType {
    Crypto,
    Fiat,
}

impl TryFrom<String> for CurrencyType {
    type Error = Parse;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_ref() {
            "crypto" | "Crypto" | "CRYPTO" => Ok(Self::Crypto),
            "fiat" | "Fiat" | "FIAT" => Ok(Self::Fiat),
            other => Err(Parse::new(other, "&str", "CurrencyType")),
        }
    }
}

#[allow(missing_docs)]
#[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    #[allow(missing_docs)]
    pub enable: bool,
    #[allow(missing_docs)]
    pub name: Option<String>,
    #[allow(missing_docs)]
    pub min_len: Option<u64>,
    #[allow(missing_docs)]
    pub max_len: Option<u64>,
}

#[allow(missing_docs)]
#[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub struct CurrencyOperationBlock {
    #[allow(missing_docs)]
    pub currency_symbol: String,
    #[allow(missing_docs)]
    pub withdrawal_disabled: bool,
    #[allow(missing_docs)]
    pub deposit_disabled: bool,
}

#[cfg(test)]
mod tests {
    use crate::http::Client;

    use super::{ExchangeInfo, Filter};

    #[ignore]
    #[async_std::test]
    async fn get_exchange_info() {
        let _ = env_logger::builder().is_test(true).try_init();

        let exchange_info = Client::new(None, None)
            .unwrap()
            .exchange_info()
            .await
            .unwrap();
        for filter in
            exchange_info.symbols.iter().flat_map(|s| s.filters.iter())
        {
            match filter {
                Filter::PriceFilter {
                    min_price,
                    max_price,
                    tick_size: _,
                    min_exchange_value: _,
                    min_amount: _,
                    max_amount: _,
                } => {
                    assert!(max_price >= min_price);
                }
            }
        }
    }

    #[test]
    fn deserialize_exchange_info() {
        let json_string = include_str!("sample.json");
        serde_json::from_str::<ExchangeInfo>(json_string).unwrap();
    }
}
