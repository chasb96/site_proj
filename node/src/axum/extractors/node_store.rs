use axum::{extract::FromRequestParts, http::{StatusCode, request::Parts}, async_trait};
use url::Host;
use crate::{data_store::postgres::PostgresDataStore, nodes::{Node, store::{error::{CreateNodeError, GetNodeError}, NodeStore}}};

pub struct NodeStoreExtractor(PostgresDataStore);

impl NodeStore for NodeStoreExtractor {
    async fn create(&self, name: String, host: Host, port: u16) -> Result<Node, CreateNodeError> {
        self.0.create(name, host, port).await
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<Node>, GetNodeError> {
        self.0.get_by_id(id).await
    }

    async fn get_by_name<'a>(&self, name: &'a str) -> Result<Option<Node>, GetNodeError> {
        self.0.get_by_name(name).await
    }

    async fn get_by_address<'a>(&self, host: &'a Host, port: u16) -> Result<Option<Node>, GetNodeError> {
        self.0.get_by_address(host, port).await
    }
}

impl Default for NodeStoreExtractor {
    fn default() -> Self {
        NodeStoreExtractor(PostgresDataStore::default())
    }
}

#[async_trait]
impl<T> FromRequestParts<T> for NodeStoreExtractor {
    type Rejection = StatusCode;

    async fn from_request_parts<'a, 'b>(_: &'a mut Parts, _: &'b T) ->  Result<Self,Self::Rejection> {
        Ok(NodeStoreExtractor::default())
    }
}