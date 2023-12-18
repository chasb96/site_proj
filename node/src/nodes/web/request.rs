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
}