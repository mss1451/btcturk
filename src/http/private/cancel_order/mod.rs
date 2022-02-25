//! Implementation of the cancel order endpoint.

use serde::Deserialize;
use surf::http::Method;

use crate::{
    error::SendRequest,
    http::{request::Parameters, Request},
    Client,
};

impl Client<'_> {
    /// Cancel an order.
    /// # Errors
    /// [`SendRequest`] if there is an error sending the request or there
    /// is an error or a malformation in the received response.
    /// # Parameters
    /// - `id`: Identifier of the order.
    ///
    /// See also <https://docs.btcturk.com/private-endpoints/cancel-order>.
    pub async fn cancel_order(&self, id: i64) -> Result<(), SendRequest> {
        let mut parameters = Parameters::new();
        parameters.push_number("id", Some(id));
        self.send::<EmptyResponse>(
            Request {
                endpoint: self.url_cache().submit_cancel_order(),
                method: Method::Delete,
                parameters,
                requires_auth: true,
            },
            false,
        )
        .await?;
        Ok(())
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct EmptyResponse;

#[cfg(test)]
mod tests {
    use crate::{error::SendRequest, ApiKeys, Client};
    use log::info;

    #[ignore]
    #[async_std::test]
    async fn cancel_order() {
        let _ = env_logger::builder().is_test(true).try_init();

        let keys = ApiKeys::load_from_env_var();

        let result = Client::new(Some(keys), None)
            .unwrap()
            .cancel_order(7218394218)
            .await;
        info!("result is {:?}", result);
        match result {
            Err(ref err) => match err {
                SendRequest::SerdeJsonError { source: _ } => result.unwrap(),
                _ => (),
            },
            _ => (),
        }
    }
}
