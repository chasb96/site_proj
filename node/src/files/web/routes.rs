use std::path::Path;
use axum::{Json, http::{StatusCode, Response, header::{CONTENT_TYPE, CONTENT_DISPOSITION}}, extract::Multipart, body::Body};
use rand::{distributions::Alphanumeric, Rng};
use crate::{axum::extractors::file_data_store::FileDataStoreExtractor, files::{store::FileDataStore, NewFileMetadata}, util::{or_status_code::{OrInternalServerError, OrBadRequest, OrNotFound}, invert::Invert}};
use super::{response::CreateFileResponse, request::GetFileRequest};

#[axum::debug_handler]
pub async fn create_file(
    file_data_store: FileDataStoreExtractor,
    mut multipart: Multipart
) -> Result<Json<CreateFileResponse>, StatusCode> { 
    let file = multipart
        .next_field()
        .await
        .or_internal_server_error()?
        .or_bad_request()?;

    let name = Path::new(
        file.name()
            .or_bad_request()?
    );

    let new_file = NewFileMetadata {
        name: name
            .file_name()
            .unwrap()
            .to_str()
            .or_bad_request()?
            .to_string(),
        content_type: file
            .content_type()
            .or_bad_request()?
            .to_string(),
        extension: name
            .extension()
            .map(|extension| extension.to_str().or_bad_request())
            .invert()?
            .map(|extension| extension.to_string()),
        internal_name: rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect(),
    };

    let bytes = file
        .bytes()
        .await
        .or_internal_server_error()?;
    
    file_data_store
        .create(&new_file, bytes)
        .await
        .or_internal_server_error()
        .map(|id| Json(
            CreateFileResponse {
                id,
            }
        ))
}

pub async fn get_file(
    file_data_store: FileDataStoreExtractor,
    Json(request): Json<GetFileRequest>
) -> Result<Response<Body>, StatusCode> {
    let metadata = file_data_store
        .get_metadata_by_id(request.id)
        .await
        .or_internal_server_error()?
        .or_not_found()?;

    let bytes = file_data_store
        .get_bytes_by_id(request.id)
        .await
        .or_internal_server_error()?
        .or_internal_server_error()?;

    Response::builder()
        .header(CONTENT_DISPOSITION, format!("attachment; filename=\"{}\"", metadata.name))
        .header(CONTENT_TYPE, metadata.content_type)
        .body(Body::from(bytes))
        .or_internal_server_error()
}