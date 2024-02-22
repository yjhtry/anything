mod category;
mod package;
mod relation;

pub use category::*;
pub use package::*;
pub use relation::*;

use sqlx::{sqlite::SqliteRow, FromRow, Row};

impl FromRow<'_, SqliteRow> for Package {
    fn from_row(row: &SqliteRow) -> Result<Self, sqlx::Error> {
        // in real word, reservation must have a start time
        let id = row.get("id");
        let name = row.get("name");
        let description = row.get("description");
        let link = row.get("link");
        let category_ids: String = row.get("category_ids");
        let reason = row.get("reason");
        let created_at = row.get("created_at");
        let updated_at = row.get("updated_at");

        Ok(Self {
            id,
            name,
            description,
            link,
            categories: category_ids
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect(),
            reason,
            created_at,
            updated_at,
        })
    }
}

impl FromRow<'_, SqliteRow> for PackageCategory {
    fn from_row(row: &SqliteRow) -> Result<Self, sqlx::Error> {
        // in real word, reservation must have a start time
        let id = row.get("id");
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

impl FromRow<'_, SqliteRow> for PackageCategoryRelation {
    fn from_row(row: &SqliteRow) -> Result<Self, sqlx::Error> {
        // in real word, reservation must have a start time
        let id = row.get("id");
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
