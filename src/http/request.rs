//! General request implementation.

use rust_decimal::Decimal;
use serde_json::{Map, Number, Value};
use surf::{http::Method, Url};

#[derive(Debug, Clone)]
pub struct Request<'a> {
    pub endpoint: &'a Url,
    pub parameters: Parameters,
    pub method: Method,
    pub requires_auth: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parameters {
    json_root: Map<String, Value>,
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
            json_root: Map::new(),
        }
    }
}

impl Parameters {
    pub fn new() -> Self {
        Self::default()
    }

    pub const fn root(&self) -> &Map<String, Value> {
        &self.json_root
    }

    pub fn push_decimal(
        &mut self,
        name: impl Into<String>,
        value: Option<Decimal>,
    ) {
        self.push_string(name, value.map(|d| d.to_string()));
    }

    pub fn push_string(
        &mut self,
        name: impl Into<String>,
        value: Option<String>,
    ) {
        if let Some(value) = value {
            self.json_root.insert(name.into(), Value::String(value));
        }
    }

    pub fn push_object(
        &mut self,
        name: impl Into<String>,
        value: Option<impl Into<String>>,
    ) {
        if let Some(value) = value {
            self.json_root
                .insert(name.into(), Value::String(value.into()));
        }
    }

    pub fn push_number<D>(&mut self, name: impl Into<String>, value: Option<D>)
    where
        serde_json::Number: From<D>,
    {
        if let Some(value) = value {
            self.json_root
                .insert(name.into(), Value::Number(Number::from(value)));
        }
    }
}
