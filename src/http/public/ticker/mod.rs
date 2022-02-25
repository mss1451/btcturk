//! Implementation of the ticker and currency endpoints.

use surf::http::Method;

use crate::{
    error::{Response as ResponseError, SendRequest},
    http::{request::Parameters, Client, Request},
};

use rust_decimal::Decimal;

use std::fmt::Display;

/// Available currencies in the exchange to be used with
/// the [`currency`][Client::currency] method.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Currency {
    /// Tether
    Usdt,
    /// Turkish Lira
    Try,
    /// Bitcoin
    Btc,
}

impl Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Currency::Usdt => "USDT",
            Currency::Try => "TRY",
            Currency::Btc => "BTC",
        })
    }
}

impl From<Currency> for String {
    fn from(value: Currency) -> Self {
        value.to_string()
    }
}

impl Client<'_> {
    /// Gets snapshot information about the last trade (tick), best bid/ask and
    /// 24h volume. \
    /// Using the `pair_symbol` parameter, you can send a request for a single
    /// pair.
    ///
    /// If you want to get all tickers, use [`tickers`][Self::tickers]
    /// method.
    ///
    /// # Parameters
    /// - `pair_symbol`: For example, `BTCUSDT`.
    /// # Errors
    /// [`SendRequest`] if there is an error sending the request or there
    /// is an error or a malformation in the received response.
    ///
    /// See also <https://docs.btcturk.com/public-endpoints/ticker>.
    pub async fn ticker(
        &self,
        pair_symbol: impl Into<String> + Send,
    ) -> Result<Ticker, SendRequest> {
        let mut parameters = Parameters::new();
        parameters.push_string("pairSymbol", Some(pair_symbol.into()));
        self.send::<Vec<Ticker>>(
            Request {
                endpoint: self.url_cache().ticker(),
                method: Method::Get,
                parameters,
                requires_auth: false,
            },
            false,
        )
        .await?
        .into_iter()
        .next()
        .ok_or(SendRequest::ResponseError {
            source: ResponseError::EmptyData,
        })
    }

    /// Same as [`ticker`][Self::ticker] but gets ticker for all
    /// pairs.
    /// # Errors
    /// [`SendRequest`] if there is an error sending the request or there
    /// is an error or a malformation in the received response.
    ///
    /// See also <https://docs.btcturk.com/public-endpoints/ticker>.
    pub async fn tickers(&self) -> Result<Vec<Ticker>, SendRequest> {
        self.send(
            Request {
                endpoint: self.url_cache().ticker(),
                method: Method::Get,
                parameters: Parameters::new(),
                requires_auth: false,
            },
            false,
        )
        .await
    }

    /// Same as [`ticker`][Self::ticker] but accepts a currency
    /// instead of a symbol pair and returns tickers of the symbols paired with
    /// that currency.
    ///
    /// # Parameters
    /// - `symbol`: Can be one of `USDT`, `TRY`, or `BTC`.
    /// # Errors
    /// [`SendRequest`] if there is an error sending the request or there
    /// is an error or a malformation in the received response.
    ///
    /// See also <https://docs.btcturk.com/public-endpoints/ticker#get-currency>.
    pub async fn currency(
        &self,
        symbol: Currency,
    ) -> Result<Vec<Ticker>, SendRequest> {
        let mut parameters = Parameters::new();
        parameters.push_object("symbol", Some(symbol));
        self.send(
            Request {
                endpoint: self.url_cache().currency(),
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
/// See also <https://docs.btcturk.com/public-endpoints/ticker>
#[derive(
    serde::Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[serde(rename_all = "camelCase")]
pub struct Ticker {
    #[allow(missing_docs)]
    pub pair: String,
    #[allow(missing_docs)]
    pub pair_normalized: String,
    #[allow(missing_docs)]
    pub timestamp: u64,
    #[allow(missing_docs)]
    pub last: Decimal,
    #[allow(missing_docs)]
    pub high: Decimal,
    #[allow(missing_docs)]
    pub low: Decimal,
    #[allow(missing_docs)]
    pub bid: Decimal,
    #[allow(missing_docs)]
    pub ask: Decimal,
    #[allow(missing_docs)]
    pub open: Decimal,
    #[allow(missing_docs)]
    pub volume: Decimal,
    #[allow(missing_docs)]
    pub average: Decimal,
    #[allow(missing_docs)]
    pub daily: Decimal,
    #[allow(missing_docs)]
    pub daily_percent: Decimal,
    #[allow(missing_docs)]
    pub denominator_symbol: String,
    #[allow(missing_docs)]
    pub numerator_symbol: String,
    #[allow(missing_docs)]
    pub order: u64,
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;

    use crate::http::{
        public::ticker::{Currency, Ticker},
        Client,
    };

    #[ignore]
    #[async_std::test]
    async fn get_ticker() {
        let _ = env_logger::builder().is_test(true).try_init();

        let last = Client::new(None, None)
            .unwrap()
            .ticker("XRPUSDT")
            .await
            .unwrap()
            .last;
        assert!(last > Decimal::ZERO);
    }

    #[ignore]
    #[async_std::test]
    async fn get_tickers() {
        let _ = env_logger::builder().is_test(true).try_init();

        let last = Client::new(None, None)
            .unwrap()
            .tickers()
            .await
            .unwrap()
            .into_iter()
            .find(|ticker| ticker.pair_normalized.eq("ETH_USDT"))
            .unwrap()
            .last;
        assert!(last > Decimal::ZERO);
    }

    #[ignore]
    #[async_std::test]
    async fn get_currency() {
        let _ = env_logger::builder().is_test(true).try_init();

        let Ticker { low, high, .. } = Client::new(None, None)
            .unwrap()
            .currency(Currency::Try)
            .await
            .unwrap()
            .into_iter()
            .find(|ticker| ticker.pair.eq("LTCTRY"))
            .unwrap();
        assert!(high >= low);
    }

    #[test]
    fn deserialize_ticker() {
        let json_string = include_str!("sample.json");
        serde_json::from_str::<Ticker>(json_string).unwrap();
    }
}
