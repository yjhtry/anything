mod category;
mod package;
mod relation;

pub use category::*;
pub use package::*;
pub use relation::*;

use sqlx::{sqlite::SqliteRow, FromRow, Row};

/// Query as Package
impl FromRow<'_, SqliteRow> for Package {
    fn from_row(row: &SqliteRow) -> Result<Self, sqlx::Error> {
        // in real word, reservation must have a start time
        let id = row.get("id");

        if id == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        let name = row.get("name");
        let description = row.get("description");
        let link = row.get("link");
        let category_ids: String = row.get("category_ids");
        let reason = row.get("reason");
        let created_at = row.get("created_at");
        let updated_at = row.get("updated_at");

        let categories = if category_ids.is_empty() {
            vec![]
        } else {
            category_ids
                .split(',')
                .map(|v| v.parse::<i64>().unwrap())
                .collect()
        };

        Ok(Self {
            id,
            name,
            description,
            link,
            categories,
            reason,
            created_at,
            updated_at,
        })
    }
}

/// Query as PackageCategory
impl FromRow<'_, SqliteRow> for PackageCategory {
    fn from_row(row: &SqliteRow) -> Result<Self, sqlx::Error> {
        // in real word, reservation must have a start time
        let id = row.get("id");

        if id == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        let name = row.get("name");
        let parent_id = row.get("parent_id");
        let created_at = row.get("created_at");
        let updated_at = row.get("updated_at");

        Ok(Self {
            id,
            name,
            parent_id,
            created_at,
            updated_at,
        })
    }
}

/// Query as PackageCategoryRelation
impl FromRow<'_, SqliteRow> for PackageCategoryRelation {
    fn from_row(row: &SqliteRow) -> Result<Self, sqlx::Error> {
        // in real word, reservation must have a start time
        let id = row.get("id");

        if id == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        let package_id = row.get("package_id");
        let category_id = row.get("category_id");
        let created_at = row.get("created_at");
        let updated_at = row.get("updated_at");

        Ok(Self {
            id,
            package_id,
            category_id,
            created_at,
            updated_at,
        })
    }
}
