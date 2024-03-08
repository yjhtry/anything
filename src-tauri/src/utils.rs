use std::path::{Path, PathBuf};

use crate::const_var::APP_NAME;

/// User's home directory. joined with APP_NAME like /home/user/APP_NAME
/// Returns the path to the application folder.
/// If the folder does not exist, it will be created.
pub fn get_app_folder() -> PathBuf {
    let app_folder = dirs::home_dir().unwrap().join(APP_NAME);

    if !app_folder.exists() {
        std::fs::create_dir(&app_folder).unwrap();
    }

    app_folder
}

/// Returns the path to a file within a given folder.
/// If the file does not exist, it will be created.
///
/// # Arguments
///
/// * `folder` - The folder where the file is located.
/// * `path` - The relative path to the file.
///
/// # Returns
///
/// The path to the file.
pub fn get_file_path(folder: &Path, path: &str) -> std::path::PathBuf {
    let file_path = folder.join(path);

    if !file_path.exists() {
        std::fs::File::create(&file_path).unwrap();
    }
    file_path
}

/// Returns the path to a folder within a given folder.
/// If the folder does not exist, it will be created.
///
/// # Arguments
///
/// * `folder` - The folder where the subfolder is located.
/// * `path` - The relative path to the subfolder.
///
/// # Returns
///
/// The path to the subfolder.
pub fn get_folder_path(folder: &Path, path: &str) -> std::path::PathBuf {
    let folder_path = folder.join(path);

    if !folder_path.exists() {
        std::fs::create_dir(&folder_path).unwrap();
    }
    folder_path
}
