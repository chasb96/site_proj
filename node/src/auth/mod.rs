pub mod web;
pub mod password;
mod hmac;
pub mod jwt;

use crate::config::AuthenticationConfig;

pub fn on_start(config: &AuthenticationConfig) {
    hmac::initialize_hmac_key(config)
}