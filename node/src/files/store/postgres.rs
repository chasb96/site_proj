use sqlx::{postgres::PgRow, Row};
use crate::{data_store::postgres::PostgresDatabase, files::{FileMetadata, NewFileMetadata}};
use super::{FileStoreMeta, error::{CreateFileError, GetFileError}};

impl FileStoreMeta for PostgresDatabase {
    async fn create<'a>(&self, file: &'a NewFileMetadata) -> Result<i32, CreateFileError> {
        const CREATE_QUERY: &'static str = r#"
            INSERT INTO files (name, content_type, extension, internal_name)
            VALUES ($1, $2, $3, $4)
            RETURNING id
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(CREATE_QUERY)
            .bind(&file.name)
            .bind(&file.content_type)
            .bind(&file.extension)
            .bind(&file.internal_name)
            .map(|row: PgRow| row.get("id"))
            .fetch_one(conn.as_mut())
            .await
            .map_err(Into::into)
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<FileMetadata>, GetFileError> {
        const GET_BY_ID_QUERY: &'static str = r#"
            SELECT id, name, content_type, extension, internal_name
            FROM files
            WHERE id = $1
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(GET_BY_ID_QUERY)
            .bind(id)
            .map(|row: PgRow| FileMetadata {
                id: row.get("id"),
                name: row.get("name"),
                content_type: row.get("content_type"),
                extension: row.get("extension"),
                internal_name: row.get("internal_name"),
            })
            .fetch_optional(conn.as_mut())
            .await
            .map_err(Into::into)
    }
}