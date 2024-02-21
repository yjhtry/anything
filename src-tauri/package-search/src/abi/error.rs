#[derive(Debug, thiserror::Error)]
pub enum PackError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
