pub mod web;
pub mod password;
mod hmac;
pub mod jwt;

use ::jwt::Claims;

use crate::{config::AuthenticationConfig, users::User};

pub fn on_start(config: &AuthenticationConfig) {
    hmac::initialize_hmac_key(config)
}