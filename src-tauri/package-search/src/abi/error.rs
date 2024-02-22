#[derive(Debug, thiserror::Error)]
pub enum PackError {
    #[error("Database error: {0}")]
    DbError(#[from] sqlx::Error),
}
