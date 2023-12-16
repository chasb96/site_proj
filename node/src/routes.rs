use axum::{Router, routing::get};

use crate::health;

pub fn routes() -> Router {
    Router::new()
        .route("/health", get(health::health))
}