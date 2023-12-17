use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    #[allow(dead_code)]
    pub password: String,
}

#[derive(Deserialize)]
pub struct GetUserRequest {
    pub id: Option<i32>,
    pub username: Option<String>,
}

#[derive(Deserialize)]
pub struct DeleteUserRequest {
    pub id: i32,
}