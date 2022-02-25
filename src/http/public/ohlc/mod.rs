//! Implementation of the OHLC endpoint.

use std::ops::Range;

use surf::http::Method;

use rust_decimal::Decimal;

use crate::{
    error::SendRequest,
    http::{request::Parameters, Client, Request},
};

impl Client<'_> {
    /// Returns daily cumulative data.
    ///
    /// This is the data that is shown in our charting interface.
    ///
    /// - `open`, `high`, `low`, `close`, `volume`, `total` and `average`
    /// information can be viewed with OHLC endpoint.
    ///
    /// # Parameters
    /// - `pair`: For example, `BTCUSDT`.
    /// - `range`: This is the combination of `from` and `to` parameters.
    /// The range is UNIX time in **seconds**. An example range is
    /// 1321234542..143143265.
    /// # Errors
    /// [`SendRequest`] if there is an error sending the request or there
    /// is an error or a malformation in the received response.
    ///
    /// See also <https://docs.btcturk.com/public-endpoints/ohcl-data>.
    pub async fn ohlc(
        &self,
        pair: impl Into<String> + Send,
        range: Option<Range<u64>>,
    ) -> Result<Vec<Ohlc>, SendRequest> {
        let mut parameters = Parameters::new();
        parameters.push_string("pair", Some(pair.into()));
        if let Some(range) = range {
            parameters.push_number("from", Some(range.start));
            parameters.push_number("to", Some(range.end));
        }
        self.send(
            Request {
                endpoint: self.url_cache().ohlc(),
                method: Method::Get,
                parameters,
                requires_auth: false,
            },
            true,
        )
        .await
    }
}

/// **Sample**:
/// ```json
#[doc = include_str!("sample.json")]
/// ```
/// See also <https://docs.btcturk.com/public-endpoints/ohcl-data>
#[derive(
    serde::Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[serde(rename_all = "camelCase")]
pub struct Ohlc {
    #[allow(missing_docs)]
    pub pair: String,
    #[allow(missing_docs)]
    pub time: u64,
    #[allow(missing_docs)]
    pub open: Decimal,
    #[allow(missing_docs)]
    pub high: Decimal,
    #[allow(missing_docs)]
    pub low: Decimal,
    #[allow(missing_docs)]
    pub close: Decimal,
    #[allow(missing_docs)]
    pub volume: Decimal,
    #[allow(missing_docs)]
    pub total: Decimal,
    #[allow(missing_docs)]
    pub average: Decimal,
    #[allow(missing_docs)]
    pub daily_change_amount: Decimal,
    #[allow(missing_docs)]
    pub daily_change_percentage: Decimal,
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::Ohlc;
    use crate::http::Client;
    use pretty_assertions::assert_eq;

    #[ignore]
    #[async_std::test]
    async fn get_ohlc() {
        let _ = env_logger::builder().is_test(true).try_init();
        let current_seconds = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        // The given data is daily.
        let data_vector = Client::new(None, None)
            .unwrap()
            .ohlc("BTCUSDT", Some(current_seconds - 86400..current_seconds))
            .await
            .unwrap();
        assert_eq!(data_vector.len(), 2);
    }

    #[test]
    fn deserialize_ohlc() {
        let json_string = include_str!("sample.json");
        serde_json::from_str::<Ohlc>(json_string).unwrap();
    }
}
