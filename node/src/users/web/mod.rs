mod request;
mod response;

use axum::{extract::State, Json, http::StatusCode};
use crate::app_state::AppState;
use self::{request::{CreateUserRequest, GetUserRequest, DeleteUserRequest}, response::{CreateUserResponse, GetUserResponse}};
use super::store::UserStore;

pub async fn create_user(
    State(app_state): State<AppState>, 
    Json(request): Json<CreateUserRequest>
) -> Result<Json<CreateUserResponse>, StatusCode> {
    let user = app_state.user_store
        .create(request.username)
        .await;

    let response = match user {
        Ok(user) => CreateUserResponse {
            id: user.id
        },
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    Ok(Json(response))
}

pub async fn get_user(
    State(app_state): State<AppState>,
    Json(request): Json<GetUserRequest>
) -> Result<Json<GetUserResponse>, StatusCode> {
    let user = match (request.id, request.username) {
        (Some(id), None) => {
            app_state.user_store
                .get_by_id(id)
                .await
        }
        (None, Some(un)) => {
            app_state.user_store
                .get_by_username(un)
                .await
        },
        _ => return Err(StatusCode::BAD_REQUEST)
    };

    let response = match user {
        Ok(Some(user)) => GetUserResponse {
            id: user.id,
            username: user.username,
        },
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    Ok(Json(response))
}

pub async fn delete_user(
    State(app_state): State<AppState>,
    Json(request): Json<DeleteUserRequest>
) -> StatusCode {
    let delete_result = app_state.user_store
        .delete(request.id)
        .await;

    match delete_result {
        Ok(true) => StatusCode::NO_CONTENT,
        Ok(false) => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}