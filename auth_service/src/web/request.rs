use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct SetPasswordRequest {
    pub id: i32,
    pub password: String,
}

#[derive(Deserialize)]
pub struct VerifyJwtRequest {
    pub jwt: String,
}