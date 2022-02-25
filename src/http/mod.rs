//! HTTP related section of the API is implemented under this module.
//!
//! In this section, there is an HTTP [`Client`] which requests a certain data
//! from [`public`]/[`private`] endpoints. The client needs to authenticate by
//! signing its request with [`ApiKeys`] to make use of the private endpoints.

mod request;
pub(crate) use request::Request;

mod response;
pub(crate) use response::Response;

pub mod private;
pub mod public;

mod client;
pub use client::Client;

mod api_keys;
pub use api_keys::ApiKeys;

mod order_type;
pub use order_type::OrderType;

mod order_method;
pub use order_method::OrderMethod;

mod order_status;
pub use order_status::OrderStatus;
