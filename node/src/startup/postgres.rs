use std::error::Error;

use diesel::{PgConnection, Connection, ConnectionError};
use diesel_migrations::{MigrationHarness, embed_migrations, EmbeddedMigrations};
use crate::app_state::AppState;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations/");

pub enum PostgresStartError {
    Connection(ConnectionError),
    // Migration(diesel_migrations::RunMigrationsError)
}

// impl From<RunMigrationsError> for PostgresStartError {
//     fn from(value: RunMigrationsError) -> Self {
//         Self::Migration(value)
//     }
// }

pub fn on_start(app_state: &AppState) -> Result<(), PostgresStartError> {
    let mut conn: PgConnection = Connection::establish(&app_state.config.database.connection_string).unwrap();

    EmbeddedMigrations::new().run_pending_migrations(conn)?;

    Ok(())
}