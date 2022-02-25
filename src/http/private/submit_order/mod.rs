//! Implementation of the submit order endpoint and its helper methods.

use rust_decimal::Decimal;
use serde::Deserialize;
use surf::http::Method;

use crate::{
    error::SendRequest,
    http::{request, OrderMethod, OrderType, Request},
    Client,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Parameters {
    quantity: Option<Decimal>,
    price: Option<Decimal>,
    stop_price: Option<Decimal>,
    new_order_client_id: Option<String>,
    order_method: OrderMethod,
    order_type: OrderType,
    pair_symbol: String,
}

impl<'a, 'i> Client<'i> {
    async fn submit_order(
        &self,
        parameters: Parameters,
    ) -> Result<NewOrder, SendRequest> {
        let mut params = request::Parameters::new();
        params.push_decimal("quantity", parameters.quantity);
        params.push_decimal("price", parameters.price);
        params.push_decimal("stopPrice", parameters.stop_price);
        params.push_string("newOrderClientId", parameters.new_order_client_id);
        params.push_object("orderMethod", Some(parameters.order_method));
        params.push_object("orderType", Some(parameters.order_type));
        params.push_string("pairSymbol", Some(parameters.pair_symbol));
        self.send(
            Request {
                endpoint: self.url_cache().submit_cancel_order(),
                method: Method::Post,
                parameters: params,
                requires_auth: true,
            },
            false,
        )
        .await
    }

    async fn market(
        &self,
        pair_symbol: String,
        quantity: Decimal,
        order_type: OrderType,
    ) -> Result<NewOrder, SendRequest> {
        self.submit_order(Parameters {
            quantity: Some(quantity),
            price: None,
            stop_price: None,
            new_order_client_id: self.id().map(ToOwned::to_owned),
            order_method: OrderMethod::Market,
            order_type,
            pair_symbol,
        })
        .await
    }

    /// Submits an order with parameters adjusted to perform a market buy.
    ///
    /// [SubmitOrder]: https://docs.btcturk.com/private-endpoints/submit-order
    /// # Errors
    /// [`SendRequest`] if there is an error sending the request or there
    /// is an error or a malformation in the received response.
    /// # Parameters
    /// - `pair_symbol`: For example, `BTCUSDT`.
    /// - `quantity`: Mandatory for market or limit orders.
    pub async fn market_buy(
        &self,
        pair_symbol: impl Into<String> + Send,
        quantity: Decimal,
    ) -> Result<NewOrder, SendRequest> {
        self.market(pair_symbol.into(), quantity, OrderType::Buy)
            .await
    }

    /// Submits an order with parameters adjusted to perform a market sell.
    ///
    /// [SubmitOrder]: https://docs.btcturk.com/private-endpoints/submit-order
    /// # Errors
    /// [`SendRequest`] if there is an error sending the request or there
    /// is an error or a malformation in the received response.
    /// # Parameters
    /// - `pair_symbol`: For example, `BTCUSDT`.
    /// - `quantity`: Mandatory for market or limit orders.
    pub async fn market_sell(
        &self,
        pair_symbol: impl Into<String> + Send,
        quantity: Decimal,
    ) -> Result<NewOrder, SendRequest> {
        self.market(pair_symbol.into(), quantity, OrderType::Sell)
            .await
    }

    async fn limit(
        &self,
        pair_symbol: String,
        quantity: Decimal,
        price: Decimal,
        order_type: OrderType,
    ) -> Result<NewOrder, SendRequest> {
        self.submit_order(Parameters {
            quantity: Some(quantity),
            price: Some(price),
            stop_price: None,
            new_order_client_id: self.id().map(ToOwned::to_owned),
            order_method: OrderMethod::Limit,
            order_type,
            pair_symbol,
        })
        .await
    }

    /// Submits an order with parameters adjusted to perform a limit buy.
    ///
    /// [SubmitOrder]: https://docs.btcturk.com/private-endpoints/submit-order
    /// # Errors
    /// [`SendRequest`] if there is an error sending the request or there
    /// is an error or a malformation in the received response.
    /// # Parameters
    /// - `pair_symbol`: For example, `BTCUSDT`.
    /// - `price`: Will be ignored for market orders. Market orders
    /// get filled with different prices until your order is completely filled.
    /// There is a 5% limit on the difference between the first price and the
    /// last price. I.e. you can't buy at a price more than 5% higher than the
    /// best sell at the time of order submission and you can't sell at a price
    /// less than 5% lower than the best buy at the time of order submission.
    /// - `quantity`: Mandatory for market or limit orders.
    pub async fn limit_buy(
        &self,
        pair_symbol: impl Into<String> + Send,
        price: Decimal,
        quantity: Decimal,
    ) -> Result<NewOrder, SendRequest> {
        self.limit(pair_symbol.into(), quantity, price, OrderType::Buy)
            .await
    }

    /// Submits an order with parameters adjusted to perform a limit sell.
    ///
    /// [SubmitOrder]: https://docs.btcturk.com/private-endpoints/submit-order
    /// # Errors
    /// [`SendRequest`] if there is an error sending the request or there
    /// is an error or a malformation in the received response.
    /// # Parameters
    /// - `pair_symbol`: For example, `BTCUSDT`.
    /// - `price`: Will be ignored for market orders. Market orders
    /// get filled with different prices until your order is completely filled.
    /// There is a 5% limit on the difference between the first price and the
    /// last price. I.e. you can't buy at a price more than 5% higher than the
    /// best sell at the time of order submission and you can't sell at a price
    /// less than 5% lower than the best buy at the time of order submission.
    /// - `quantity`: Mandatory for market or limit orders.
    pub async fn limit_sell(
        &self,
        pair_symbol: impl Into<String> + Send,
        price: Decimal,
        quantity: Decimal,
    ) -> Result<NewOrder, SendRequest> {
        self.limit(pair_symbol.into(), quantity, price, OrderType::Sell)
            .await
    }

    async fn stop_limit(
        &self,
        pair_symbol: String,
        quantity: Decimal,
        price: Decimal,
        stop_price: Decimal,
        order_type: OrderType,
    ) -> Result<NewOrder, SendRequest> {
        self.submit_order(Parameters {
            quantity: Some(quantity),
            price: Some(price),
            stop_price: Some(stop_price),
            new_order_client_id: self.id().map(ToOwned::to_owned),
            order_method: OrderMethod::Limit,
            order_type,
            pair_symbol,
        })
        .await
    }

    /// Submits an order with parameters adjusted to perform a stop limit
    /// buy.
    ///
    /// [SubmitOrder]: https://docs.btcturk.com/private-endpoints/submit-order
    /// # Errors
    /// [`SendRequest`] if there is an error sending the request or there
    /// is an error or a malformation in the received response.
    /// # Parameters
    /// - `pair_symbol`: For example, `BTCUSDT`.
    /// - `price`: Will be ignored for market orders. Market orders
    /// get filled with different prices until your order is completely filled.
    /// There is a 5% limit on the difference between the first price and the
    /// last price. I.e. you can't buy at a price more than 5% higher than the
    /// best sell at the time of order submission and you can't sell at a price
    /// less than 5% lower than the best buy at the time of order submission.
    /// - `stop_price`: For stop orders.
    /// - `quantity`: Mandatory for market or limit orders.
    pub async fn stop_limit_buy(
        &self,
        pair_symbol: impl Into<String> + Send,
        price: Decimal,
        stop_price: Decimal,
        quantity: Decimal,
    ) -> Result<NewOrder, SendRequest> {
        self.stop_limit(
            pair_symbol.into(),
            quantity,
            price,
            stop_price,
            OrderType::Buy,
        )
        .await
    }

    /// Submits an order with parameters adjusted to perform a stop limit
    /// sell.
    ///
    /// [SubmitOrder]: https://docs.btcturk.com/private-endpoints/submit-order
    /// # Errors
    /// [`SendRequest`] if there is an error sending the request or there
    /// is an error or a malformation in the received response.
    /// # Parameters
    /// - `pair_symbol`: For example, `BTCUSDT`.
    /// - `price`: Will be ignored for market orders. Market orders
    /// get filled with different prices until your order is completely filled.
    /// There is a 5% limit on the difference between the first price and the
    /// last price. I.e. you can't buy at a price more than 5% higher than the
    /// best sell at the time of order submission and you can't sell at a price
    /// less than 5% lower than the best buy at the time of order submission.
    /// - `stop_price`: For stop orders.
    /// - `quantity`: Mandatory for market or limit orders.
    pub async fn stop_limit_sell(
        &self,
        pair_symbol: impl Into<String> + Send,
        price: Decimal,
        stop_price: Decimal,
        quantity: Decimal,
    ) -> Result<NewOrder, SendRequest> {
        self.stop_limit(
            pair_symbol.into(),
            quantity,
            price,
            stop_price,
            OrderType::Sell,
        )
        .await
    }
}

/// **Sample**:
/// ```json
#[doc = include_str!("sample.json")]
/// ```
/// See also <https://docs.btcturk.com/private-endpoints/submit-order>
#[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub struct NewOrder {
    #[allow(missing_docs)]
    pub id: i64,
    #[allow(missing_docs)]
    #[serde(rename = "datetime")]
    pub date_time: u64,
    #[allow(missing_docs)]
    pub r#type: OrderType,
    #[allow(missing_docs)]
    pub method: OrderMethod,
    #[allow(missing_docs)]
    pub price: Option<Decimal>,
    #[allow(missing_docs)]
    pub stop_price: Option<Decimal>,
    #[allow(missing_docs)]
    pub quantity: Option<Decimal>,
    #[allow(missing_docs)]
    pub pair_symbol: String,
    #[allow(missing_docs)]
    pub pair_symbol_normalized: String,
    #[allow(missing_docs)]
    pub new_order_client_id: String,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{ApiKeys, Client};
    use pretty_assertions::{assert_eq, assert_str_eq};
    use rust_decimal::Decimal;

    use super::NewOrder;

    fn init_client() -> Client<'static> {
        let _ = env_logger::builder().is_test(true).try_init();
        let keys = ApiKeys::load_from_env_var();
        Client::new(Some(keys), Some("test")).unwrap()
    }

    #[ignore]
    #[async_std::test]
    async fn market_buy() {
        let new_order = init_client()
            .market_buy("XRPUSDT", Decimal::TEN)
            .await
            .unwrap();
        assert_str_eq!(new_order.new_order_client_id, "test");
        assert_str_eq!(new_order.pair_symbol_normalized, "XRP_USDT");
        assert_eq!(new_order.price, None);
        assert_eq!(new_order.quantity, Some(Decimal::TEN));
    }

    #[ignore]
    #[async_std::test]
    async fn market_sell() {
        let new_order = init_client()
            .market_sell("DOGEUSDT", Decimal::ONE_HUNDRED)
            .await
            .unwrap();
        assert_str_eq!(new_order.new_order_client_id, "test");
        assert_str_eq!(new_order.pair_symbol_normalized, "DOGE_USDT");
        assert_eq!(new_order.price, None);
        assert_eq!(new_order.quantity, Some(Decimal::ONE_HUNDRED));
    }

    #[ignore]
    #[async_std::test]
    async fn limit_buy() {
        let price = Decimal::from_str("0.679").unwrap();
        let quantity = Decimal::from_str("15").unwrap();
        let new_order = init_client()
            .limit_buy("XRPUSDT", price, quantity)
            .await
            .unwrap();
        assert_str_eq!(new_order.new_order_client_id, "test");
        assert_str_eq!(new_order.pair_symbol_normalized, "XRP_USDT");
        assert_eq!(new_order.price, Some(price));
        assert_eq!(new_order.quantity, Some(quantity));
    }

    #[ignore]
    #[async_std::test]
    async fn limit_sell() {
        let new_order = init_client()
            .limit_buy("ADAUSDT", Decimal::ONE, Decimal::TEN)
            .await
            .unwrap();
        assert_str_eq!(new_order.new_order_client_id, "test");
        assert_str_eq!(new_order.pair_symbol_normalized, "ADA_USDT");
        assert_eq!(new_order.price, Some(Decimal::TEN));
        assert_eq!(new_order.quantity, Some(Decimal::ONE));
    }

    #[ignore]
    #[async_std::test]
    async fn stop_limit_buy() {
        let new_order = init_client()
            .stop_limit_buy(
                "DOGEUSDT",
                Decimal::ONE_HUNDRED,
                Decimal::TEN,
                Decimal::ONE,
            )
            .await
            .unwrap();
        assert_str_eq!(new_order.new_order_client_id, "test");
        assert_str_eq!(new_order.pair_symbol_normalized, "DOGE_USDT");
        assert_eq!(new_order.price, Some(Decimal::ONE_HUNDRED));
        assert_eq!(new_order.stop_price, Some(Decimal::TEN));
        assert_eq!(new_order.quantity, Some(Decimal::ONE));
    }

    #[ignore]
    #[async_std::test]
    async fn stop_limit_sell() {
        let new_order = init_client()
            .stop_limit_buy("XRPUSDT", Decimal::ONE, Decimal::TWO, Decimal::TEN)
            .await
            .unwrap();
        assert_str_eq!(new_order.new_order_client_id, "test");
        assert_str_eq!(new_order.pair_symbol_normalized, "XRP_USDT");
        assert_eq!(new_order.price, Some(Decimal::ONE));
        assert_eq!(new_order.stop_price, Some(Decimal::TWO));
        assert_eq!(new_order.quantity, Some(Decimal::TEN));
    }

    #[test]
    fn deserialize_new_order() {
        let json_string = include_str!("sample.json");
        serde_json::from_str::<NewOrder>(json_string).unwrap();
    }
}
