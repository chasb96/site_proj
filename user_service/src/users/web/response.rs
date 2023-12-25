use serde::Serialize;

#[derive(Serialize)]
pub struct CreateUserResponse {
    pub id: i32,
}

#[derive(Serialize)]
pub struct GetUserResponse {
    pub id: i32,
    pub username: String,
}