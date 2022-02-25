//! Implementation of the all orders endpoint.

use std::ops::Range;

use rust_decimal::Decimal;
use serde::Deserialize;
use surf::http::Method;

use crate::{
    error::{Parameter, SendRequest},
    http::{request::Parameters, OrderMethod, OrderStatus, Request},
    Client,
};

impl Client<'_> {
    /// Retrieve all orders of any status.
    /// # Errors
    /// [`SendRequest`] if there is an error sending the request or there
    /// is an error or a malformation in the received response.
    /// # Parameters
    /// - `order_id`: If orderId set, it will return all orders greater than or
    /// equals to this order id.
    /// - `pair_symbol`: For example, `BTCTRY`.
    /// - `time_range`: Start-end date timestamp range.
    /// - `page`: Page number.
    /// - `limit`: Default **100**, max **1000**.
    ///
    /// See also <https://docs.btcturk.com/private-endpoints/all-orders>.
    pub async fn all_orders(
        &self,
        order_id: Option<i64>,
        pair_symbol: impl Into<String> + Send,
        time_range: Option<Range<u64>>,
        page: Option<u64>,
        limit: Option<u16>,
    ) -> Result<Vec<Order>, SendRequest> {
        let mut parameters = Parameters::new();
        parameters.push_number("orderId", order_id);
        parameters.push_string("pairSymbol", Some(pair_symbol.into()));
        if let Some(range) = time_range {
            parameters.push_number("startTime", Some(range.start));
            parameters.push_number("endTime", Some(range.end));
        }
        parameters.push_number("page", page);
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
                endpoint: self.url_cache().all_orders(),
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
/// See also <https://docs.btcturk.com/private-endpoints/all-orders>
#[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    #[allow(missing_docs)]
    pub id: i64,
    #[allow(missing_docs)]
    pub price: Decimal,
    #[allow(missing_docs)]
    pub amount: Decimal,
    #[allow(missing_docs)]
    pub quantity: Decimal,
    #[allow(missing_docs)]
    pub pair_symbol: String,
    #[allow(missing_docs)]
    pub pair_symbol_normalized: String,
    #[allow(missing_docs)]
    pub r#type: String,
    #[allow(missing_docs)]
    pub method: OrderMethod,
    #[allow(missing_docs)]
    pub order_client_id: String,
    #[allow(missing_docs)]
    pub time: u64,
    #[allow(missing_docs)]
    pub update_time: u64,
    #[allow(missing_docs)]
    pub status: OrderStatus,
}

#[cfg(test)]
mod tests {
    use crate::{ApiKeys, Client};
    use pretty_assertions::assert_str_eq;

    use super::Order;

    #[ignore]
    #[async_std::test]
    async fn get_all_orders() {
        let _ = env_logger::builder().is_test(true).try_init();

        let keys = ApiKeys::load_from_env_var();

        let orders = Client::new(Some(keys), None)
            .unwrap()
            .all_orders(None, "XRPUSDT", None, None, None)
            .await
            .unwrap();
        for order in orders {
            assert_str_eq!(order.pair_symbol_normalized, "XRP_USDT");
        }
    }

    #[test]
    fn deserialize_all_orders() {
        let json_string = include_str!("sample.json");
        serde_json::from_str::<Vec<Order>>(json_string).unwrap();
    }
}
