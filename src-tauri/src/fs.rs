use oss::{get_kind_from_path, move_file, pick_file, OssError};
use tauri::command;

use crate::utils::{get_app_folder, get_folder_path};

#[command]
pub async fn move_file_to_oss() -> Result<(), OssError> {
    let from = pick_file().await;

    if from.is_none() {
        return Err(OssError::MoveFileError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No file picked",
        )));
    }

    let from = from.unwrap();

    let app_folder = get_app_folder();
    let folder_name = get_kind_from_path(&from);
    let to = get_folder_path(&app_folder, folder_name);

    move_file(&from, &to).await?;

    Ok(())
}
