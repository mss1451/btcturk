//! Implementation of the account balance endpoint.

use rust_decimal::Decimal;
use surf::http::Method;

use crate::{
    error::SendRequest,
    http::{request::Parameters, Request},
    Client,
};

impl Client<'_> {
    /// Retrieve all cash balances.
    /// # Errors
    /// [`SendRequest`] if there is an error sending the request or there
    /// is an error or a malformation in the received response.
    ///
    /// See also <https://docs.btcturk.com/private-endpoints/account-balance>.
    pub async fn account_balance(
        &self,
    ) -> Result<Vec<AssetBalance>, SendRequest> {
        self.send(
            Request {
                endpoint: self.url_cache().account_balance(),
                method: Method::Get,
                parameters: Parameters::new(),
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
/// See also <https://docs.btcturk.com/private-endpoints/account-balance>
#[derive(
    serde::Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[serde(rename_all = "camelCase")]
pub struct AssetBalance {
    #[allow(missing_docs)]
    pub asset: String,
    #[serde(rename = "assetname")]
    #[allow(missing_docs)]
    pub asset_name: String,
    #[allow(missing_docs)]
    pub balance: Decimal,
    #[allow(missing_docs)]
    pub locked: Decimal,
    #[allow(missing_docs)]
    pub free: Decimal,
}

#[cfg(test)]
mod tests {
    use crate::{ApiKeys, Client};

    use super::AssetBalance;

    #[ignore]
    #[async_std::test]
    async fn get_account_balance() {
        let _ = env_logger::builder().is_test(true).try_init();

        let keys = ApiKeys::load_from_env_var();

        let assets = Client::new(Some(keys), None)
            .unwrap()
            .account_balance()
            .await
            .unwrap();
        for asset in assets {
            assert!(asset.balance >= asset.free + asset.locked);
        }
    }

    #[test]
    fn deserialize_account_balance() {
        let json_string = include_str!("sample.json");
        serde_json::from_str::<Vec<AssetBalance>>(json_string).unwrap();
    }
}
