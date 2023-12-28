use bytes::Bytes;
use self::error::{CreateFileError, GetFileError, DeleteFileError};
use super::file::{NewFileMetadata, FileMetadata};

pub mod error;
pub mod paired;
mod postgres;
mod disk;

pub trait FileDataStore {
    async fn create<'a>(&self, file: &'a NewFileMetadata, data: Bytes) -> Result<i32, CreateFileError>;

    async fn get_metadata_by_id(&self, id: i32) -> Result<Option<FileMetadata>, GetFileError>;

    async fn get_bytes_by_id(&self, id: i32) -> Result<Option<Bytes>, GetFileError>;

    async fn delete(&self, id: i32) -> Result<bool, DeleteFileError>;
}

trait FileStoreMeta {
    async fn create<'a>(&self, file: &'a NewFileMetadata) -> Result<i32, CreateFileError>;

    async fn get_by_id(&self, id: i32) -> Result<Option<FileMetadata>, GetFileError>;

    async fn delete(&self, id: i32) -> Result<bool, DeleteFileError>;
}

trait FileStoreBytes {
    async fn create<'a>(&self, internal_name: &'a str, data: Bytes) -> Result<(), CreateFileError>;

    async fn get_by_id(&self, id: i32) -> Result<Option<Bytes>, GetFileError>;

    async fn delete(&self, id: i32) -> Result<bool, DeleteFileError>;
}