use crate::config::AuthenticationConfig;

pub mod axum;
mod token;

pub fn initialize(config: &AuthenticationConfig) {
    token::initialize(config)
}

trait Authorizer {
    async fn verify<'a>(&self, token: &'a str) -> bool;
}