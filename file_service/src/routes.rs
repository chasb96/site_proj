use axum::{Router, routing::{get, post}, extract::DefaultBodyLimit};
use crate::{health, files};

pub fn routes() -> Router {
    Router::new()
        .route("/health", get(health::health))
        .route("/files/create", post(files::web::routes::create_file))
        .route("/files/get", post(files::web::routes::get_file))
        .route("/files/delete", post(files::web::routes::delete_file))
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024))
}