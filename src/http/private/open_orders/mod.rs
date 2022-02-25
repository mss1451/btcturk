//! Implementation of the open orders endpoint.

use rust_decimal::Decimal;
use serde::Deserialize;
use surf::http::Method;

use crate::{
    error::SendRequest,
    http::{request::Parameters, OrderMethod, OrderType, Request},
    Client,
};

impl Client<'_> {
    /// List your current open orders. Only open or un-settled orders are
    /// returned by default. As soon as an order is no longer open and settled,
    /// it will no longer appear in the default request. Open orders may change
    /// state between the request and the response depending on market
    /// conditions.
    /// # Errors
    /// [`SendRequest`] if there is an error sending the request or there
    /// is an error or a malformation in the received response.
    /// # Parameters
    /// - `pair_symbol`: For example, `BTCUSDT`.
    ///
    /// See also <https://docs.btcturk.com/private-endpoints/open-orders>.
    pub async fn open_orders(
        &self,
        pair_symbol: impl Into<String> + Send,
    ) -> Result<OpenOrders, SendRequest> {
        let mut parameters = Parameters::new();
        parameters.push_string("pairSymbol", Some(pair_symbol.into()));
        self.send(
            Request {
                endpoint: self.url_cache().open_orders(),
                method: Method::Get,
                parameters,
                requires_auth: true,
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
/// See also <https://docs.btcturk.com/private-endpoints/open-orders>
#[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub struct OpenOrders {
    #[allow(missing_docs)]
    pub asks: Vec<BidAsk>,
    #[allow(missing_docs)]
    pub bids: Vec<BidAsk>,
}

#[allow(missing_docs)]
#[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub struct BidAsk {
    #[allow(missing_docs)]
    pub id: i64,
    #[allow(missing_docs)]
    pub price: Decimal,
    #[allow(missing_docs)]
    pub amount: Decimal,
    #[allow(missing_docs)]
    pub quantity: Decimal,
    #[allow(missing_docs)]
    pub stop_price: Decimal,
    #[allow(missing_docs)]
    pub pair_symbol: String,
    #[allow(missing_docs)]
    pub pair_symbol_normalized: String,
    #[allow(missing_docs)]
    pub r#type: OrderType,
    #[allow(missing_docs)]
    pub method: OrderMethod,
    #[allow(missing_docs)]
    pub order_client_id: String,
    #[allow(missing_docs)]
    pub time: u64,
    #[allow(missing_docs)]
    pub update_time: u64,
    #[allow(missing_docs)]
    pub status: String,
    #[allow(missing_docs)]
    pub left_amount: Decimal,
}

#[cfg(test)]
mod tests {
    use super::OpenOrders;
    use crate::{ApiKeys, Client};
    use pretty_assertions::assert_str_eq;

    #[ignore]
    #[async_std::test]
    async fn get_open_orders() {
        let _ = env_logger::builder().is_test(true).try_init();

        let keys = ApiKeys::load_from_env_var();

        let orders = Client::new(Some(keys), None)
            .unwrap()
            .open_orders("SHIBUSDT")
            .await
            .unwrap();
        for bid_ask in orders.asks.iter().chain(orders.bids.iter()) {
            assert_str_eq!(bid_ask.pair_symbol_normalized, "SHIB_USDT");
        }
    }

    #[test]
    fn deserialize_open_orders() {
        let json_string = include_str!("sample.json");
        serde_json::from_str::<OpenOrders>(json_string).unwrap();
    }
}
