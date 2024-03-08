use rfd::AsyncFileDialog;
use std::{
    collections::HashMap,
    ffi::OsStr,
    path::{Path, PathBuf},
};
use walkdir::{DirEntry, WalkDir};

mod error;
mod ext;
pub use error::OssError;
pub use ext::*;

/// Asynchronously opens a file dialog for the user to pick a file.
/// Returns the path of the selected file, or `None` if no file was selected.
pub async fn pick_file() -> Option<PathBuf> {
    AsyncFileDialog::new()
        .pick_file()
        .await
        .map(|file| file.path().to_owned())
}

/// Asynchronously moves a file from one location to another.
/// Returns the number of bytes copied, or an `OssError` if the operation fails.
pub async fn move_file(from: &PathBuf, to: &Path) -> Result<u64, OssError> {
    match from.file_name() {
        None => Err(OssError::FileEmpty),
        Some(file_name) => {
            let file_path = to.join(file_name);
            tokio::fs::copy(from, file_path)
                .await
                .map_err(OssError::MoveFileError)
        }
    }
}

/// Checks if a directory entry is hidden.
/// Returns `true` if the entry is hidden, `false` otherwise.
pub fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

/// Retrieves the directory tree structure of a given directory.
/// Returns a `HashMap` where the keys are folder names and the values are lists of file names.
/// Returns an `OssError` if there was an error while traversing the directory.
pub fn get_oss_tree(dir: &Path) -> Result<HashMap<String, Vec<String>>, OssError> {
    let mut tree = HashMap::new();

    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
    {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if let Some(folder) = path.file_name().and_then(OsStr::to_str) {
                tree.insert(folder.to_string(), vec![]);
            }
        } else if let Some(parent_path) = path.parent() {
            if let Some(parent_name) = parent_path.file_name().and_then(OsStr::to_str) {
                if let Some(file_name) = path.file_name().and_then(OsStr::to_str) {
                    tree.entry(parent_name.to_string())
                        .or_insert(vec![])
                        .push(file_name.to_string());
                }
            }
        }
    }

    Ok(tree)
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

    #[tokio::test]
    async fn is_hidden_should_work() {
        let dir = TempDir::new("hidden_test_temp_dir").unwrap();
        let entry = WalkDir::new(dir.path())
            .into_iter()
            .next()
            .unwrap()
            .unwrap();

        assert!(!is_hidden(&entry));

        let hidden = dir.path().join(".hidden");
        println!("{:?}", hidden.to_str());
        File::create(&hidden).await.unwrap();

        let entry = WalkDir::new(dir.path())
            .into_iter()
            .filter(|e| is_hidden(e.as_ref().unwrap()))
            .next()
            .unwrap()
            .unwrap();

        println!("{:?}", entry.file_name());

        assert_eq!(entry.file_name().to_str().unwrap(), ".hidden");
    }

    #[tokio::test]
    async fn get_oss_tree_should_work() {
        let dir = TempDir::new("get_oss_tree_test_temp_dir").unwrap();
        let file = dir.path().join("foo.txt");
        let file2 = dir.path().join("bar.txt");
        let sub_dir = dir.path().join("sub");
        let sub_file = sub_dir.join("sub_foo.txt");

        let mut f = File::create(&file).await.unwrap();
        f.write_all(b"Hello, world!").await.unwrap();
        f.sync_all().await.unwrap();

        let mut f = File::create(&file2).await.unwrap();
        f.write_all(b"Hello, world!").await.unwrap();
        f.sync_all().await.unwrap();

        tokio::fs::create_dir(&sub_dir).await.unwrap();

        let mut f = File::create(&sub_file).await.unwrap();
        f.write_all(b"Hello, world!").await.unwrap();
        f.sync_all().await.unwrap();

        let tree = get_oss_tree(dir.path()).unwrap();

        println!("{:?}", tree);

        // Because tempdir creates a random directory, so we need to get the name of the directory
        let dir_name = dir.path().file_name().unwrap().to_str().unwrap();

        assert_eq!(tree.len(), 2);
        assert_eq!(tree.get(dir_name).unwrap().len(), 2);
        assert_eq!(tree.get("sub").unwrap().len(), 1);
    }
}
