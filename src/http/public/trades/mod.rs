//! Implementation of the trades endpoint.

use rust_decimal::Decimal;
use surf::http::Method;

use crate::{
    error::{Parameter, SendRequest},
    http::{request::Parameters, OrderType, Request},
    Client,
};

impl Client<'_> {
    /// Gets a list the latest trades for a product.
    ///
    /// # Parameters
    /// - `pair_symbol`: For example, `BTCUSDT`. \
    /// - `last`: Number of the most recent trades to get. Max **50**,
    /// defaults to 50.
    /// # Errors
    /// [`SendRequest`] if there is an error sending the request or there
    /// is an error or a malformation in the received response.
    ///
    /// See also <https://docs.btcturk.com/public-endpoints/trades>.
    pub async fn trades(
        &self,
        pair_symbol: impl Into<String> + Send,
        last: Option<u8>,
    ) -> Result<Vec<Trade>, SendRequest> {
        let mut parameters = Parameters::new();
        parameters.push_string("pairSymbol", Some(pair_symbol.into()));
        if let Some(last) = last {
            if last > 50 {
                return Err(SendRequest::ParameterError {
                    source: Parameter::new("last", last.to_string()),
                });
            }
            parameters.push_number("last", Some(last));
        }
        self.send(
            Request {
                endpoint: self.url_cache().trades(),
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
/// ```json
#[doc = include_str!("sample.json")]
/// ```
/// See also <https://docs.btcturk.com/public-endpoints/trades>
#[derive(
    serde::Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    #[allow(missing_docs)]
    pub pair: String,
    #[allow(missing_docs)]
    pub pair_normalized: String,
    #[allow(missing_docs)]
    pub denominator: String,
    #[allow(missing_docs)]
    pub date: u64,
    #[allow(missing_docs)]
    #[serde(rename = "tid")]
    pub trade_id: String,
    #[allow(missing_docs)]
    pub price: Decimal,
    #[allow(missing_docs)]
    pub amount: Decimal,
    #[allow(missing_docs)]
    pub side: OrderType,
}

#[cfg(test)]
mod tests {
    use crate::Client;
    use pretty_assertions::assert_ne;

    use super::Trade;

    #[ignore]
    #[async_std::test]
    async fn get_trades() {
        let _ = env_logger::builder().is_test(true).try_init();

        let trades = Client::new(None, None)
            .unwrap()
            .trades("BTCUSDT", Some(2))
            .await
            .unwrap();
        assert!(trades.len() == 2);
        assert!(trades[0].date > trades[1].date);
        assert_ne!(trades[0].trade_id, trades[1].trade_id);
    }

    #[test]
    fn deserialize_trades() {
        let json_string = include_str!("sample.json");
        serde_json::from_str::<Vec<Trade>>(json_string).unwrap();
    }
}
