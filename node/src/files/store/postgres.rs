use diesel::{deserialize::Queryable, Selectable, prelude::Insertable, SelectableHelper, RunQueryDsl};
use crate::{data_store::postgres::{PostgresDatabase, files}, files::File};
use super::FileStoreMeta;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::data_store::postgres::files)]
struct FileModel {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::data_store::postgres::files)]
struct NewFileModel<'a> {
    pub name: &'a str,
}

impl FileStoreMeta for PostgresDatabase {
    async fn create<'a>(&self, name: &'a str) -> Result<crate::files::File, super::error::CreateFileError> {
        let file_entity = {
            let mut conn = self.connection_pool
                .get()?;

            diesel::insert_into(files::table)
                .values(
                    NewFileModel {
                        name: name,
                    }
                )
                .returning(FileModel::as_returning())
                .get_result(&mut conn)?
        };

        let node = File {
            id: file_entity.id,
            name: file_entity.name,
        };

        Ok(node)
    }
}