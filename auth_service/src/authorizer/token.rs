use std::sync::OnceLock;

use crate::config::AuthenticationConfig;

use super::Authorizer;

static TOKEN: OnceLock<String> = OnceLock::new();

pub fn initialize_token(config: &AuthenticationConfig) {
    TOKEN.get_or_init(|| config.token.to_string());
}

pub struct TokenAuthorizer {
    token: &'static str
}

impl Default for TokenAuthorizer {
    fn default() -> Self {
        Self { 
            token: TOKEN
                .get()
                .expect("Token used before initialization") 
        }
    }
}

impl Authorizer for TokenAuthorizer {
    async fn verify<'a>(&self, token: &'a str) -> bool {
        self.token == token
    }
}