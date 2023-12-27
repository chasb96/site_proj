use axum::{Router, routing::{get, post}, extract::DefaultBodyLimit};
use crate::{health, nodes, files};

pub fn routes() -> Router {
    Router::new()
        .route("/health", get(health::health))
        .route("/nodes/create", post(nodes::web::routes::create_node))
        .route("/nodes/get", post(nodes::web::routes::get_node))
        .route("/files/create", post(files::web::routes::create_file))
        .route("/files/get", post(files::web::routes::get_file))
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024))
        // .route("/nodes/delete", post(nodes::web::routes::delete_node))
}