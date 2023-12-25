use super::{FileDataStore, FileStoreMeta, FileStoreBytes};

pub struct Paired<T, S> {
    meta: T,
    bytes: S,
}

impl<T, S> FileDataStore for Paired<T, S>
where
    T: FileStoreMeta,
    S: FileStoreBytes
{
    async fn create<'a>(&self, name: &'a str, data: bytes::Bytes) -> Result<crate::files::File, super::error::CreateFileError> {
        self.bytes.create(name, data).await?;

        self.meta.create(name).await
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