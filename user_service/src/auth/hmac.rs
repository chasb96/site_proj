use std::{sync::OnceLock, fmt::Display, error::Error};

use hmac::{Hmac, Mac, digest::InvalidLength};
use sha2::Sha256;

use crate::config::AuthenticationConfig;

static HMAC_KEY: OnceLock<String> = OnceLock::new();

pub fn initialize_hmac_key(config: &AuthenticationConfig) {
    _ = HMAC_KEY
        .get_or_init(|| {
            config.hmac_key.clone()
        })
}

#[derive(Debug)]
pub struct InvalidKeyError(InvalidLength);

impl Error for InvalidKeyError { }

impl Display for InvalidKeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<InvalidLength> for InvalidKeyError {
    fn from(value: InvalidLength) -> Self {
        Self(value)
    }
}

pub fn get_hmac_hasher() -> Result<Hmac<Sha256>, InvalidKeyError> {
    let key = HMAC_KEY
        .get()
        .expect("HMAC Key used before initialization");

    let hasher: Hmac<Sha256> = Hmac::new_from_slice(key.as_bytes())?;

    Ok(hasher)
}