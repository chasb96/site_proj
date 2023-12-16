use axum::{extract::State, http::{Response, StatusCode, header::CONTENT_TYPE}};
use serde_json::json;

use crate::app_state::AppState;

pub async fn index(State(app_state): State<AppState>) -> Response<String> {
    let content = app_state
        .handlebars
        .render(
            "index", 
            &json!({})
        )
        .unwrap();

    Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, "text/html")
        .body(content)
        .unwrap()
}