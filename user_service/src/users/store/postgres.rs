use diesel::{deserialize::Queryable, Selectable, prelude::Insertable, SelectableHelper, RunQueryDsl, ExpressionMethods, QueryDsl, result::Error::NotFound};
use crate::{data_store::postgres::{PostgresDataStore, users}, users::User};
use super::{UserStore, error::{CreateUserError, GetUserError, DeleteUserError, GetPasswordError}};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::data_store::postgres::users)]
struct UserModel {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub admin: bool,
}

#[derive(Insertable)]
#[diesel(table_name = crate::data_store::postgres::users)]
struct NewUserModel {
    pub username: String,
    pub password_hash: String,
}

impl UserStore for PostgresDataStore {
    async fn create(&self, username: String, password_hash: String) -> Result<User, CreateUserError> {
        let user_entity = {
            let mut conn = self.connection_pool
                .get()?;

            diesel::insert_into(users::table)
                .values(
                    NewUserModel {
                        username,
                        password_hash,
                    }
                )
                .returning(UserModel::as_returning())
                .get_result(&mut conn)?
        };

        let user = User {
            id: user_entity.id,
            username: user_entity.username,
            admin: user_entity.admin,
        };

        Ok(user)
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<User>, GetUserError> {
        let query_result = {
            let mut conn = self.connection_pool
                .get()?;

            users::table
                .filter(users::id.eq(id))
                .select(UserModel::as_select())
                .get_result(&mut conn)
        };

        let user_entity = match query_result {
            Ok(user) => user,
            Err(NotFound) => return Ok(None),
            Err(e) => return Err(e.into()),
        };

        let user = User {
            id: user_entity.id,
            username: user_entity.username,
            admin: user_entity.admin,
        };

        Ok(Some(user))
    }

    async fn get_by_username<'a>(&self, username: &'a str) -> Result<Option<User>, GetUserError> {
        let query_result = {
            let mut conn = self.connection_pool
                .get()?;

            users::table
                .filter(users::username.eq(&username))
                .select(UserModel::as_select())
                .get_result(&mut conn)
        };

        let user_entity = match query_result {
            Ok(user) => user,
            Err(NotFound) => return Ok(None),
            Err(e) => return Err(e.into()),
        };

        let user = User {
            id: user_entity.id,
            username: user_entity.username,
            admin: user_entity.admin,
        };

        Ok(Some(user))
    }

    async fn delete(&self, id: i32) -> Result<bool, DeleteUserError> {
        let delete_count = {
            let mut conn = self.connection_pool
                .get()?;

            diesel::delete(users::table)
                .filter(users::id.eq(id))
                .execute(&mut conn)?
        };

        Ok(delete_count == 1)
    }

    async fn get_password_hash(&self, id: i32) -> Result<Option<String>, GetPasswordError> {
        let query_result = {
            let mut conn = self.connection_pool
                .get()?;

            users::table
                .filter(users::id.eq(id))
                .select(UserModel::as_select())
                .get_result(&mut conn)
        };

        let user_entity = match query_result {
            Ok(user) => user,
            Err(NotFound) => return Ok(None),
            Err(e) => return Err(e.into()),
        };

        Ok(Some(user_entity.password_hash))
    }
}