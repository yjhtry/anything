use sqlx::{Postgres, Sqlite};

use crate::{
    abi::PkgError,
    types::{DelKvPair, PackageCategory, PackageCategoryRelation, PackageWithOutCategories},
    DbSync, PackManager,
};

use super::PkgSync;

impl PkgSync for PackManager<Postgres> {
    async fn sync_packages(
        &self,
        data: Vec<PackageWithOutCategories>,
        dels: Vec<i64>,
    ) -> Result<(), PkgError> {
        // compare local and remote data find out which need to be added, updated and deleted
        let mut add_data = vec![];
        let mut update_data = vec![];

        for d in data.into_iter() {
            match d.synced {
                0 => add_data.push(d),
                2 => update_data.push(d),
                _ => {}
            }
        }

        if !dels.is_empty() {
            let query = format!(
                "DELETE FROM packages WHERE id IN ({})",
                dels.iter()
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

    async fn sync_package_categories(
        &self,
        data: Vec<PackageCategory>,
        dels: Vec<i64>,
    ) -> Result<(), PkgError> {
        // compare local and remote data find out which need to be added, updated and deleted
        let mut add_data = vec![];
        let mut update_data = vec![];

        for d in data.into_iter() {
            match d.synced {
                0 => add_data.push(d),
                2 => update_data.push(d),
                _ => {}
            }
        }

        if !dels.is_empty() {
            let query = format!(
                "DELETE FROM package_categories WHERE id IN ({})",
                dels.iter()
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
        dels: Vec<i64>,
    ) -> Result<(), PkgError> {
        // compare local and remote data find out which need to be added, updated and deleted
        let mut add_data = vec![];
        let mut update_data = vec![];

        for d in data.into_iter() {
            match d.synced {
                0 => add_data.push(d),
                2 => update_data.push(d),
                _ => {}
            }
        }

        if !dels.is_empty() {
            let query = format!(
                "DELETE FROM package_category_relations WHERE id IN ({})",
                dels.iter()
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
        let del_records: Vec<DelKvPair> = sqlx::query_as("SELECT * FROM del_records")
            .fetch_all(&self.pool)
            .await?;

        let mut del_packages = vec![];
        let mut del_categories = vec![];
        let mut del_relations = vec![];

        for d in del_records {
            match d.r#type {
                1 => del_packages.push(d.del_id),
                2 => del_relations.push(d.del_id),
                3 => del_categories.push(d.del_id),
                _ => {}
            }
        }

        let local_packages = sqlx::query_as("SELECT * FROM packages where synced <> 0")
            .fetch_all(&self.pool)
            .await
            .unwrap();

        let local_categories = sqlx::query_as("SELECT * FROM package_categories where synced <> 0")
            .fetch_all(&self.pool)
            .await?;

        let local_relations =
            sqlx::query_as("SELECT * FROM package_category_relations where synced <> 0")
                .fetch_all(&self.pool)
                .await?;

        other_manager
            .sync_packages(local_packages, del_packages)
            .await
            .unwrap();

        other_manager
            .sync_package_categories(local_categories, del_categories)
            .await?;
        other_manager
            .sync_package_category_relations(local_relations, del_relations)
            .await?;

        // drop all del_records

        sqlx::query("DELETE FROM del_records")
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
