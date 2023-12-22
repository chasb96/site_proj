use std::future::join;
use axum::{Json, http::StatusCode};
use url::Host;
use crate::{util::{or_status_code::{OrBadRequest, OrInternalServerError, OrNotFound}, invert::Invert}, axum::extractors::node_store::NodeStoreExtractor};
use super::{request::{CreateNodeRequest, GetNodeRequest}, response::{CreateNodeResponse, GetNodeResponse}};
use crate::nodes::store::NodeStore;

pub async fn create_node(
    node_store: NodeStoreExtractor,
    Json(request): Json<CreateNodeRequest>
) -> Result<Json<CreateNodeResponse>, StatusCode> {
    let parsed_host = Host::parse(&request.host).or_bad_request()?;
        
    let (name_exists, address_exists) = join!(
        node_store.get_by_name(&request.name), 
        node_store.get_by_address(&parsed_host, request.port)
    ).await;

    let (name_collision, address_collision) = (
        name_exists.or_internal_server_error()?.is_some(),
        address_exists.or_internal_server_error()?.is_some()
    );

    if name_collision || address_collision {
        return Err(StatusCode::BAD_REQUEST);
    }

    node_store
        .create(request.name, parsed_host, request.port)
        .await
        .or_internal_server_error()
        .map(|node| Json(
            CreateNodeResponse {
                id: node.id,
            }
        ))
}

pub async fn get_node(
    node_store: NodeStoreExtractor,
    Json(request): Json<GetNodeRequest>
) -> Result<Json<GetNodeResponse>, StatusCode> {
    let parsed_host = request
        .host
        .map(|host_string| Host::parse(&host_string))
        .invert()
        .or_internal_server_error()?;

    let node = match (request.id, request.name, parsed_host, request.port) {
        (Some(id), None, None, None) => node_store.get_by_id(id).await,
        (None, Some(name), None, None) => node_store.get_by_name(&name).await,
        (None, None, Some(host), Some(port)) => node_store.get_by_address(&host, port).await,
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    node.or_internal_server_error()
        .and_then(|ok| ok.or_not_found())
        .map(|node| Json(
            GetNodeResponse {
                id: node.id,
                name: node.name,
                host: node.host.to_string(),
                port: node.port,
            }
        ))
}