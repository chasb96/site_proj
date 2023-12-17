use diesel::{PgConnection, r2d2::ConnectionManager};
use r2d2::Pool;
use crate::config::Config;

pub struct PostgresDataStore {
    pub connection_pool: Pool<ConnectionManager<PgConnection>>,
}

impl From<&Config> for PostgresDataStore {
    fn from(value: &Config) -> Self {
        let connection_string = value
            .database
            .connection_string
            .to_string();

        let manager = ConnectionManager::<PgConnection>::new(connection_string);

        let pool = Pool::builder()
            .build(manager)
            .unwrap();

        Self {
            connection_pool: pool
        }
    }
}

diesel::table! {
    users (id) {
        id -> Serial,
        username -> VarChar,
        password_hash -> VarChar,
    }
}