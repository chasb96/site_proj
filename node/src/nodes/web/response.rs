use serde::Serialize;

#[derive(Serialize)]
pub struct CreateNodeResponse {
    pub id: i32,
}

#[derive(Serialize)]
pub struct GetNodeResponse {
    pub id: i32,
    pub name: String,
    pub host: String,
    pub port: u16,
}