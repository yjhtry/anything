use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};
use tauri::{App, State};

use crate::{
    const_var::{PKG_DB_FILE, PKG_MIGRATIONS_FOLDER, SETTINGS_FILE},
    utils::{get_app_folder, get_file_path},
};

#[tauri::command]
pub fn get_app_settings(settings: State<'_, Settings>) -> SettingValue {
    settings.inner().value.clone()
}

/// Represents the values of the settings.
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct SettingValue {
    pub pkg_sync_url: String,
    pub oss_host: String,
    pub oss_port: u16,
}

/// Represents the application settings.
#[derive(Debug, Clone)]
pub struct Settings {
    pub pkg_migrations_folder: PathBuf,
    pub pkg_db_file: PathBuf,
    pub oss_folder: PathBuf,
    pub value: SettingValue,
}

impl Settings {
    /// Creates a new instance of `Settings`.
    ///
    /// # Arguments
    ///
    /// * `app` - The Tauri application instance.
    ///
    /// # Returns
    ///
    /// A new instance of `Settings`.
    pub fn new(app: &App) -> Self {
        let app_folder = get_app_folder();
        let rt_app_path = app.handle().path_resolver().resource_dir().unwrap();
        let mut content =
            fs::read_to_string(get_file_path(&app_folder, SETTINGS_FILE)).unwrap_or_default();

        if content.is_empty() {
            content = "{}".to_string();
        }

        let value = serde_json::from_str(remove_comments(content).as_str())
            .unwrap_or(SettingValue::default());

        Self {
            pkg_migrations_folder: rt_app_path.join(PKG_MIGRATIONS_FOLDER),
            pkg_db_file: get_file_path(&app_folder, PKG_DB_FILE),
            oss_folder: get_file_path(&app_folder, "oss"),
            value,
        }
    }

    /// Returns the path to the package database file.
    ///
    /// # Returns
    ///
    /// The path to the package database file as a string.
    pub fn get_pkg_db_file(&self) -> &str {
        self.pkg_db_file.to_str().unwrap()
    }

    /// Returns the path to the package migrations folder.
    ///
    /// # Returns
    ///
    /// The path to the package migrations folder as a string.
    pub fn get_pkg_migrations_folder(&self) -> &str {
        self.pkg_migrations_folder.to_str().unwrap()
    }
}

fn remove_comments(json: String) -> String {
    let lines: Vec<&str> = json
        .lines()
        .filter(|line| !line.trim().starts_with("//"))
        .collect();

    lines.join("\n")
}
