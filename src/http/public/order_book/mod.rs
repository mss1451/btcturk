//! Implementation of the order book endpoint.

use surf::http::Method;

use rust_decimal::Decimal;
use serde::Deserialize;

use crate::{
    error::{Parameter, SendRequest},
    http::{request::Parameters, Client, Request},
};

impl Client<'_> {
    /// Get a list of all open orders for a product.
    ///
    /// In case of a system failure and delays in real time order book data,
    /// this endpoint will return HTTP 503 in order to prevent false market data
    /// feed to clients.
    ///
    /// # Parameters
    /// - `pair_symbol`: For example, `BTCUSDT`.
    /// - `limit`: Number of orders to get. Maximum is **1000**.
    /// Defaults to **100**.
    /// # Errors
    /// [`SendRequest`] if there is an error sending the request or there
    /// is an error or a malformation in the received response.
    ///
    /// See also <https://docs.btcturk.com/public-endpoints/orderbook>.
    pub async fn order_book(
        &self,
        pair_symbol: impl Into<String> + Send,
        limit: Option<u16>,
    ) -> Result<OrderBook, SendRequest> {
        let mut parameters = Parameters::new();
        parameters.push_string("pairSymbol", Some(pair_symbol.into()));
        if let Some(limit) = limit {
            if limit > 1000 {
                return Err(
                    Parameter::new("limit", limit.to_string()).into()
                );
            }
            parameters.push_number("limit", Some(limit));
        }
        self.send(
            Request {
                endpoint: self.url_cache().order_book(),
                method: Method::Get,
                parameters,
                requires_auth: false,
            },
            false,
        )
        .await
    }
}

/// **Sample**:
///```json
#[doc = include_str!("sample.json")]
///```
///See also <https://docs.btcturk.com/public-endpoints/orderbook>
#[derive(serde::Deserialize, Debug, Clone, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
    #[allow(missing_docs)]
    pub timestamp: f64,
    #[allow(missing_docs)]
    pub bids: Vec<BidAsk>,
    #[allow(missing_docs)]
    pub asks: Vec<BidAsk>,
}

/// **Sample**:
/// ```json
/// [
///    "36371", // Best bid/ask price.
///    "0.00080000" // Best bid/ask amount.
/// ]
/// ```
/// See also <https://docs.btcturk.com/public-endpoints/orderbook>
#[derive(
    Deserialize, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[serde(from = "BidAskRaw")]
pub struct BidAsk {
    /// Best bid/ask price.
    pub price: Decimal,
    /// Best bid/ask amount.
    pub amount: Decimal,
}

impl From<BidAskRaw> for BidAsk {
    fn from(raw: BidAskRaw) -> Self {
        Self {
            price: raw.0,
            amount: raw.1,
        }
    }
}

#[derive(serde::Deserialize)]
struct BidAskRaw(Decimal, Decimal);

#[cfg(test)]
mod tests {
    use crate::{
        error::SendRequest,
        http::{public::order_book::OrderBook, Client},
    };

    #[ignore]
    #[async_std::test]
    async fn get_order_book() {
        let _ = env_logger::builder().is_test(true).try_init();

        const BID_ASK_COUNT: u16 = 3;
        let OrderBook { bids, asks, .. } = Client::new(None, None)
            .unwrap()
            .order_book("BTCUSDT", Some(BID_ASK_COUNT))
            .await
            .unwrap();
        assert!(bids.len() <= BID_ASK_COUNT.try_into().unwrap());
        assert!(asks.len() <= BID_ASK_COUNT.try_into().unwrap());
    }

    #[ignore]
    #[async_std::test]
    async fn get_order_book_bad_limit() {
        let _ = env_logger::builder().is_test(true).try_init();

        const BID_ASK_COUNT: u16 = 1001;
        let err = Client::new(None, None)
            .unwrap()
            .order_book("BTCUSDT", Some(BID_ASK_COUNT))
            .await
            .unwrap_err();
        match err {
            SendRequest::ParameterError { source } => {
                assert_eq!(source.name(), "limit");
                assert_eq!(
                    source.value().to_string(),
                    BID_ASK_COUNT.to_string()
                );
            }
            other => panic!("unexpected error type: `{}`", other),
        }
    }

    #[test]
    fn deserialize_order_book() {
        let json_string = include_str!("sample.json");
        serde_json::from_str::<OrderBook>(json_string).unwrap();
    }
}
