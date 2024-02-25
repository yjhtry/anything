#[derive(Debug, thiserror::Error)]
pub enum PkgError {
    #[error("Database error: {0}")]
    DbError(#[from] sqlx::Error),

    #[error("Not found package")]
    NotFoundPackage,

    #[error("Not found package category")]
    NotFoundCategory,

    #[error("Cannot delete has child category")]
    CannotDeleteHasChildCategory,
}

impl serde::Serialize for PkgError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            PkgError::DbError(e) => {
                serializer.serialize_str(format!("DbError: {}", e.to_string()).as_str())
            }
            e => serializer.serialize_str(e.to_string().as_str()),
        }
    }
}

pub fn default_page() -> i64 {
    1
}

pub fn default_page_size() -> i64 {
    10
}
