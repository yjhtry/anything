use sqlx::migrate::Migrator;
use sqlx::{Connection, SqliteConnection, SqlitePool};
use std::path::PathBuf;
use std::thread;

use crate::settings::Settings;

/// Initializes the database connection pool and runs migrations.
///
/// # Arguments
///
/// * `settings` - The settings object containing the necessary configuration.
///
/// # Returns
///
/// Returns a `Result` containing the initialized `SqlitePool` or an error.
pub fn init(settings: Settings) -> Result<SqlitePool, Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Runtime::new()?;

    let pool = thread::spawn(move || {
        rt.block_on(async {
            let db_url = settings.get_pkg_db_file();

            let mut conn = SqliteConnection::connect(db_url).await.unwrap();

            let m = Migrator::new(PathBuf::from(settings.get_pkg_migrations_folder()))
                .await
                .unwrap();

            m.run(&mut conn).await.unwrap();

            sqlx::sqlite::SqlitePool::connect(db_url).await.unwrap()
        })
    })
    .join()
    .unwrap();

    Ok(pool)
}
