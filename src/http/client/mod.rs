mod url_cache;
use surf::{http::Method, StatusCode};
pub use url_cache::UrlCache;

use serde::de::DeserializeOwned;

use crate::{error::SendRequest, http::Response, ApiKeys};

use super::Request;

const X_PCK: &str = "X-PCK";
const X_STAMP: &str = "X-Stamp";
const X_SIGNATURE: &str = "X-Signature";

/** Used to send HTTP requests.
# Examples
## Get ticker
```no_run
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# async_std::task::block_on(async {
use btcturk::Client;

// We don't need to authenticate for this example.
let client = Client::new(None, Some("test"))?;

let btc_price = client
    .ticker("BTCUSDT")
    .await?
    .last;

# Ok::<(), Box<dyn std::error::Error>>(())
# })
# }
```
## Query open orders
```no_run
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# async_std::task::block_on(async {
use btcturk::{ApiKeys, Client, http::private::OpenOrders};

// Authentication is required for private endpoints.
let keys = ApiKeys::new("YOUR_PUBLIC_KEY", "YOUR_PRIVATE_KEY")?;

let client = Client::new(Some(keys), None)?;

let orders: OpenOrders = client
    .open_orders("BTCUSDT")
    .await?;

# Ok::<(), Box<dyn std::error::Error>>(())
# })
# }
```
*/
#[derive(Debug, Clone)]
pub struct Client<'i> {
    keys: Option<ApiKeys>,
    id: Option<&'i str>,
    http_client: surf::Client,
    url_cache: UrlCache,
}

impl<'i> Client<'i> {
    /// Construct a client with an optional [`ApiKeys`] and an optional `id`.
    /// # Parameters
    /// - `keys`: Pass some keys to the constructor to be able
    /// to use the private endpoints requiring authentication.
    /// You can also set it later by calling [`set_keys`][Self::set_keys]
    /// method.
    /// - `id`: Some requests accept an optional client identifier. If set,
    /// such a parameter will be substituted by this parameter.
    /// # Errors
    /// A [`surf`] error will occur if there is an error building an HTTP
    /// client.
    /// # Panics
    /// If any of the hardcoded endpoint URLs can't be parsed, this function
    /// will panic.
    pub fn new(
        keys: Option<ApiKeys>,
        id: Option<&'i str>,
    ) -> surf::Result<Self> {
        Ok(Self {
            keys,
            id,
            http_client: surf::Client::new(),
            url_cache: UrlCache::new(),
        })
    }

    /// Set the client's API keys. You can remove the current
    /// keys by passing `None`.
    pub fn set_keys(&mut self, keys: Option<ApiKeys>) {
        self.keys = keys;
    }

    /// Set the client's identifier. You can remove the current
    /// identifier by passing `None`.
    pub fn set_id(&mut self, id: Option<&'i str>) {
        self.id = id;
    }

    /// Get the client's id.
    #[must_use]
    pub const fn id(&self) -> Option<&str> {
        self.id
    }

    pub(crate) const fn url_cache(&self) -> &UrlCache {
        &self.url_cache
    }

    pub(crate) async fn send<D: DeserializeOwned>(
        &self,
        request: Request<'_>,
        bare_data: bool,
    ) -> Result<D, SendRequest> {
        let mut url = request.endpoint.clone();
        let body = if request.method == Method::Post {
            Some(serde_json::to_string(request.parameters.root())?)
        } else {
            let mut queries = url.query_pairs_mut();
            for (key, value) in request.parameters.root() {
                if let Some(string) = value.as_str() {
                    queries.append_pair(key, string);
                } else {
                    let string = value.to_string();
                    queries.append_pair(key, &string);
                };
            }
            None
        };
        let mut surf_request = surf::Request::new(request.method, url);
        if let Some(body) = body {
            surf_request.set_body(body);
        }
        surf_request.set_header("Content-Type", "application/json");
        if request.requires_auth {
            if let Some(keys) = &self.keys {
                let (sign, nonce) = keys.generate_sign_nonce()?;
                surf_request.set_header(X_PCK, keys.public_key());
                surf_request.set_header(X_STAMP, nonce);
                surf_request.set_header(X_SIGNATURE, sign);
            } else {
                return Err(SendRequest::AuthenticationRequired);
            }
        }
        let mut response = self.http_client.send(surf_request).await?;

        // Using `body_string` instead of `body_json` to be able to log the
        // string. The error type contains the HTTP status code.
        let response_string = response.body_string().await?;

        log::debug!("JSON response string: {}", response_string);

        let status_code = response.status();

        if status_code != StatusCode::Ok {
            let (code, message) = if let Ok(response) =
                serde_json::from_str::<Response<D>>(&response_string)
            {
                (
                    Some(response.code()),
                    response.message().map(ToOwned::to_owned),
                )
            } else {
                (None, None)
            };
            return Err(SendRequest::BadStatusCode {
                status_code,
                response_string,
                code,
                message,
            });
        }

        if bare_data {
            Ok(serde_json::from_str::<D>(&response_string)?)
        } else {
            let response =
                serde_json::from_str::<Response<D>>(&response_string)?;
            Ok(response.data()?)
        }
    }
}
