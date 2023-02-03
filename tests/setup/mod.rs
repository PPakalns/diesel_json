use std::sync::Once;

use diesel::pg::PgConnection;
use diesel::prelude::Connection;
use diesel_migrations::{FileBasedMigrations, MigrationHarness};

static INIT: Once = Once::new();

fn establish_connection() -> PgConnection {
    dotenv::dotenv().ok();

    let database_url = std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|e| panic!("Error connecting to {} {}", database_url, e))
}

/// Setup function that is only run once, even if called multiple times.
pub fn setup() {
    INIT.call_once(|| {
        let migrations = FileBasedMigrations::find_migrations_directory().unwrap();
        let mut connection = establish_connection();
        connection
            .run_pending_migrations(migrations)
            .expect("Migrations didn't succeed");
        connection.begin_test_transaction().unwrap();
    });
}

pub fn transaction_connection() -> PgConnection {
    let mut connection = establish_connection();
    // #[cfg(feature = "sqlite")]
    // connection.execute("PRAGMA foreign_keys = ON").unwrap();
    connection.begin_test_transaction().unwrap();
    connection
}
