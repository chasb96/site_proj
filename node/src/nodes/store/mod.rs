use url::Host;
use self::error::{CreateNodeError, GetNodeError};
use super::Node;

mod error;
mod postgres;

pub trait NodeStore {
    async fn create(&self, name: String, host: Host, port: u16) -> Result<Node, CreateNodeError>;

    async fn get_by_id(&self, id: i32) -> Result<Option<Node>, GetNodeError>;

    async fn get_by_name(&self, name: String) -> Result<Option<Node>, GetNodeError>;

    async fn get_by_address(&self, host: Host, port: u16) -> Result<Option<Node>, GetNodeError>;
}