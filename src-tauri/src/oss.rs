use std::collections::HashMap;

use oss_service::{get_kind_from_path, move_file, pick_file, OssError, OssItem};
use tauri::{command, State};

use crate::{
    settings::Settings,
    utils::{get_app_folder, get_folder_path},
};

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
    let to = get_folder_path(&app_folder, format!("oss/{}", folder_name).as_str());

    move_file(&from, &to).await?;

    Ok(())
}

#[command]
pub async fn get_oss_tree(
    settings: State<'_, Settings>,
) -> Result<HashMap<String, Vec<OssItem>>, OssError> {
    let app_folder = get_app_folder();
    let oss_folder = get_folder_path(&app_folder, "oss");
    let host = settings.inner().value.oss_host.clone();
    let port = settings.inner().value.oss_port;
    let server = format!("http://{}:{}", host, port);

    oss_service::get_oss_tree(&oss_folder, server.as_str())
}
