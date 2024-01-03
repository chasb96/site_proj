use axum::{http::StatusCode, Json};
use crate::{axum::extractors::UserStoreExtractor, users::{UserStore, ClaimsUser}, util::or_status_code::{OrInternalServerError, OrBadRequest}, authorizer::password::{verify_password, generate_password_hash}, authorizer::jwt};
use super::{request::{LoginRequest, SetPasswordRequest, VerifyJwtRequest}, response::{LoginResponse, VerifyJwtResponse}};

pub async fn authenticate(
    user_store: UserStoreExtractor,
    Json(request): Json<LoginRequest>
) -> Result<Json<LoginResponse>, StatusCode> {
    let user = user_store
        .get_by_username(&request.username)
        .await
        .or_internal_server_error()?
        .or_bad_request()?;

    if !verify_password(&request.password, &user.password_hash).or_internal_server_error()? {
        return Err(StatusCode::UNAUTHORIZED)
    }

    jwt::generate_jwt(ClaimsUser::from(user))
        .or_internal_server_error()
        .map(|token| Json(
            LoginResponse {
                jwt: token,
            }
        ))
}

pub async fn verify_jwt(
    Json(request): Json<VerifyJwtRequest>,
) -> Result<Json<VerifyJwtResponse>, StatusCode> {
    jwt::verify_jwt(request.jwt)
        .map(|user: ClaimsUser| Json(VerifyJwtResponse {
            id: user.id,
        }))
        .map_err(|err| match err {
            jwt::ValidateJwtError::HmacKey(_) => StatusCode::INTERNAL_SERVER_ERROR,
            jwt::ValidateJwtError::Verify(_) => StatusCode::INTERNAL_SERVER_ERROR,
            jwt::ValidateJwtError::Claims(_) => StatusCode::UNAUTHORIZED,
        })
}

pub async fn set_password(
    user_store: UserStoreExtractor,
    Json(request): Json<SetPasswordRequest>
) -> Result<StatusCode, StatusCode> {
    let password_hash = generate_password_hash(&request.password)
        .or_internal_server_error()?;

    user_store
        .set_password_hash(request.id, &password_hash)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .or_internal_server_error()
}