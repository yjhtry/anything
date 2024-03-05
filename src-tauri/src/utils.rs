use std::path::PathBuf;

use crate::const_var::APP_NAME;

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
pub fn get_file_path(folder: PathBuf, path: &str) -> std::path::PathBuf {
    let file_path = folder.clone().join(path);

    if !file_path.exists() {
        std::fs::File::create(&file_path).unwrap();
    }
    file_path
}
