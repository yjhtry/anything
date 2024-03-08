// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod const_var;
mod db_init;
mod fs;
mod pkg;
mod settings;
mod utils;

use pkg_service::PackManager;
use settings::Settings;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let settings = Settings::new(&app);
            let pool = db_init::init(settings.clone()).expect("error while initializing database");

            app.manage(PackManager::new(pool));
            app.manage(settings);

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
            pkg::sync_data_to_postgres,
            fs::move_file_to_oss,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
