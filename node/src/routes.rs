use axum::{Router, routing::{get, post}};
use crate::{health, app_state::AppState, users, nodes};

pub fn routes(app_state: AppState) -> Router {
    Router::new()
        .route("/health", get(health::health))
        .route("/users/create", post(users::web::create_user))
        .route("/users/get", post(users::web::get_user))
        .route("/users/delete", post(users::web::delete_user))
        .route("/nodes/create", post(nodes::web::create_node))
        .route("/nodes/get", post(nodes::web::get_node))
        .with_state(app_state)
}