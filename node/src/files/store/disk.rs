use std::{fs::File, io::Write};

use crate::data_store::files::DiskDataStore;

use super::FileStoreBytes;

impl FileStoreBytes for DiskDataStore {
    async fn create<'a>(&self, name: &'a str, data: bytes::Bytes) -> Result<(), super::error::CreateFileError> {
        File::create(format!("{}/{}/0", self.directory, name))
            .and_then(|mut file| file.write_all(&data))
            .map(|_| ())
            .map_err(Into::into)
    }
}