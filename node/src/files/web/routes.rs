use axum::{Json, http::StatusCode, extract::Multipart};
use crate::{axum::extractors::file_data_store::FileDataStoreExtractor, files::store::FileDataStore, util::or_status_code::{OrInternalServerError, OrBadRequest}};
use super::response::CreateFileResponse;

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

    let name = file
        .name()
        .or_bad_request()?
        .to_string();

    let bytes = file
        .bytes()
        .await
        .or_internal_server_error()?;
    
    file_data_store
        .create(&name, bytes)
        .await
        .or_internal_server_error()
        .map(|file| Json(
            CreateFileResponse {
                id: file.id,
            }
        ))
}