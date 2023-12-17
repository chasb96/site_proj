use axum::{Router, routing::{get, post}};

use crate::{index, app_state::AppState, assets, users};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(index::index))
        .route("/assets/:type/:file", get(assets::assets))
        .route("/users/create", get(users::create_user))
        .route("/users/create", post(users::post_user))
        // .route("/users/:id", get(users::view_user))
        .with_state(AppState::default())
}