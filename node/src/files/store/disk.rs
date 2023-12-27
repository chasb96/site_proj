use std::{fs::File, io::{Write, ErrorKind}, io::Read};
use bytes::Bytes;
use crate::data_stores::files::DiskDataStore;
use super::{FileStoreBytes, error::GetFileError};

impl FileStoreBytes for DiskDataStore {
    async fn create<'a>(&self, id: &'a str, data: bytes::Bytes) -> Result<(), super::error::CreateFileError> {
        File::create(format!("{}/{}/0", self.directory, id))
            .and_then(|mut file| file.write_all(&data))
            .map_err(Into::into)
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<bytes::Bytes>, GetFileError> {
        let file = match File::open(format!("{}/{}/0", self.directory, id)) {
            Ok(file) => file,
            Err(e) => match e.kind() {
                ErrorKind::NotFound => return Ok(None),
                _ => return Err(GetFileError::from(e)),
            },
        };

        // We've already been able to open this file,
        //  fair to assume we can read from the thing
        //  we're reading from. 
        let byte_stream = file.bytes()
            .into_iter()
            .map(|byte| byte.unwrap());

        let bytes = Bytes::from_iter(byte_stream);

        Ok(Some(bytes))
    }
}