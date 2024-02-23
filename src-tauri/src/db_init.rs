use sqlx::migrate::Migrator;
use sqlx::{Connection, SqliteConnection, SqlitePool};
use std::thread;

use crate::const_var::{APP_NAME, MIGRATIONS_FOLDER, PKG_DB_FILE};
use tauri::App;

pub fn init(app: &App) -> Result<SqlitePool, Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Runtime::new()?;
    let migrations_folder = app
        .handle()
        .path_resolver()
        .resource_dir()
        .unwrap()
        .join(MIGRATIONS_FOLDER);

    let pool = thread::spawn(move || {
        rt.block_on(async {
            let app_folder = dirs::home_dir().unwrap().join(APP_NAME);

            if !app_folder.exists() {
                std::fs::create_dir(&app_folder).unwrap();
            }

            let db_file = app_folder.clone().join(PKG_DB_FILE);

            if !db_file.exists() {
                std::fs::File::create(&db_file).unwrap();
            }

            let db_url = db_file.to_str().unwrap();

            let mut conn = SqliteConnection::connect(db_url).await.unwrap();

            let m = Migrator::new(migrations_folder).await.unwrap();

            m.run(&mut conn).await.unwrap();

            sqlx::sqlite::SqlitePool::connect(db_url).await.unwrap()
        })
    })
    .join()
    .unwrap();

    Ok(pool)
}
