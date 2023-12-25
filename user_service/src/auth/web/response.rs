use serde::Serialize;

#[derive(Serialize)]
pub struct LoginResponse {
    pub jwt: String,
}