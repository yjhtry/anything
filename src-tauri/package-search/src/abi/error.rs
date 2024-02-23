#[derive(Debug, thiserror::Error)]
pub enum PkgError {
    #[error("Database error: {0}")]
    DbError(#[from] sqlx::Error),

    #[error("Not found package")]
    NotFoundPackage,

    #[error("Not found package category")]
    NotFoundCategory,
}
