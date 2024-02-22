use sqlx::Row;

use crate::{abi, PackManager};

use super::Pkg;

impl Pkg for PackManager {
    async fn add_package(
        &self,
        data: abi::types::PackageAddReq,
    ) -> Result<abi::types::PackageAddRes, abi::PkgError> {
        let mut tx = self.pool.begin().await?;

        // add package to the database
        let res = sqlx::query!(
            r#"INSERT INTO packages (name, description, reason, link)
          VALUES ($1, $2, $3, $4)
          RETURNING id"#,
            data.name,
            data.description,
            data.reason,
            data.link
        )
        .fetch_one(&mut *tx)
        .await;

        let id = match res {
            Ok(row) => row.id,
            Err(e) => {
                return Err(abi::PkgError::DbError(e));
            }
        };

        // Relation between package and category
        match data.categories {
            Some(categories) if !categories.is_empty() => {
                let mut query =
                    "INSERT INTO package_category_relations (package_id, category_id) VALUES "
                        .to_string();
                let values = categories
                    .iter()
                    .map(|v| format!("({}, {})", id, v))
                    .collect::<Vec<String>>()
                    .join(", ");

                query.push_str(values.as_str());

                let res = sqlx::query(query.as_str()).execute(&mut *tx).await;

                if let Err(e) = res {
                    return Err(abi::PkgError::DbError(e));
                }
            }
            Some(_) => {}
            None => {}
        }

        // if not commit, the transaction will be rollback when drop
        tx.commit().await?;

        Ok(abi::types::PackageAddRes { id })
    }

    async fn update_package(
        &self,
        data: abi::types::PackageUpdateReq,
    ) -> Result<abi::types::PackageUpdateRes, abi::PkgError> {
        let res = sqlx::query!(
            r#"UPDATE packages
          SET name = $1, description = $2, reason = $3, link = $4
          WHERE id = $5
          RETURNING id"#,
            data.name,
            data.description,
            data.reason,
            data.link,
            data.id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(abi::types::PackageUpdateRes {
            id: res.id.map_or(0, |v| v),
        })
    }

    async fn delete_package(&self, id: i64) -> Result<(), abi::PkgError> {
        sqlx::query!(
            r#"DELETE FROM packages
          WHERE id = $1"#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_package_categories(
        &self,
        data: abi::types::PackageUpdateCategoriesReq,
    ) -> Result<abi::types::PackageUpdateCategoriesRes, abi::PkgError> {
        let abi::types::PackageUpdateCategoriesReq { id, categories } = data;
        let mut tx = self.pool.begin().await?;
        // Delete package all relations
        let del_res = sqlx::query!(
            r#"DELETE FROM package_category_relations
          WHERE package_id = $1"#,
            id,
        )
        .execute(&mut *tx)
        .await;

        match del_res {
            Ok(_) => {}
            Err(e) => {
                return Err(abi::PkgError::DbError(e));
            }
        }

        // Add new relations
        if !categories.is_empty() {
            let mut query =
                "INSERT INTO package_category_relations (package_id, category_id) VALUES "
                    .to_string();
            let values = categories
                .iter()
                .map(|v| format!("({}, {})", id, v))
                .collect::<Vec<String>>()
                .join(", ");

            query.push_str(values.as_str());

            let res = sqlx::query(query.as_str()).execute(&mut *tx).await;

            if let Err(e) = res {
                return Err(abi::PkgError::DbError(e));
            }
        }

        tx.commit().await?;

        Ok(abi::types::PackageUpdateCategoriesRes { id })
    }

    async fn get_package_by_id(&self, id: i64) -> Result<abi::types::Package, abi::PkgError> {
        // get package by id and join category
        let res: abi::types::Package = sqlx::query_as(
            "SELECT p.id, p.name, p.description, p.reason, p.link, p.created_at, p.updated_at,
                GROUP_CONCAT(r.id, ',') as category_ids
            FROM packages p
            LEFT JOIN package_category_relations r ON p.id = r.package_id
            WHERE p.id = $1",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(res)
    }

    async fn query_packages(
        &self,
        data: abi::types::PackageQueryReq,
    ) -> Result<abi::types::PackageQueryRes, abi::PkgError> {
        let (page, page_size) = abi::get_valid_pagination(data.page, data.page_size);
        // query packages by name and category
        let categories = data
            .categories
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(", ");

        let mut query =
            "SELECT p.id, p.name, p.description, p.reason, p.link, p.created_at, p.updated_at,
            GROUP_CONCAT(r.id, ',') as category_ids
            FROM packages p
            LEFT JOIN package_category_relations r ON p.id = r.package_id
            WHERE 1 = 1 "
                .to_string();

        if !data.name.is_empty() {
            query.push_str(" AND p.name LIKE $1");
        }
        if !data.description.is_empty() {
            query.push_str(" AND p.description LIKE $2");
        }
        if !data.reason.is_empty() {
            query.push_str(" AND p.reason LIKE $3");
        }

        if !categories.is_empty() {
            query.push_str(" AND r.category_id IN ($4)");
        }

        query.push_str(" GROUP BY p.id LIMIT $5 OFFSET $6");

        let res: Vec<abi::types::Package> = sqlx::query_as(query.as_str())
            .bind(format!("%{}%", data.name))
            .bind(format!("%{}%", data.description))
            .bind(format!("%{}%", data.reason))
            .bind(categories)
            .bind(page_size)
            .bind((page - 1) * page_size)
            .fetch_all(&self.pool)
            .await?;

        // query total
        let total: i64 = sqlx::query("SELECT COUNT(*) FROM packages")
            .fetch_one(&self.pool)
            .await?
            .get(0);

        Ok(abi::types::PackageQueryRes {
            total,
            packages: res,
        })
    }

    async fn add_category(
        &self,
        data: abi::types::PackageCategoryAddReq,
    ) -> Result<abi::types::PackageCategoryAddRes, abi::PkgError> {
        // add category to the database

        let res = sqlx::query!(
            r#"INSERT INTO package_categories (name, parent_id)
          VALUES ($1, $2)
          RETURNING id"#,
            data.name,
            data.parent_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(abi::types::PackageCategoryAddRes { id: res.id })
    }

    async fn update_category(
        &self,
        _data: abi::types::PackageCategoryUpdateReq,
    ) -> Result<abi::types::PackageCategoryUpdateRes, abi::PkgError> {
        todo!()
    }

    async fn delete_category(&self, _id: i64) -> Result<(), abi::PkgError> {
        todo!()
    }

    async fn query_categories(
        &self,
        _query: abi::types::PackageCategoryQueryReq,
    ) -> Result<abi::types::PackageCategoryQueryRes, abi::PkgError> {
        let res = sqlx::query_as(
            "SELECT id, name, parent_id, created_at, updated_at
            FROM package_categories",
        )
        .fetch_all(&self.pool)
        .await?;

        // query total
        let total: i64 = sqlx::query("SELECT COUNT(*) FROM package_categories")
            .fetch_one(&self.pool)
            .await?
            .get(0);

        Ok(abi::types::PackageCategoryQueryRes { total, data: res })
    }

    async fn query_relations(
        &self,
        id: i64,
    ) -> Result<Vec<abi::types::PackageCategoryRelation>, abi::PkgError> {
        let res = sqlx::query_as(
            "SELECT * FROM package_category_relations
            WHERE package_id = $1",
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await?;

        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::abi::types::PackageAddReq;

    #[tokio::test]
    async fn test_add_category() {
        let pool = get_test_sqlite_pool().await;

        let pk = PackManager::new(pool);
        let data = abi::types::PackageCategoryAddReq {
            name: "test".to_string(),
            parent_id: 1,
        };

        let res = pk.add_category(data).await.unwrap();

        assert_eq!(res.id, 1);
    }

    #[tokio::test]
    async fn test_add_package() {
        let pool = get_test_sqlite_pool().await;

        let pkg = PackManager::new(pool.clone());

        let res = add_package(
            &pkg,
            PackageAddReq {
                name: "test".to_string(),
                description: "test".to_string(),
                reason: "test".to_string(),
                link: "test".to_string(),
                categories: Some(vec![1, 2]),
            },
        )
        .await;

        let res = pkg.get_package_by_id(res.id).await.unwrap();

        assert_eq!(res.id, 1);
        assert_eq!(res.name, "test");
        assert_eq!(res.categories, [1, 2]);
    }

    #[tokio::test]
    async fn query_packages_should_work() {
        let pool = get_test_sqlite_pool().await;

        let pkg = PackManager::new(pool.clone());

        add_package(
            &pkg,
            PackageAddReq {
                name: "test".to_string(),
                description: "test".to_string(),
                reason: "test".to_string(),
                link: "test".to_string(),
                categories: Some(vec![1, 2]),
            },
        )
        .await;

        add_package(
            &pkg,
            PackageAddReq {
                name: "test2".to_string(),
                description: "some test2".to_string(),
                reason: "this test2".to_string(),
                link: "link test2".to_string(),
                categories: Some(vec![1, 2]),
            },
        )
        .await;

        let res = pkg
            .query_packages(abi::types::PackageQueryReq {
                name: "2".to_string(),
                description: "".to_string(),
                categories: vec![],
                reason: "".to_string(),
                page: 1,
                page_size: 10,
            })
            .await
            .unwrap();

        assert_eq!(res.total, 2);
        assert_eq!(res.packages[0].name, "test2");
    }

    async fn add_categories(pkg: &PackManager, data: Vec<abi::types::PackageCategoryAddReq>) {
        for d in data {
            pkg.add_category(d).await.unwrap();
        }
    }

    async fn add_package(pkg: &PackManager, data: PackageAddReq) -> abi::types::PackageAddRes {
        add_categories(
            &pkg,
            vec![
                abi::types::PackageCategoryAddReq {
                    name: get_rand_name(6),
                    parent_id: 1,
                },
                abi::types::PackageCategoryAddReq {
                    name: get_rand_name(6),
                    parent_id: 1,
                },
            ],
        )
        .await;

        pkg.add_package(data).await.unwrap()
    }
}

#[cfg(test)]
async fn get_test_sqlite_pool() -> sqlx::sqlite::SqlitePool {
    use sqlx::{migrate::Migrator, Connection, SqliteConnection};
    use std::path::Path;

    // @see https://github.com/launchbadge/sqlx/issues/2510
    // sqlite in memory database does not support shared memory, but named memory connection is useful
    let mut conn = SqliteConnection::connect("sqlite:file:test?mode=memory&cache=shared")
        .await
        .unwrap();
    let m = Migrator::new(Path::new("./migrations")).await.unwrap();

    m.run(&mut conn).await.unwrap();

    sqlx::sqlite::SqlitePool::connect("sqlite:file:test?mode=memory&cache=shared")
        .await
        .unwrap()
}

#[cfg(test)]
fn get_rand_name(length: usize) -> String {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    let rng = thread_rng();
    let random_string: String = rng
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();

    random_string
}
