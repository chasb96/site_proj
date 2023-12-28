use axum::{Json, http::{StatusCode, Response, header::{CONTENT_TYPE, CONTENT_DISPOSITION}}, extract::Multipart, body::Body};
use crate::{util::or_status_code::{OrInternalServerError, OrBadRequest, OrNotFound}, files::{axum::FileDataStoreExtractor, file::NewFileMetadata, store::FileDataStore}, auth::axum::AuthExtractor};
use super::{response::CreateFileResponse, request::{GetFileRequest, DeleteFileRequest}};

pub async fn create_file(
    _: AuthExtractor,
    file_data_store: FileDataStoreExtractor,
    mut multipart: Multipart
) -> Result<Json<CreateFileResponse>, StatusCode> { 
    let file = multipart
        .next_field()
        .await
        .or_internal_server_error()?
        .or_bad_request()?;

    let file_metadata = NewFileMetadata::try_from(&file)
        .or_bad_request()?;

    let bytes = file
        .bytes()
        .await
        .or_internal_server_error()?;

    file_data_store
        .create(&file_metadata, bytes)
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

pub async fn delete_file(
    _: AuthExtractor,
    file_data_store: FileDataStoreExtractor,
    Json(request): Json<DeleteFileRequest>
) -> Result<StatusCode, StatusCode> {
    file_data_store
        .delete(request.id)
        .await
        .or_internal_server_error()?;

    Ok(StatusCode::NO_CONTENT)
}