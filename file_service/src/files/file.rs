use std::{error::Error, fmt::Display, path::Path};

use axum::extract::multipart::Field;
use rand::{distributions::Distribution, Rng};
use sqlx::{postgres::PgRow, Row};

pub struct FileMetadata {
    pub id: i32,
    pub name: String,
    pub content_type: String,
    pub extension: String,
    pub internal_name: String,
}

pub struct NewFileMetadata {
    pub name: String,
    pub content_type: String,
    pub extension: Option<String>,
    pub internal_name: String,
}

impl From<PgRow> for FileMetadata {
    fn from(row: PgRow) -> Self {
        FileMetadata {
            id: row.get("id"),
            name: row.get("name"),
            content_type: row.get("content_type"),
            extension: row.get("extension"),
            internal_name: row.get("internal_name"),
        }
    }
}

#[derive(Debug)]
pub struct FieldMissingError(String);

impl Error for FieldMissingError { }

impl Display for FieldMissingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FieldMissing(\"{}\")", self.0)
    }
}

const CHARS: &'static [u8] = "0123456789abcdef".as_bytes();

struct InternalNameDistribution;

impl Distribution<char> for InternalNameDistribution {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> char {
        CHARS[rng.gen_range(0..16) as usize] as char
    }
}

impl<'a> TryFrom<&Field<'a>> for NewFileMetadata {
    type Error = FieldMissingError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        let name = Path::new(
            value.name().ok_or(FieldMissingError("name".to_string()))?
        );

        let file_name = name
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .ok_or(FieldMissingError("file_name".to_string()))?
            .to_string();

        let content_type = value
            .content_type()
            .ok_or(FieldMissingError("content_type".to_string()))?
            .to_string();

        let extension = name
            .extension()
            .and_then(|extension| extension.to_str())
            .map(|extension| extension.to_string());
    
        Ok(NewFileMetadata {
            name: file_name,
            content_type,
            extension,
            internal_name: rand::thread_rng()
                .sample_iter(&InternalNameDistribution)
                .take(64)
                .map(char::from)
                .collect(),
        })
    }
}