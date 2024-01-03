use axum::{Router, routing::post};
use crate::web::routes::{authenticate, set_password, verify_jwt};

pub fn routes() -> Router {
    Router::new()
        .route("/authenticate", post(authenticate))
        .route("/verify_jwt", post(verify_jwt))
        .route("/set_password", post(set_password))
}