use axum::{async_trait, extract::FromRequestParts, http::{StatusCode, request::Parts}};
use crate::{data_store::{files::DiskDataStore, postgres::PostgresDatabase}, files::{store::{paired::Paired, FileDataStore, error::CreateFileError}, File}};

pub struct FileDataStoreExtractor(Paired<PostgresDatabase, DiskDataStore>);

impl FileDataStore for FileDataStoreExtractor {
    async fn create<'a>(&self, name: &'a str, data: bytes::Bytes) -> Result<File, CreateFileError> {
        self.0.create(name, data).await
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