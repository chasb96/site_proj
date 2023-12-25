use url::Host;
use self::error::{CreateNodeError, GetNodeError, DeleteNodeError};
use super::Node;

pub mod error;
mod postgres;

pub trait NodeStore {
    async fn create(&self, name: String, host: Host, port: u16) -> Result<Node, CreateNodeError>;

    async fn get_by_id(&self, id: i32) -> Result<Option<Node>, GetNodeError>;

    async fn get_by_name<'a>(&self, name: &'a str) -> Result<Option<Node>, GetNodeError>;

    async fn get_by_address<'a>(&self, host: &'a Host, port: u16) -> Result<Option<Node>, GetNodeError>;

    async fn delete(&self, id: i32) -> Result<bool, DeleteNodeError>;
}