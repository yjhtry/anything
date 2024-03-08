#[derive(thiserror::Error, Debug)]
pub enum OssError {
    #[error("Failed to move file: {0}")]
    MoveFileError(#[from] std::io::Error),
}

impl serde::Serialize for OssError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            e => serializer.serialize_str(e.to_string().as_str()),
        }
    }
}
