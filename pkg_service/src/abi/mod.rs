mod error;
pub mod types;

pub use error::*;

pub fn get_valid_pagination(page: i64, page_size: i64) -> (i64, i64) {
    let page = if page < 1 { 1 } else { page };
    let page_size = if page_size < 1 { 10 } else { page_size };

    (page, page_size)
}
