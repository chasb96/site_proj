use axum::{async_trait, extract::FromRequestParts, http::{StatusCode, request::Parts}};
use crate::util::or_status_code::OrBadRequest;
use super::{token::TokenAuthorizer, Authorizer};

pub struct AuthExtractor;

#[async_trait]
impl<T> FromRequestParts<T> for AuthExtractor {
    type Rejection = StatusCode;

    async fn from_request_parts<'a, 'b>(parts: &'a mut Parts, _: &'b T) ->  Result<Self, Self::Rejection> {
        let (auth_type, value) = parts
            .headers
            .get("Authorization")
            .or_bad_request()?
            .to_str()
            .or_bad_request()?
            .split_once(' ')
            .or_bad_request()?;

        let authorized = match (auth_type.to_uppercase().as_str(), value.to_string()) {
            ("TOKEN", value) => TokenAuthorizer::default().verify(&value).await,
            ("BASIC", _) => return Err(StatusCode::NOT_IMPLEMENTED),
            _ => return Err(StatusCode::NOT_IMPLEMENTED),
        };

        if authorized {
            Ok(AuthExtractor)
        } else {
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}