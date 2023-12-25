use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateFileRequest {
    pub name: String,
    pub content_base64: String,
}