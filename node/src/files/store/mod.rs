use bytes::Bytes;
use self::error::CreateFileError;
use super::File;

pub mod error;
pub mod paired;
mod postgres;
mod disk;

pub trait FileDataStore {
    async fn create<'a>(&self, name: &'a str, data: Bytes) -> Result<File, CreateFileError>;
}

trait FileStoreMeta {
    async fn create<'a>(&self, name: &'a str) -> Result<File, CreateFileError>;
}

trait FileStoreBytes {
    async fn create<'a>(&self, name: &'a str, data: Bytes) -> Result<(), CreateFileError>;
}