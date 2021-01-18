use std::sync::Once;

use diesel::pg::PgConnection;
use diesel::prelude::Connection;

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
        let migration_dir = diesel_migrations::find_migrations_directory().unwrap();
        diesel_migrations::run_pending_migrations_in_directory(
            &establish_connection(),
            &migration_dir,
            &mut std::io::sink(),
        )
        .expect("Migrations didn't succeed");
    });
}

pub fn transaction_connection() -> PgConnection {
    let result = establish_connection();
    // #[cfg(feature = "sqlite")]
    // result.execute("PRAGMA foreign_keys = ON").unwrap();
    result.begin_test_transaction().unwrap();
    result
}
