use sqlx::{Postgres, Sqlite};

use crate::{
    abi::PkgError,
    types::{
        PackageCategory, PackageCategoryRelation, PackageIdAndUpdatedAt, PackageWithOutCategories,
        PkgCateRelIdAndUpdatedAt, PkgCategoryIdAndUpdatedAt,
    },
    DbSync, PackManager,
};

use super::PkgSync;

impl PkgSync for PackManager<Postgres> {
    async fn sync_packages(&self, data: Vec<PackageWithOutCategories>) -> Result<(), PkgError> {
        let remote_data: Vec<PackageIdAndUpdatedAt> =
            sqlx::query_as("SELECT id, updated_at FROM packages")
                .fetch_all(&self.pool)
                .await?;

        // compare local and remote data find out which need to be added, updated and deleted
        let mut add_data = vec![];
        let mut update_data = vec![];
        let mut delete_data = vec![];

        for d in &remote_data {
            let local = data.iter().find(|v| v.id == d.id);

            if local.is_none() {
                delete_data.push(d.id);
            }
        }

        for d in data {
            let remote = remote_data.iter().find(|v| v.id == d.id);

            match remote {
                Some(v) => {
                    if v.updated_at != d.updated_at {
                        update_data.push(d);
                    }
                }
                None => {
                    add_data.push(d);
                }
            }
        }

        if !delete_data.is_empty() {
            let query = format!(
                "DELETE FROM packages WHERE id IN ({})",
                delete_data
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(", ")
            );

            sqlx::query(query.as_str()).execute(&self.pool).await?;
        }

        if !add_data.is_empty() {
            let mut query =
              "INSERT INTO packages (id, name, description, reason, link, created_at, updated_at) VALUES ".to_string();
            let values = add_data
                .iter()
                .map(|v| {
                    format!(
                        "({}, '{}', '{}', '{}', '{}', '{}', '{}')",
                        v.id, v.name, v.description, v.reason, v.link, v.created_at, v.updated_at
                    )
                })
                .collect::<Vec<String>>()
                .join(", ");

            query.push_str(values.as_str());

            sqlx::query(query.as_str()).execute(&self.pool).await?;
        }

        if !update_data.is_empty() {
            for d in update_data {
                sqlx::query(
                    "UPDATE packages
              SET name = $1, description = $2, reason = $3, link = $4,  updated_at = $6
              WHERE id = $7",
                )
                .bind(d.name)
                .bind(d.description)
                .bind(d.reason)
                .bind(d.link)
                .bind(d.updated_at)
                .execute(&self.pool)
                .await?;
            }
        }
        Ok(())
    }

    async fn sync_package_categories(&self, data: Vec<PackageCategory>) -> Result<(), PkgError> {
        let remote_data: Vec<PkgCategoryIdAndUpdatedAt> =
            sqlx::query_as("SELECT id, updated_at FROM package_categories")
                .fetch_all(&self.pool)
                .await?;

        // compare local and remote data find out which need to be added, updated and deleted
        let mut add_data = vec![];
        let mut update_data = vec![];
        let mut delete_data = vec![];

        for d in &remote_data {
            let local = data.iter().find(|v| v.id == d.id);

            if local.is_none() {
                delete_data.push(d.id);
            }
        }

        for d in data {
            let remote = remote_data.iter().find(|v| v.id == d.id);

            match remote {
                Some(v) => {
                    if v.updated_at != d.updated_at {
                        update_data.push(d);
                    }
                }
                None => {
                    add_data.push(d);
                }
            }
        }

        if !delete_data.is_empty() {
            let query = format!(
                "DELETE FROM package_categories WHERE id IN ({})",
                delete_data
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(", ")
            );

            sqlx::query(query.as_str()).execute(&self.pool).await?;
        }

        if !add_data.is_empty() {
            let mut query =
              "INSERT INTO package_categories (id, name, parent_id, created_at, updated_at) VALUES ".to_string();
            let values = add_data
                .iter()
                .map(|v| {
                    format!(
                        "({}, '{}', {}, '{}', '{}')",
                        v.id, v.name, v.parent_id, v.created_at, v.updated_at
                    )
                })
                .collect::<Vec<String>>()
                .join(", ");

            query.push_str(values.as_str());

            sqlx::query(query.as_str()).execute(&self.pool).await?;
        }

        if !update_data.is_empty() {
            for d in update_data {
                sqlx::query(
                    "UPDATE package_categories
              SET name = $1, parent_id = $2, updated_at = $3
              WHERE id = $4",
                )
                .bind(d.name)
                .bind(d.parent_id)
                .bind(d.updated_at)
                .execute(&self.pool)
                .await?;
            }
        }

        Ok(())
    }

    async fn sync_package_category_relations(
        &self,
        data: Vec<PackageCategoryRelation>,
    ) -> Result<(), PkgError> {
        let remote_data: Vec<PkgCateRelIdAndUpdatedAt> =
            sqlx::query_as("SELECT * FROM package_category_relations")
                .fetch_all(&self.pool)
                .await?;

        // compare local and remote data find out which need to be added, updated and deleted
        let mut add_data = vec![];
        let mut update_data = vec![];
        let mut delete_data = vec![];

        for d in &remote_data {
            let local = data.iter().find(|v| v.id == d.id);

            if local.is_none() {
                delete_data.push(d.id);
            }
        }

        for d in data {
            let remote = remote_data.iter().find(|v| v.id == d.id);

            match remote {
                Some(v) => {
                    if v.updated_at != d.updated_at {
                        update_data.push(d);
                    }
                }
                None => {
                    add_data.push(d);
                }
            }
        }

        if !delete_data.is_empty() {
            let query = format!(
                "DELETE FROM package_category_relations WHERE id IN ({})",
                delete_data
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(", ")
            );

            sqlx::query(query.as_str()).execute(&self.pool).await?;
        }

        if !add_data.is_empty() {
            let mut query =
              "INSERT INTO package_category_relations (id, package_id, category_id, created_at, updated_at) VALUES "
                  .to_string();
            let values = add_data
                .iter()
                .map(|v| {
                    format!(
                        "({}, {}, {}, '{}', '{}')",
                        v.id, v.package_id, v.category_id, v.created_at, v.updated_at
                    )
                })
                .collect::<Vec<String>>()
                .join(", ");

            query.push_str(values.as_str());

            sqlx::query(query.as_str()).execute(&self.pool).await?;
        }

        if !update_data.is_empty() {
            for d in update_data {
                sqlx::query(
                    "UPDATE package_category_relations
              SET package_id = $1, category_id = $2, updated_at = $3
              WHERE id = $4",
                )
                .bind(d.package_id)
                .bind(d.category_id)
                .bind(d.updated_at)
                .execute(&self.pool)
                .await?;
            }
        }

        Ok(())
    }
}

impl DbSync<PackManager<Postgres>> for PackManager<Sqlite> {
    async fn sync(&self, other_manager: PackManager<Postgres>) -> Result<(), PkgError> {
        let local_packages = sqlx::query_as("SELECT * FROM packages where synced = 0")
            .fetch_all(&self.pool)
            .await
            .unwrap();

        let local_categories = sqlx::query_as("SELECT * FROM package_categories where synced = 0")
            .fetch_all(&self.pool)
            .await?;

        let local_relations =
            sqlx::query_as("SELECT * FROM package_category_relations where synced = 0")
                .fetch_all(&self.pool)
                .await?;

        other_manager.sync_packages(local_packages).await.unwrap();

        other_manager
            .sync_package_categories(local_categories)
            .await?;
        other_manager
            .sync_package_category_relations(local_relations)
            .await?;
        Ok(())
    }
}
