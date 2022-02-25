use std::{
    fmt::Display,
    hash::Hash,
    time::{SystemTime, SystemTimeError, UNIX_EPOCH},
};

use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::error;

/// Used for authentication. Pass this to [`Client`][super::Client] to be able
/// to use the private endpoints.
/// # Example
/// ```no_run
/// # use btcturk::ApiKeys;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let keys = ApiKeys::new("PUBLIC_KEY", "PRIVATE_KEY")?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct ApiKeys {
    public_key: String,
    private_key: String,
    mac: Hmac<Sha256>,
}

impl ApiKeys {
    /// Creates new API keys object by the given public/private keys.
    /// # Errors
    /// [`PrivateKey`][error::PrivateKey] error occurs if the private key length
    /// is invalid.
    pub fn new(
        public_key: impl Into<String>,
        private_key: impl Into<String>,
    ) -> Result<Self, error::PrivateKey> {
        let private_key = private_key.into();
        Ok(Self {
            public_key: public_key.into(),
            private_key: private_key.clone(),
            mac: Hmac::<Sha256>::new_from_slice(&base64::decode(
                &private_key,
            )?)?,
        })
    }

    /// Load API keys from a file path passed by `KEYS_PATH` environment var.
    /// The variable stores the path to the keys file which consist of two
    /// lines of text: Public key and secret key.
    ///
    /// Example: `KEYS_PATH={path} cargo test -- --ignored`
    #[cfg(test)]
    pub fn load_from_env_var() -> Self {
        if let Ok(path) = std::env::var("KEYS_PATH") {
            let key_str = std::fs::read_to_string(path).unwrap();
            let mut lines = key_str.lines();
            let public_key = lines.next().unwrap().to_owned();
            let private_key = lines.next().unwrap().to_owned();
            return Self::new(public_key, private_key).unwrap();
        }
        panic!(
            "KEYS_PATH environment var is missing. The key file must consist
            of public key + line break + secret key. Pass the path like this: \
            `KEYS_PATH={{path}} cargo test -- --ignored`"
        );
    }

    /// Get a reference to the public key.
    #[must_use]
    pub fn public_key(&self) -> &str {
        self.public_key.as_ref()
    }

    /// Get a reference to the private key.
    #[must_use]
    pub fn private_key(&self) -> &str {
        self.private_key.as_ref()
    }

    /// Sign the query part of a request's URL.
    /// # Errors
    /// [`SystemTimeError`] occurs if there is an error retrieving the current
    /// timestamp (_nonce_) of the system.
    /// # Returns
    /// Sign and nonce(timestamp) values in a tuple, respectively.
    pub(crate) fn generate_sign_nonce(
        &self,
    ) -> Result<(String, String), SystemTimeError> {
        let mut mac = self.mac.clone();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_millis()
            .to_string();
        mac.update((self.public_key.clone() + &timestamp).as_bytes());
        let signature: String = base64::encode(mac.finalize().into_bytes());
        Ok((signature, timestamp))
    }
}

impl Display for ApiKeys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Public Key: {}, Private Key: {}",
            self.public_key, self.private_key
        )
    }
}

impl Eq for ApiKeys {}

impl PartialEq for ApiKeys {
    fn eq(&self, other: &Self) -> bool {
        self.public_key == other.public_key
            && self.private_key == other.private_key
    }
}

impl Hash for ApiKeys {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.public_key.hash(state);
        self.private_key.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use hmac::{Hmac, Mac};
    use log::info;
    use sha2::Sha256;

    use crate::http::ApiKeys;

    #[test]
    fn verify_sign() {
        let _ = env_logger::builder().is_test(true).try_init();

        // Randomly generated dummy keys.
        let public_key = "63762e79-cb5c-4c0b-b714-5f0ce94bf100".to_owned();
        let private_key = "L2tW3CeHzXH16im1pIhofRw0GdlqCdb8".to_owned();

        let keys =
            ApiKeys::new(public_key.clone(), private_key.clone()).unwrap();
        let (sign, nonce) = keys.generate_sign_nonce().unwrap();

        info!("sign: {}, nonce: {}", sign, nonce);

        let sign_bytes = base64::decode(sign).unwrap();

        info!("sign bytes: {:?}", sign_bytes);

        let mut mac = Hmac::<Sha256>::new_from_slice(
            &base64::decode(private_key).unwrap(),
        )
        .unwrap();
        mac.update((public_key.to_owned() + nonce.as_str()).as_bytes());
        mac.verify_slice(sign_bytes.as_slice()).unwrap();
    }
}
