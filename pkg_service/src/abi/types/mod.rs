mod category;
mod del_record;
mod package;
mod relation;

pub use category::*;
use chrono::NaiveDateTime;
pub use del_record::*;
pub use package::*;
pub use relation::*;

use sqlx::{postgres::PgRow, sqlite::SqliteRow, FromRow, Row};

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
        let synced = row.get("synced");
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
            synced,
            created_at,
            updated_at,
        })
    }
}

impl FromRow<'_, SqliteRow> for PackageWithOutCategories {
    fn from_row(row: &SqliteRow) -> Result<Self, sqlx::Error> {
        // in real word, reservation must have a start time
        let id = row.get("id");

        if id == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        let name = row.get("name");
        let description = row.get("description");
        let link = row.get("link");
        let reason = row.get("reason");
        let synced = row.get("synced");
        let created_at = row.get("created_at");
        let updated_at = row.get("updated_at");

        Ok(Self {
            id,
            name,
            description,
            link,
            reason,
            synced,
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
        let synced = row.get("synced");
        let created_at = row.get("created_at");
        let updated_at = row.get("updated_at");

        Ok(Self {
            id,
            name,
            parent_id,
            synced,
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
        let synced = row.get("synced");
        let created_at = row.get("created_at");
        let updated_at = row.get("updated_at");

        Ok(Self {
            id,
            package_id,
            category_id,
            synced,
            created_at,
            updated_at,
        })
    }
}

/// Query as Package
impl FromRow<'_, PgRow> for PackageIdAndUpdatedAt {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        // in real word, reservation must have a start time
        let id = row.get("id");

        if id == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        let updated_at: NaiveDateTime = row.get("updated_at");

        Ok(Self {
            id,
            updated_at: updated_at.to_string(),
        })
    }
}

/// Query as PackageCategory
impl FromRow<'_, PgRow> for PkgCategoryIdAndUpdatedAt {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        // in real word, reservation must have a start time
        let id = row.get("id");

        if id == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        let updated_at: NaiveDateTime = row.get("updated_at");

        Ok(Self {
            id,
            updated_at: updated_at.to_string(),
        })
    }
}

/// Query as PackageCategoryRelation
impl FromRow<'_, PgRow> for PkgCateRelIdAndUpdatedAt {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        // in real word, reservation must have a start time
        let id = row.get("id");

        if id == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        let updated_at: NaiveDateTime = row.get("updated_at");

        Ok(Self {
            id,
            updated_at: updated_at.to_string(),
        })
    }
}

impl FromRow<'_, SqliteRow> for DelKvPair {
    fn from_row(row: &SqliteRow) -> Result<Self, sqlx::Error> {
        let r#type = row.get("type");
        let del_id = row.get("del_id");

        Ok(Self { r#type, del_id })
    }
}
