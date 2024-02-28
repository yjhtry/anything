// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod const_var;
mod db_init;
mod pkg;

use pkg_service::PackManager;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let pool = db_init::init(app).expect("error while initializing database");
            app.manage(PackManager::new(pool));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            pkg::query_packages,
            pkg::add_package,
            pkg::update_package,
            pkg::update_package_categories,
            pkg::delete_package,
            pkg::get_package_by_id,
            pkg::add_category,
            pkg::query_categories,
            pkg::delete_category,
            pkg::update_category,
            pkg::sync_data_to_postgres
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
