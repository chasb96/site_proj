use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetFileRequest {
    pub id: i32,
}