use std::sync::OnceLock;

use crate::config::AuthenticationConfig;

static HMAC_KEY: OnceLock<String> = OnceLock::new();

pub fn initialize_hmac_key(config: &AuthenticationConfig) {
    _ = HMAC_KEY
        .get_or_init(|| {
            config.hmac_key.clone()
        })
}