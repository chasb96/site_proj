use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetFileRequest {
    pub id: i32,
}

#[derive(Deserialize)]
pub struct DeleteFileRequest {
    pub id: i32,
}