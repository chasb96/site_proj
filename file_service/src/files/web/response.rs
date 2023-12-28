use serde::Serialize;

#[derive(Serialize)]
pub struct CreateFileResponse {
    pub id: i32,
}