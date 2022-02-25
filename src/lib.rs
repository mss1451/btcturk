#![forbid(unsafe_code)]
#![deny(missing_docs, rustdoc::broken_intra_doc_links)]
#![warn(missing_copy_implementations, missing_debug_implementations)]
#![warn(clippy::all, clippy::pedantic)]
#![warn(clippy::nursery, clippy::cargo)]

//! Unofficial [`BtcTurk` exchange](https://www.btcturk.com/) API bindings.
//!
//! Use this crate to make API calls to [`public`][crate::http::public] and
//! [`private`][crate::http::private] endpoints. Websocket feed is not
//! implemented yet. This is an async crate and blocking calls are
//! not supported yet.
//!
//! This crate was made with the help of the following documents:
//! - <https://docs.btcturk.com/>
//! - <https://github.com/BTCTrader/broker-api-docs/blob/master/README-pro.md>
//!
//! # Examples
//! ## Get a ticker
//! ```no_run
//! use btcturk::Client;
//!
//! #[async_std::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Use `Client` to make API calls.
//!     // You may optionally pass API keys and a client identifier.
//!     // API keys are not needed for public endpoints.
//!     // Client identifier is passed as an additional parameter for API calls
//!     // which require it and is optional.
//!     let client = Client::new(None, None)?;
//!
//!     // This method will return a data structure if it succeeds.
//!     // If there is a network error or an error either in the parameters
//!     // or in the response, a wrapping error will be returned.
//!     let ticker = client.ticker("BTCTRY").await?;
//!
//!     println!("Last price of BTCTRY pair is {}", ticker.last);
//!
//!     Ok(())
//! }
//! ```
//! ## Submit and cancel an order
//! ```no_run
//! # #[async_std::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     use btcturk::{Client, ApiKeys};
//!     use rust_decimal_macros::dec;
//!
//!     // In this example, we are going to use the private endpoints so
//!     // we will need API keys.
//!     let keys = ApiKeys::new("PUBLIC_KEY", "PRIVATE_KEY")?;
//!     
//!     // We can pass the API keys here or set it later. For the sake of the
//!     // example, we pass the keys here and set the client identifier later.
//!     let mut client = Client::new(Some(keys), None)?;
//!     client.set_id(Some("test"));
//!
//!     // In financial applications, rounding errors in floating-point
//!     // arithmetic may not be acceptable. Instead of `f32` or `f64`, we use
//!     // `Decimal` types which are more suitable for such applications.
//!     let price = dec!(500000);
//!     let quantity = dec!(0.01);
//!     let new_order = client.limit_buy("BTCTRY", price, quantity).await?;
//!
//!     println!("New order with id {} has been submitted", new_order.id);
//!
//!     client.cancel_order(new_order.id).await?;
//!
//!     println!("New order with id {} has been cancelled", new_order.id);
//!
//!     # Ok(())
//! # }
//! ```
//! # Testing
//! There are plenty of tests but many of them have `ignored` attribute which
//! means just running `cargo test` command won't cause them to run. Such tests
//! are ignored because they require network connection and some of them even
//! require API keys. We will talk about how to run such tests.
//!
//! Run ignored tests individually as running all of them at once
//! might get your IP banned by exceeding the rate limit as documented in
//! <https://docs.btcturk.com/rate-limits>.
//! ## Testing endpoint
//! <https://api-dev.btcturk.com/> endpoint is used in test configuration.
//! Private API calls (e.g. buy, sell) won't take have real effect when this
//! base endpoint is used.
//!
//! **Important note:** Normal API keys won't work with the testing endpoint.
//! You must either take testing account API keys from
//! <https://pro-dev.btcturk.com/> as documented here in this [page], or, change
//! the `api-dev` part of the endpoint with just `api` in the source code
//! of `url_cache` module. \
//! Beware that if you use the normal endpoint, tests will submit/cancel
//! **real** orders.
//!
//! [page]: https://github.com/BTCTrader/broker-api-docs/blob/master/README-pro.md#testing
//! ## Logging
//! Logging is supported in the tests. Pass `RUST_LOG` environment variable
//! which is set to the logging level you desire (e.g. `RUST_LOG=trace`).
//!
//! Logs will normally be visible only when the test fails.
//! ```console
//! $ RUST_LOG=debug cargo test
//! ```
//! ## Testing public API calls
//! The following example will run `get_ohlc` test or any test including that
//! name.
//! ```console
//! $ cargo test get_ohlc -- --ignored
//! ```
//! ## Testing private API calls
//! Private endpoints require API keys so do the tests.
//! - Create a text file with name, say, `keys.txt`.
//! - Put your public key in the first line of the file.
//! - Put your private key in the second line of the file.
//! - Save and note down the path of the file.
//! - Pass `KEYS_PATH` environment variable to the cargo test which is set to
//! the file's path.
//! ```console
//! $ KEYS_PATH=~/keys.txt cargo test get_all_orders -- --ignored
//! ```

pub mod http;
pub use http::ApiKeys;
pub use http::Client;

pub mod websocket;

pub mod error;
pub use error::Parameter as ParameterError;
pub use error::Parse as ParseError;
pub use error::PrivateKey as PrivateKeyError;
pub use error::Response as ResponseError;
pub use error::SendRequest as SendRequestError;
