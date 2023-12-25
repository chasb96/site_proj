use axum::{Router, routing::{get, post}};
use crate::{health, users, auth};

pub fn routes() -> Router {
    Router::new()
        .route("/health", get(health::health))
        .route("/authenticate", post(auth::web::routes::authenticate))
        .route("/users/create", post(users::web::routes::create_user))
        .route("/users/get", post(users::web::routes::get_user))
        .route("/users/delete", post(users::web::routes::delete_user))
}