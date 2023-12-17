use diesel::{PgConnection, Connection};
use diesel_migrations::{MigrationHarness, embed_migrations, EmbeddedMigrations};
use crate::app_state::AppState;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations/");

pub fn on_start(app_state: &AppState) {
    let mut conn: PgConnection = Connection::establish(&app_state.config.database.connection_string).unwrap();

    conn.run_pending_migrations(MIGRATIONS).unwrap();
}