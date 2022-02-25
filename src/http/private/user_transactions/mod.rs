//! Implementation of the user transaction endpoints.

use rust_decimal::Decimal;
use serde::Deserialize;
use std::{fmt::Display, ops::Range};
use surf::http::Method;

use crate::{
    error::{self, SendRequest},
    http::{request::Parameters, OrderType, Request},
    Client,
};

#[allow(missing_docs)]
#[derive(
    Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[serde(try_from = "String")]
pub enum TransactionType {
    #[allow(missing_docs)]
    Deposit,
    #[allow(missing_docs)]
    Withdrawal,
}

impl Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            TransactionType::Deposit => "deposit",
            TransactionType::Withdrawal => "withdrawal",
        })
    }
}

impl From<TransactionType> for String {
    fn from(value: TransactionType) -> Self {
        value.to_string()
    }
}

impl TryFrom<String> for TransactionType {
    type Error = error::Parse;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_ref() {
            "deposit" => Ok(Self::Deposit),
            "withdrawal" => Ok(Self::Withdrawal),
            other => Err(error::Parse::new(other, "&str", "TransactionType")),
        }
    }
}

impl Client<'_> {
    /// Get all user trade transactions.
    /// # Errors
    /// [`SendRequest`] if there is an error sending the request or there
    /// is an error or a malformation in the received response.
    /// # Parameters
    /// - `order_id`: If you use this parameter, you can't use the other
    /// parameters (they will be ignored).
    /// - `type`: Type of the trade (`buy` or `sell`). Defaults to both.
    /// - `symbols`: Array of `btc`, `try`, etc. Can be empty.
    /// - `date_range`: Start-end date timestamp range. Defaults to last 30
    /// days.
    ///
    /// See also <https://docs.btcturk.com/private-endpoints/user-transactions>.
    pub async fn trade_transactions(
        &self,
        order_id: Option<i64>,
        r#type: Option<OrderType>,
        symbols: Vec<impl Into<String> + Send>,
        date_range: Option<Range<u64>>,
    ) -> Result<Vec<TradeTransaction>, SendRequest> {
        let mut parameters = Parameters::new();
        if let Some(id) = order_id {
            parameters.push_number("orderId", Some(id));
        } else {
            parameters.push_object("type", r#type);
            for symbol in symbols {
                parameters.push_string("symbol", Some(symbol.into()));
            }
            if let Some(range) = date_range {
                parameters.push_number("startDate", Some(range.start));
                parameters.push_number("endDate", Some(range.end));
            }
        }
        self.send(
            Request {
                endpoint: self.url_cache().trade_transactions(),
                method: Method::Get,
                parameters,
                requires_auth: true,
            },
            false,
        )
        .await
    }

    async fn normal_transactions<T>(
        &self,
        r#type: Option<TransactionType>,
        symbols: Vec<impl Into<String> + Send>,
        date_range: Option<Range<u64>>,
        fiat: bool,
    ) -> Result<T, SendRequest>
    where
        for<'de> T: Deserialize<'de>,
    {
        let mut parameters = Parameters::new();
        parameters.push_object("type", r#type);

        for symbol in symbols {
            parameters.push_string("symbol", Some(symbol.into()));
        }
        if let Some(range) = date_range {
            parameters.push_number("startDate", Some(range.start));
            parameters.push_number("endDate", Some(range.end));
        }
        let endpoint = if fiat {
            self.url_cache().fiat_transactions()
        } else {
            self.url_cache().crypto_transactions()
        };
        self.send(
            Request {
                endpoint,
                method: Method::Get,
                parameters,
                requires_auth: true,
            },
            false,
        )
        .await
    }

    /// Get all user crypto transactions.
    /// # Errors
    /// [`SendRequest`] if there is an error sending the request or there
    /// is an error or a malformation in the received response.
    /// # Parameters
    /// - `type`: Type of the transaction (`deposit` or `withdrawal`). Defaults
    /// to both.
    /// - `symbols`: Array of `btc`, `eth`, etc. Can be empty.
    /// - `date_range`: Start-end date timestamp range. Defaults to last 30
    /// days.
    ///
    /// See also <https://docs.btcturk.com/private-endpoints/user-transactions>.
    pub async fn crypto_transactions(
        &self,
        r#type: Option<TransactionType>,
        symbols: Vec<impl Into<String> + Send>,
        date_range: Option<Range<u64>>,
    ) -> Result<Vec<CryptoTransaction>, SendRequest> {
        self.normal_transactions(r#type, symbols, date_range, false)
            .await
    }

    /// Get all user fiat transactions.
    /// # Errors
    /// [`SendRequest`] if there is an error sending the request or there
    /// is an error or a malformation in the received response.
    /// # Parameters
    /// - `type`: Type of the transaction (`deposit` or `withdrawal`). Defaults
    /// to both.
    /// - `symbols`: Array of `try`, etc. Can be empty.
    /// - `date_range`: Start-end date timestamp range. Defaults to last 30
    /// days.
    ///
    /// See also <https://docs.btcturk.com/private-endpoints/user-transactions>.
    pub async fn fiat_transactions(
        &self,
        r#type: Option<TransactionType>,
        symbols: Vec<impl Into<String> + Send>,
        date_range: Option<Range<u64>>,
    ) -> Result<Vec<FiatTransaction>, SendRequest> {
        self.normal_transactions(r#type, symbols, date_range, true)
            .await
    }
}

/// **Sample**:
/// ```json
#[doc = include_str!("trade_sample.json")]
/// ```
/// See also <https://docs.btcturk.com/private-endpoints/user-transactions>
#[derive(
    serde::Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[serde(rename_all = "camelCase")]
pub struct TradeTransaction {
    #[allow(missing_docs)]
    pub price: Decimal,
    #[allow(missing_docs)]
    pub numerator_symbol: String,
    #[allow(missing_docs)]
    pub denominator_symbol: String,
    #[allow(missing_docs)]
    pub order_type: OrderType,
    #[allow(missing_docs)]
    pub order_id: i64,
    #[allow(missing_docs)]
    pub id: i64,
    #[allow(missing_docs)]
    pub timestamp: u64,
    #[allow(missing_docs)]
    pub amount: Decimal,
    #[allow(missing_docs)]
    pub fee: Decimal,
    #[allow(missing_docs)]
    pub tax: Decimal,
}

/// **Sample**:
/// ```json
#[doc = include_str!("crypto_sample.json")]
/// ```
/// See also <https://docs.btcturk.com/private-endpoints/user-transactions>
#[derive(
    serde::Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[serde(rename_all = "camelCase")]
pub struct CryptoTransaction {
    #[allow(missing_docs)]
    pub balance_type: TransactionType,
    #[allow(missing_docs)]
    pub currency_symbol: String,
    #[allow(missing_docs)]
    pub address: String,
    #[allow(missing_docs)]
    pub tag: String,
    #[allow(missing_docs)]
    pub tx_hash: String,
    #[allow(missing_docs)]
    pub confirmation_count: u64,
    #[allow(missing_docs)]
    pub is_confirmed: bool,
    #[allow(missing_docs)]
    pub id: i64,
    #[allow(missing_docs)]
    pub timestamp: u64,
    #[allow(missing_docs)]
    pub amount: Decimal,
    #[allow(missing_docs)]
    pub fee: Decimal,
    #[allow(missing_docs)]
    pub tax: Decimal,
}

/// **Sample**:
/// ```json
#[doc = include_str!("fiat_sample.json")]
/// ```
/// See also <https://docs.btcturk.com/private-endpoints/user-transactions>
#[derive(
    serde::Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[serde(rename_all = "camelCase")]
pub struct FiatTransaction {
    #[allow(missing_docs)]
    pub balance_type: TransactionType,
    #[allow(missing_docs)]
    pub currency_symbol: String,
    #[allow(missing_docs)]
    pub address: Option<String>,
    #[allow(missing_docs)]
    pub id: i64,
    #[allow(missing_docs)]
    pub timestamp: u64,
    #[allow(missing_docs)]
    pub amount: Decimal,
    #[allow(missing_docs)]
    pub fee: Decimal,
    #[allow(missing_docs)]
    pub tax: Decimal,
}

#[cfg(test)]
mod tests {
    use crate::{ApiKeys, Client};
    use pretty_assertions::assert_str_eq;

    use super::{CryptoTransaction, FiatTransaction, TradeTransaction};

    #[ignore]
    #[async_std::test]
    async fn get_trade_transactions() {
        let _ = env_logger::builder().is_test(true).try_init();

        let keys = ApiKeys::load_from_env_var();

        let transactions = Client::new(Some(keys), None)
            .unwrap()
            .trade_transactions(None, None, vec!["try"], None)
            .await
            .unwrap();
        for transaction in transactions {
            assert_str_eq!(transaction.numerator_symbol, "TRY");
        }
    }

    #[ignore]
    #[async_std::test]
    async fn get_crypto_transactions() {
        let _ = env_logger::builder().is_test(true).try_init();

        let keys = ApiKeys::load_from_env_var();

        let transactions = Client::new(Some(keys), None)
            .unwrap()
            .crypto_transactions(None, vec!["btc"], None)
            .await
            .unwrap();
        for transaction in transactions {
            assert_str_eq!(transaction.currency_symbol, "BTC");
        }
    }

    #[ignore]
    #[async_std::test]
    async fn get_fiat_transactions() {
        let _ = env_logger::builder().is_test(true).try_init();

        let keys = ApiKeys::load_from_env_var();

        let transactions = Client::new(Some(keys), None)
            .unwrap()
            .fiat_transactions(None, vec!["try"], None)
            .await
            .unwrap();
        for transaction in transactions {
            assert_str_eq!(transaction.currency_symbol, "TRY");
        }
    }

    #[test]
    fn deserialize_trade_transaction() {
        let json_string = include_str!("trade_sample.json");
        serde_json::from_str::<Vec<TradeTransaction>>(json_string).unwrap();
    }

    #[test]
    fn deserialize_crypto_transaction() {
        let json_string = include_str!("crypto_sample.json");
        serde_json::from_str::<Vec<CryptoTransaction>>(json_string).unwrap();
    }

    #[test]
    fn deserialize_fiat_transaction() {
        let json_string = include_str!("fiat_sample.json");
        serde_json::from_str::<Vec<FiatTransaction>>(json_string).unwrap();
    }
}
