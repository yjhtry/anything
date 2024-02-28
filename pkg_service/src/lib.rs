mod abi;
mod pkg;

pub use abi::{types, PkgError};
pub use pkg::Pkg;
use sqlx::{Database, Pool};

pub struct PackManager<T: Database> {
    pool: Pool<T>,
}

impl<T: Database> PackManager<T> {
    pub fn new(pool: Pool<T>) -> Self {
        Self { pool }
    }
}
