// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::incompatible_msrv)]

mod const_var;
mod db_init;
mod oss;
mod pkg;
mod settings;
mod utils;

use oss_service::OssService;
use pkg_service::PackManager;
use settings::Settings;
use tauri::Manager;

// TODO use a error state to manage all error during app initialization and show them in frontend

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let settings = Settings::new(app);
            let pool = db_init::init(settings.clone()).expect("error while initializing database");

            if !settings.value.oss_host.is_empty() && settings.value.oss_port > 0 {
                let host = settings.value.oss_host.clone();
                if let Ok(host) = host.parse() {
                    OssService::new(settings.oss_folder.clone(), host, settings.value.oss_port)
                        .start();
                }
            }

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
            oss::move_file_to_oss,
            oss::get_oss_tree,
            settings::get_app_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
