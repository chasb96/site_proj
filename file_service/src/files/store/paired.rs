use crate::files::file::{NewFileMetadata, FileMetadata};
use super::{FileDataStore, FileStoreMeta, FileStoreBytes, error::CreateFileError};

pub struct Paired<T, S> {
    meta: T,
    bytes: S,
}

impl<T, S> FileDataStore for Paired<T, S>
where
    T: FileStoreMeta,
    S: FileStoreBytes
{
    async fn create<'a>(&self, file: &'a NewFileMetadata, data: bytes::Bytes) -> Result<i32, CreateFileError> {
        self.bytes.create(&file.internal_name, data).await?;

        self.meta.create(file).await
    }

    async fn get_metadata_by_id(&self, id: i32) -> Result<Option<FileMetadata>, super::error::GetFileError> {
        self.meta.get_by_id(id).await
    }

    async fn get_bytes_by_id(&self, id: i32) -> Result<Option<bytes::Bytes>, super::error::GetFileError> {
        self.bytes.get_by_id(id).await
    }

    async fn delete(&self, id: i32) -> Result<bool, super::error::DeleteFileError> {
        Ok(self.meta.delete(id).await? && self.bytes.delete(id).await?)
    }
}

impl<T, S> Default for Paired<T, S>
where 
    T: Default,
    S: Default,
{
    fn default() -> Self {
        Self { 
            meta: Default::default(), 
            bytes: Default::default(),
        }
    }
}