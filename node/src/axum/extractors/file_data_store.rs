use axum::{async_trait, extract::FromRequestParts, http::{StatusCode, request::Parts}};
use bytes::Bytes;
use crate::{data_store::{files::DiskDataStore, postgres::PostgresDatabase}, files::{store::{paired::Paired, FileDataStore, error::{CreateFileError, GetFileError}}, FileMetadata, NewFileMetadata}};

pub struct FileDataStoreExtractor(Paired<PostgresDatabase, DiskDataStore>);

impl FileDataStore for FileDataStoreExtractor {
    async fn create<'a>(&self, file: &'a NewFileMetadata, data: bytes::Bytes) -> Result<i32, CreateFileError> {
        self.0.create(file, data).await
    }

    async fn get_metadata_by_id(&self, id: i32) -> Result<Option<FileMetadata>, GetFileError> {
        self.0.get_metadata_by_id(id).await
    }

    async fn get_bytes_by_id(&self, id: i32) -> Result<Option<Bytes>, GetFileError> {
        self.0.get_bytes_by_id(id).await
    }
}

impl Default for FileDataStoreExtractor {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[async_trait]
impl<T> FromRequestParts<T> for FileDataStoreExtractor {
    type Rejection = StatusCode;

    async fn from_request_parts<'a, 'b>(_: &'a mut Parts, _: &'b T) -> Result<Self, Self::Rejection> {
        Ok(FileDataStoreExtractor::default())
    }
}