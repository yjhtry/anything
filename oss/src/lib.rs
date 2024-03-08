use rfd::AsyncFileDialog;
use std::path::{Path, PathBuf};

mod error;
mod ext;
pub use error::OssError;
pub use ext::*;

pub async fn pick_file() -> Option<PathBuf> {
    AsyncFileDialog::new()
        .pick_file()
        .await
        .map(|file| file.path().to_owned())
}

pub async fn move_file(from: &PathBuf, to: &Path) -> Result<u64, OssError> {
    let file_path = to.join(from.file_name().unwrap());

    tokio::fs::copy(from, file_path)
        .await
        .map_err(OssError::MoveFileError)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempdir::TempDir;
    use tokio::fs::File;
    use tokio::io::AsyncWriteExt;

    #[tokio::test]
    async fn move_file_should_work() {
        let from_dir = TempDir::new("from_test_temp_dir").unwrap();
        let from = from_dir.path().join("foo.txt");

        let to_dir = TempDir::new("to_test_temp_dir").unwrap();

        let mut f = File::create(&from).await.unwrap();

        f.write_all(b"Hello, world!").await.unwrap();
        f.sync_all().await.unwrap();

        let result = move_file(&from, &to_dir.path().to_owned()).await;

        assert!(result.is_ok());
    }
}
