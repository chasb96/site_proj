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