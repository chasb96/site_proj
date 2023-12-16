use axum::{Router, routing::get};

use crate::{index, app_state::AppState, assets};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(index::index))
        .route("/assets/:type/:file", get(assets::assets))
        .with_state(AppState::default())
}