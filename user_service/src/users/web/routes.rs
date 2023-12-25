use axum::{Json, http::StatusCode};
use crate::{util::or_status_code::{OrInternalServerError, OrNotFound}, auth::password::generate_password_hash, users::store::UserStore, axum::extractors::{user_store::UserStoreExtractor, session::SessionExtractor}};
use super::{request::{CreateUserRequest, GetUserRequest, DeleteUserRequest}, response::{CreateUserResponse, GetUserResponse}};

pub async fn create_user(
    user_store: UserStoreExtractor, 
    Json(request): Json<CreateUserRequest>
) -> Result<Json<CreateUserResponse>, StatusCode> {
    let user_exists = user_store
        .get_by_username(&request.username)
        .await
        .or_internal_server_error()?
        .is_some();

    if user_exists {
        return Err(StatusCode::BAD_REQUEST);
    }

    let password_hash = generate_password_hash(&request.password)
        .or_internal_server_error()?;

    user_store
        .create(request.username, password_hash)
        .await
        .or_internal_server_error()
        .map(|user| Json(
            CreateUserResponse {
                id: user.id
            }
        ))
}

pub async fn get_user(
    user_store: UserStoreExtractor,
    Json(request): Json<GetUserRequest>
) -> Result<Json<GetUserResponse>, StatusCode> {
    let user = match (request.id, request.username) {
        (Some(id), None) => user_store.get_by_id(id).await,
        (None, Some(un)) => user_store.get_by_username(&un).await,
        _ => return Err(StatusCode::BAD_REQUEST)
    };

    user.or_internal_server_error()
        .and_then(|ok| ok.or_not_found())
        .map(|user| Json(
            GetUserResponse {
                id: user.id,
                username: user.username,
            }
        ))
}

pub async fn delete_user(
    session: SessionExtractor,
    user_store: UserStoreExtractor,
    Json(request): Json<DeleteUserRequest>
) -> Result<StatusCode, StatusCode> {
    if session.user.id != request.id {
        return Err(StatusCode::FORBIDDEN)
    }

    user_store
        .delete(request.id)
        .await
        .or_internal_server_error()
        .map(|delete_result| match delete_result {
            true => StatusCode::NO_CONTENT,
            false => StatusCode::NOT_FOUND,
        })
}