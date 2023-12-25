use axum::{http::StatusCode, Json};
use crate::{auth::{jwt::generate_jwt, password::verify_password, claims_user::ClaimsUser}, users::store::UserStore, util::or_status_code::{OrInternalServerError, OrBadRequest}, axum::extractors::user_store::UserStoreExtractor};
use super::{request::LoginRequest, response::LoginResponse};

pub async fn authenticate(
    user_store: UserStoreExtractor,
    Json(request): Json<LoginRequest>
) -> Result<Json<LoginResponse>, StatusCode> {
    let user = user_store
        .get_by_username(&request.username)
        .await
        .or_internal_server_error()?
        .or_bad_request()?;

    let p_hash = user_store
        .get_password_hash(user.id)
        .await
        .or_internal_server_error()?
        .unwrap();

    if !verify_password(&request.password, &p_hash).or_internal_server_error()? {
        return Err(StatusCode::UNAUTHORIZED)
    }

    generate_jwt(ClaimsUser::from(user))
        .or_internal_server_error()
        .map(|token| Json(
            LoginResponse {
                jwt: token,
            }
        ))
}