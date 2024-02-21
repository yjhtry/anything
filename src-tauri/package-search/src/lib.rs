pub mod abi;
pub mod pack;

use sqlx::sqlite::SqlitePool;

pub struct PackManager {
    pub pool: SqlitePool,
}

impl PackManager {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}
