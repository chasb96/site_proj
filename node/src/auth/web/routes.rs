use axum::{http::StatusCode, Json};
use crate::{auth::{jwt::generate_jwt, password::verify_password}, users::{web::UserStoreExtractor, store::UserStore}, util::or_status_code::OrInternalServerError};
use super::{request::LoginRequest, response::LoginResponse};

#[axum::debug_handler]
pub async fn authenticate(
    user_store: UserStoreExtractor,
    Json(request): Json<LoginRequest>
) -> Result<Json<LoginResponse>, StatusCode> {
    let user = user_store
        .get_by_username(&request.username)
        .await
        .or_internal_server_error()?
        .ok_or(StatusCode::BAD_REQUEST)?;

    let p_hash = user_store
        .get_password_hash(user.id)
        .await
        .or_internal_server_error()?
        .unwrap();

    if !verify_password(&request.password, &p_hash).or_internal_server_error()? {
        return Err(StatusCode::UNAUTHORIZED)
    }

    generate_jwt(user)
        .or_internal_server_error()
        .map(|token| Json(
            LoginResponse {
                jwt: token,
            }
        ))
}