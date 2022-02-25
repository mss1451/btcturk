//! General response implementation.

use crate::error::Response as ResponseError;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Response<D> {
    data: Option<D>,
    success: bool,
    message: Option<String>,
    code: i64,
}

impl<D> Response<D> {
    pub fn data(self) -> Result<D, ResponseError> {
        if !self.success {
            Err(ResponseError::Unsuccessful {
                code: self.code,
                message: self.message,
            })
        } else if let Some(data) = self.data {
            Ok(data)
        } else {
            Err(ResponseError::NullData)
        }
    }

    /// Get response's code.
    pub const fn code(&self) -> i64 {
        self.code
    }

    /// Get an optional reference to the response's message.
    pub const fn message(&self) -> Option<&String> {
        self.message.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::Response;
    use crate::error::Response as ResponseError;
    use crate::http::public::ticker::Ticker;
    use pretty_assertions::assert_eq;

    fn get_ticker_data(json_string: &str) -> Response<Vec<Ticker>> {
        serde_json::from_str::<Response<Vec<Ticker>>>(json_string).unwrap()
    }

    #[test]
    fn normal_response() {
        let json_string = r###"
    {
        "data": [
            {
                "pair": "BTCUSDT",
                "pairNormalized": "BTC_USDT",
                "timestamp": 1643883402008,
                "last": 36474,
                "high": 38724,
                "low": 36361,
                "bid": 36405,
                "ask": 36466,
                "open": 38500,
                "volume": 75.36297763,
                "average": 37550,
                "daily": -2034,
                "dailyPercent": -5.26,
                "denominatorSymbol": "USDT",
                "numeratorSymbol": "BTC",
                "order": 2001
            }
        ],
        "success": true,
        "message": null,
        "code": 0
    }
    "###;
        let response = get_ticker_data(json_string);
        assert!(response.message.is_none());
        assert_eq!(response.code, 0);
        assert_eq!(response.success, true);
        response.data().unwrap();
    }

    #[test]
    fn error_response() {
        let json_string = r###"
    {
        "data": null,
        "success": false,
        "message": "currencySymbol parameter must be set",
        "code": 1037
    }
    "###;
        let response = get_ticker_data(json_string);
        assert_eq!(
            response.data(),
            Err(ResponseError::Unsuccessful {
                code: 1037,
                message: Some(
                    "currencySymbol parameter must be set".to_string()
                )
            })
        );
    }

    #[test]
    fn null_response() {
        let json_string = r###"
    {
        "data": null,
        "success": true,
        "message": null,
        "code": 0
    }
    "###;
        let response = get_ticker_data(json_string);
        assert_eq!(response.data(), Err(ResponseError::NullData));
    }
}
