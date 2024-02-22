use crate::{abi, PackManager};

use super::Pkg;

impl Pkg for PackManager {
    async fn add_package(
        &self,
        data: abi::types::PackageAddReq,
    ) -> Result<abi::types::PackageAddRes, abi::PackError> {
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
                return Err(abi::PackError::DbError(e));
            }
        };

        match data.categories {
            Some(categories) if categories.is_empty() => {
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
                    return Err(abi::PackError::DbError(e));
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
        _data: abi::types::PackageUpdateReq,
    ) -> Result<abi::types::PackageUpdateRes, abi::PackError> {
        todo!()
    }

    async fn delete_package(&self, _id: i64) -> Result<(), abi::PackError> {
        todo!()
    }

    async fn update_package_category(
        &self,
        _data: abi::types::PackageUpdateCategoryReq,
    ) -> Result<abi::types::PackageUpdateCategoryRes, abi::PackError> {
        todo!()
    }

    async fn get_package_by_id(
        &self,
        _id: i64,
    ) -> Result<abi::types::PackageGetByIdRes, abi::PackError> {
        todo!()
    }

    async fn query_packages(
        &self,
        _query: abi::types::PackageQueryReq,
    ) -> Result<Vec<abi::types::PackageQueryRes>, abi::PackError> {
        todo!()
    }

    async fn add_category(
        &self,
        data: abi::types::PackageCategoryAddReq,
    ) -> Result<abi::types::PackageCategoryAddRes, abi::PackError> {
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
    ) -> Result<abi::types::PackageCategoryUpdateRes, abi::PackError> {
        todo!()
    }

    async fn delete_category(&self, _id: i64) -> Result<(), abi::PackError> {
        todo!()
    }

    async fn query_categories(
        &self,
        _query: abi::types::PackageCategoryQueryReq,
    ) -> Result<abi::types::PackageCategoryQueryRes, abi::PackError> {
        todo!()
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

        let data = PackageAddReq {
            name: "test11".to_string(),
            description: "test".to_string(),
            reason: "test".to_string(),
            link: "test".to_string(),
            categories: Some(vec![1, 2]),
        };

        add_categories(
            &pkg,
            vec![
                abi::types::PackageCategoryAddReq {
                    name: "test1".to_string(),
                    parent_id: 1,
                },
                abi::types::PackageCategoryAddReq {
                    name: "test2".to_string(),
                    parent_id: 1,
                },
            ],
        )
        .await;

        let res = pkg.add_package(data).await.unwrap();

        assert_eq!(res.id, 1);
    }

    async fn add_categories(pkg: &PackManager, data: Vec<abi::types::PackageCategoryAddReq>) {
        for d in data {
            pkg.add_category(d).await.unwrap();
        }
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
