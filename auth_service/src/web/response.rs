use serde::Serialize;

#[derive(Serialize)]
pub struct LoginResponse {
    pub jwt: String,
}

#[derive(Serialize)]
pub struct VerifyJwtResponse {
    pub id: i32,
}