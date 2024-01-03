use crate::config::AuthenticationConfig;

mod token;
mod hmac;
pub mod jwt;
pub mod password;

pub fn initialize(config: &AuthenticationConfig) {
    token::initialize_token(config);
    hmac::initialize_hmac_key(config)
}

trait Authorizer {
    async fn verify<'a>(&self, token: &'a str) -> bool;
}