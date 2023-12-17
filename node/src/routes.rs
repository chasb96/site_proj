use axum::{Router, routing::{get, post}};
use crate::{health, app_state::AppState, users};

pub fn routes(app_state: AppState) -> Router {
    Router::new()
        .route("/health", get(health::health))
        .route("/users/create", post(users::web::create_user))
        .route("/users/get", post(users::web::get_user))
        .route("/users/delete", post(users::web::delete_user))
        .with_state(app_state)
}