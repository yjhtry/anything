mod abi;
mod pkg;

pub use abi::{types, PkgError};
pub use pkg::Pkg;
use sqlx::sqlite::SqlitePool;

pub struct PackManager {
    pool: SqlitePool,
}

impl PackManager {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}
