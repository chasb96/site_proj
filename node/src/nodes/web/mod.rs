mod request;
mod response;

use axum::{Json, http::StatusCode, extract::State};
use log::error;
use url::Host;
use crate::app_state::AppState;
use self::{request::{CreateNodeRequest, GetNodeRequest}, response::{CreateNodeResponse, GetNodeResponse}};

use super::store::NodeStore;

pub async fn create_node(
    State(app_state): State<AppState>,
    Json(request): Json<CreateNodeRequest>
) -> Result<Json<CreateNodeResponse>, StatusCode> {
    let parsed_host = Host::parse(&request.host)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let node = app_state.node_store
        .create(request.name, parsed_host, request.port)
        .await;

    let response = match node {
        Ok(node) => CreateNodeResponse {
            id: node.id,
        },
        Err(e) => {
            error!("{:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    };

    Ok(Json(response))
}

pub async fn get_node(
    State(app_state): State<AppState>,
    Json(request): Json<GetNodeRequest>
) -> Result<Json<GetNodeResponse>, StatusCode> {
    let parsed_host = match request.host {
        Some(host_string) => Some(
            Host::parse(&host_string)
                .map_err(|_| StatusCode::BAD_REQUEST)?
        ),
        None => None,
    };

    let node = match (request.id, request.name, parsed_host, request.port) {
        (Some(id), None, None, None) => {
            app_state.node_store
                .get_by_id(id)
                .await
        },
        (None, Some(name), None, None) => {
            app_state.node_store
                .get_by_name(name)
                .await
        },
        (None, None, Some(host), Some(port)) => {
            app_state.node_store
                .get_by_address(host, port)
                .await
        },
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    let response = match node {
        Ok(Some(node)) => GetNodeResponse {
            id: node.id,
            name: node.name,
            host: node.host.to_string(),
            port: node.port,
        },
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(e) => {
            error!("{:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    };

    Ok(Json(response))
}