use std::{error::Error, fmt::Display};
use jwt::Claims;

pub mod store;
pub mod web;

pub struct User {
    id: i32,
    username: String,
}

#[derive(Debug)]
pub enum UserClaimError {
    ClaimMissing(String),
    InvalidFormat(String),
}

impl Error for UserClaimError { }

impl Display for UserClaimError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserClaimError::ClaimMissing(claim) => write!(f, "Missing {} claim", claim),
            UserClaimError::InvalidFormat(claim) => write!(f, "Invalid claim {}", claim),
        }
    }
}

impl TryFrom<Claims> for User {
    type Error = UserClaimError;

    fn try_from(claims: Claims) -> Result<Self, Self::Error> {
        let mut id = None;
        let mut uname = None;

        for (key, value) in claims.private {
            match key.as_str() {
                "user_id" => {
                    if !value.is_number() {
                        return Err(UserClaimError::InvalidFormat("user_id".to_string()))
                    }

                    id = Some(
                        value
                            .as_i64()
                            .unwrap() as i32
                    );
                },
                "username" => {
                    if !value.is_string() {
                        return Err(UserClaimError::InvalidFormat("username".to_string()))
                    }

                    uname = Some(
                        value
                            .as_str()
                            .unwrap()
                            .to_string()
                    );
                },
                _ => {},
            }
        }

        if id.is_none() { return  Err(UserClaimError::ClaimMissing("user_id".to_string())) }
        if uname.is_none() { return  Err(UserClaimError::ClaimMissing("user_id".to_string())) }

        Ok(
            User {
                id: id.unwrap(),
                username: uname.unwrap(),
            }
        )
    }
}