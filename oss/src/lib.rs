use rfd::AsyncFileDialog;
use std::path::PathBuf;

pub async fn pick_file() -> Option<PathBuf> {
    AsyncFileDialog::new()
        .pick_file()
        .await
        .map(|file| file.path().to_owned())
}
