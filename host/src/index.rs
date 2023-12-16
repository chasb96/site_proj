use axum::extract::State;
use serde_json::json;

use crate::app_state::AppState;

pub async fn index(State(app_state): State<AppState>) -> String {
    app_state
        .handlebars
        .render(
            "index", 
            &json!({})
        )
        .unwrap()
}