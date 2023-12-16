use axum::{Router, routing::get};

use crate::{index, app_state::AppState};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(index::index))
        .with_state(AppState::default())
}