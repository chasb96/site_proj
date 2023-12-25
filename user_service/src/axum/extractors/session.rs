use axum::{async_trait, extract::FromRequestParts, http::{StatusCode, request::Parts}};
use log::error;
use crate::{util::or_status_code::OrBadRequest, auth::{jwt::verify_jwt, claims_user::{ClaimsUser, UserClaimError}}};

pub struct SessionExtractor {
    pub user: ClaimsUser,
}

#[async_trait]
impl<T> FromRequestParts<T> for SessionExtractor {
    type Rejection = StatusCode;

    async fn from_request_parts<'a, 'b>(parts: &'a mut Parts, _: &'b T) ->  Result<Self,Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .or_bad_request()?
            .to_str()
            .or_bad_request()?;

        let (auth_type, value) = auth_header
            .split_once(' ')
            .or_bad_request()?;

        match (auth_type.to_uppercase().as_str(), value.to_string()) {
            ("BEARER", value) => {
                let user = verify_jwt::<ClaimsUser, UserClaimError>(value)
                    .inspect_err(|e| error!("{:?}", e))
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

                Ok(
                    SessionExtractor {
                        user
                    }
                )
            },
            ("BASIC", _) => Err(StatusCode::NOT_IMPLEMENTED),
            _ => Err(StatusCode::NOT_IMPLEMENTED),
        }
    }
}