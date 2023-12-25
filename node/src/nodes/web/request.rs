use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateNodeRequest {
    pub name: String,
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct GetNodeRequest {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
}

#[derive(Deserialize)]
pub struct DeleteNodeRequest {
    pub id: i32
}