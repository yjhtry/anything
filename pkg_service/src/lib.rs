mod abi;
mod pkg;

pub use abi::{types, PkgError};
pub use pkg::{DbSync, Pkg};
use sqlx::{Database, Pool, Sqlite};

pub struct PackManager<T: Database = Sqlite> {
    pool: Pool<T>,
}

impl<T: Database> PackManager<T> {
    pub fn new(pool: Pool<T>) -> Self {
        Self { pool }
    }
}
