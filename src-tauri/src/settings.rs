use std::{fs, path::PathBuf};

use serde::Deserialize;
use tauri::App;

use crate::{
    const_var::{PKG_DB_FILE, PKG_MIGRATIONS_FOLDER, SETTINGS_FILE},
    utils::{get_app_folder, get_file_path},
};

/// Represents the values of the settings.
#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SettingValue {
    #[serde(default)]
    pub pkg_sync_url: String,
}

/// Represents the application settings.
#[derive(Debug, Clone)]
pub struct Settings {
    pub pkg_migrations_folder: PathBuf,
    pub pkg_db_file: PathBuf,
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
        let mut content = fs::read_to_string(get_file_path(app_folder.clone(), SETTINGS_FILE))
            .unwrap_or_default();

        if content.is_empty() {
            content = "{}".to_string();
        }

        let value = serde_json::from_str(remove_comments(content).as_str())
            .unwrap_or(SettingValue::default());

        Self {
            pkg_migrations_folder: rt_app_path.join(PKG_MIGRATIONS_FOLDER),
            pkg_db_file: get_file_path(app_folder.clone(), PKG_DB_FILE),
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