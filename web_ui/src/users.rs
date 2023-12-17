use axum::{extract::State, http::{Response, StatusCode, header::CONTENT_TYPE}, Form, response::Redirect};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::app_state::AppState;

pub async fn create_user(State(app_state): State<AppState>) -> Response<String> {
    let content = app_state
        .handlebars
        .render(
            "create_user", 
            &json!({})
        )
        .unwrap();

    Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, "text/html")
        .body(content)
        .unwrap()
}

#[derive(Deserialize)]
pub struct PostUserRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct CreateUserRequest {
    username: String,
    password: String,
}

#[derive(Deserialize)]
pub struct CreateUserResponse {
    id: i32,
}

pub async fn post_user(
    Form(request): Form<PostUserRequest>
) -> Redirect {
    let client = reqwest::Client::new();

    let res: CreateUserResponse = client.post("http://node/users/create")
        .json(&CreateUserRequest {
            username: request.username,
            password: request.password,
        })
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    Redirect::to(&format!("/{}", res.id))
}

// pub async fn view_user(State(app_state): State<AppState>) -> Response<String> {
//     let client = reqwest::Client::new();

//     let res: CreateUserResponse = client.get("http://node/users/create")
//         .json(&CreateUserRequest {
//             username: request.username,
//             password: request.password,
//         })
//         .send()
//         .await
//         .unwrap()
//         .json()
//         .await
//         .unwrap();

//     let content = app_state
//         .handlebars
//         .render(
//             "view_user", 
//             &json!({})
//         )
//         .unwrap();

//     Response::builder()
//         .status(StatusCode::OK)
//         .header(CONTENT_TYPE, "text/html")
//         .body(content)
//         .unwrap()
// }