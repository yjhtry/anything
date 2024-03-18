use sqlx::{Row, Sqlite};

use crate::{abi::get_valid_pagination, PackManager};

use super::Pkg;

use crate::abi::{
    types::{
        Package, PackageAddReq, PackageAddRes, PackageCategoryAddReq, PackageCategoryAddRes,
        PackageCategoryQueryReq, PackageCategoryQueryRes, PackageCategoryRelation,
        PackageCategoryUpdateReq, PackageCategoryUpdateRes, PackageQueryReq, PackageQueryRes,
        PackageUpdateCategoriesReq, PackageUpdateCategoriesRes, PackageUpdateReq, PackageUpdateRes,
    },
    PkgError,
};

macro_rules! query_cond {
    ($name:ident, $cond:expr, $value:expr) => {
        let $name = if $value.is_empty() {
            "".to_string()
        } else {
            format!(" AND {}", $cond)
        };
    };
}

impl Pkg for PackManager<Sqlite> {
    async fn add_package(&self, data: PackageAddReq) -> Result<PackageAddRes, PkgError> {
        let mut tx = self.pool.begin().await?;

        // add package to the database
        let id: i64 = sqlx::query(
            r#"INSERT INTO packages (name, description, reason, link)
          VALUES ($1, $2, $3, $4)
          RETURNING id"#,
        )
        .bind(data.name)
        .bind(data.description)
        .bind(data.reason)
        .bind(data.link)
        .fetch_one(&mut *tx)
        .await?
        .get(0);

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
                    return Err(PkgError::DbError(e));
                }
            }
            Some(_) => {}
            None => {}
        }

        // if not commit, the transaction will be rollback when drop
        tx.commit().await?;

        Ok(PackageAddRes { id })
    }

    async fn update_package(&self, data: PackageUpdateReq) -> Result<PackageUpdateRes, PkgError> {
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

        match res.id {
            Some(id) => Ok(PackageUpdateRes { id }),
            None => Err(PkgError::NotFoundPackage),
        }
    }

    async fn delete_package(&self, id: i64) -> Result<(), PkgError> {
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
        data: PackageUpdateCategoriesReq,
    ) -> Result<PackageUpdateCategoriesRes, PkgError> {
        let PackageUpdateCategoriesReq { id, categories } = data;
        let mut tx = self.pool.begin().await?;
        // Delete package all relations
        // optimize: use BEGIN TRANSACTION; COMMIT; instead of sqlx transaction
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
                return Err(PkgError::DbError(e));
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
                return Err(PkgError::DbError(e));
            }
        }

        tx.commit().await?;

        Ok(PackageUpdateCategoriesRes { id })
    }

    async fn get_package_by_id(&self, id: i64) -> Result<Package, PkgError> {
        // get package by id and join category
        let res = sqlx::query_as(
            "SELECT p.id, p.name, p.description, p.reason, p.link, p.created_at, p.updated_at,
                GROUP_CONCAT(r.category_id, ',') as category_ids
            FROM packages p
            LEFT JOIN package_category_relations r ON p.id = r.package_id
            WHERE p.id = $1",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await;

        match res {
            Ok(row) => Ok(row),
            Err(_) => Err(PkgError::NotFoundPackage),
        }
    }

    async fn query_packages(&self, data: PackageQueryReq) -> Result<PackageQueryRes, PkgError> {
        let (page, page_size) = get_valid_pagination(data.page, data.page_size);
        // query packages by name and category
        let categories = data
            .categories
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(", ");

        query_cond!(
            name_cond,
            format!("p.name LIKE '%{}%'", data.name).as_str(),
            data.name
        );
        query_cond!(
            desc_cond,
            format!("p.description LIKE '%{}%'", data.description),
            data.description
        );
        query_cond!(
            reason_cond,
            format!("p.reason LIKE '%{}%'", data.reason),
            data.reason
        );

        let categories_cond = if categories.is_empty() {
            "".to_string()
        } else {
            format!(" AND r.category_id IN ({})", categories)
        };

        let group_by = format!(
            " GROUP BY p.id LIMIT {} OFFSET {}",
            page_size,
            (page - 1) * page_size
        );

        let where_cond = format!("{} {} {}", name_cond, desc_cond, reason_cond,);

        let query = format!(
            "SELECT p.id, p.name, p.description, p.reason, p.link, p.created_at, p.updated_at,
                GROUP_CONCAT(p.category_id, ',') as category_ids
                FROM (SELECT * FROM packages p JOIN package_category_relations r ON p.id = r.package_id WHERE TRUE {} {}) p
                {}",
            where_cond, categories_cond, group_by
        );

        let res: Vec<Package> = sqlx::query_as(query.as_str())
            .bind(format!("%{}%", data.name))
            .bind(format!("%{}%", data.description))
            .bind(format!("%{}%", data.reason))
            .bind(categories.clone())
            .bind(page_size)
            .bind((page - 1) * page_size)
            .fetch_all(&self.pool)
            .await?;

        let total_cond = if categories.is_empty() {
            format!(" WHERE 1 = 1 {}", where_cond)
        } else {
            format!(
                " JOIN package_category_relations r ON p.id = r.package_id WHERE 1 = 1 {} {} ",
                where_cond, categories_cond
            )
        };

        let total: i64 =
            sqlx::query(format!("SELECT COUNT(*) FROM packages p {}", total_cond).as_str())
                .fetch_one(&self.pool)
                .await?
                .get(0);

        Ok(PackageQueryRes { total, data: res })
    }

    async fn add_category(
        &self,
        data: PackageCategoryAddReq,
    ) -> Result<PackageCategoryAddRes, PkgError> {
        // query parent category, if not exists, return error
        if data.parent_id > 0 {
            let res = sqlx::query!(
                r#"SELECT id FROM package_categories WHERE id = $1"#,
                data.parent_id
            )
            .fetch_one(&self.pool)
            .await;

            if res.is_err() {
                return Err(PkgError::NotFoundCategory);
            }
        }

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

        Ok(PackageCategoryAddRes { id: res.id })
    }

    async fn update_category(
        &self,
        data: PackageCategoryUpdateReq,
    ) -> Result<PackageCategoryUpdateRes, PkgError> {
        let res = sqlx::query!(
            r#"UPDATE package_categories
          SET name = $1, parent_id = $2
          WHERE id = $3
          RETURNING id"#,
            data.name,
            data.parent_id,
            data.id
        )
        .fetch_one(&self.pool)
        .await?;

        match res.id {
            Some(id) => Ok(PackageCategoryUpdateRes { id }),
            None => Err(PkgError::NotFoundCategory),
        }
    }

    async fn delete_category(&self, id: i64) -> Result<(), PkgError> {
        // check if has child category
        let has_child: i64 =
            sqlx::query("SELECT COUNT(*) FROM package_categories WHERE parent_id = $1")
                .bind(id)
                .fetch_one(&self.pool)
                .await?
                .get(0);

        if has_child > 0 {
            return Err(PkgError::CannotDeleteHasChildCategory);
        }

        // check if has relation
        let has_relation: i64 =
            sqlx::query("SELECT COUNT(*) FROM package_category_relations WHERE category_id = $1")
                .bind(id)
                .fetch_one(&self.pool)
                .await?
                .get(0);

        if has_relation > 0 {
            return Err(PkgError::CannotDeleteHasRelationCategory);
        }

        sqlx::query!(
            r#"DELETE FROM package_categories
          WHERE id = $1"#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn query_categories(
        &self,
        data: PackageCategoryQueryReq,
    ) -> Result<PackageCategoryQueryRes, PkgError> {
        let (page, page_size) = get_valid_pagination(data.page, data.page_size);

        query_cond!(name_cond, format!("name LIKE '%{}%'", data.name), data.name);

        let parent_id_cond = if data.parent_id.is_some() {
            format!(" AND parent_id = {}", data.parent_id.unwrap())
        } else {
            "".to_string()
        };

        let where_cond = format!("{} {}", name_cond, parent_id_cond);

        let query = format!(
            "SELECT id, name, parent_id, created_at, updated_at FROM package_categories
              WHERE 1 = 1 {}  LIMIT {} OFFSET {};",
            where_cond,
            page_size,
            (page - 1) * page_size
        );

        let res = sqlx::query_as(query.as_str()).fetch_all(&self.pool).await?;

        // query total
        let total: i64 = sqlx::query(
            format!(
                "SELECT COUNT(*) FROM package_categories WHERE 1 = 1 {}",
                where_cond
            )
            .as_str(),
        )
        .fetch_one(&self.pool)
        .await?
        .get(0);

        Ok(PackageCategoryQueryRes { total, data: res })
    }

    async fn query_relations_by_pkg_id(
        &self,
        id: i64,
    ) -> Result<Vec<PackageCategoryRelation>, PkgError> {
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

    #[tokio::test]
    async fn add_package_should_word() {
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
    async fn update_package_should_work() {
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

        assert_eq!(res.name, "test");

        let res = pkg
            .update_package(PackageUpdateReq {
                id: res.id,
                name: "test2".to_string(),
                description: "test2".to_string(),
                reason: "test2".to_string(),
                link: "test2".to_string(),
            })
            .await
            .unwrap();

        let res = pkg.get_package_by_id(res.id).await.unwrap();

        assert_eq!(res.name, "test2");
    }

    #[tokio::test]
    async fn delete_package_should_work() {
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

        assert_eq!(res.name, "test");

        pkg.delete_package(res.id).await.unwrap();

        let res = pkg.get_package_by_id(res.id).await;

        assert!(res.is_err());
    }

    #[tokio::test]
    async fn update_package_categories_should_work() {
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

        // add id 3 category
        add_categories(
            &pkg,
            vec![PackageCategoryAddReq {
                name: get_rand_name(6),
                parent_id: 1,
            }],
        )
        .await;

        let res = pkg.get_package_by_id(res.id).await.unwrap();

        assert_eq!(res.name, "test");
        assert_eq!(res.categories, [1, 2]);

        let res = pkg
            .update_package_categories(PackageUpdateCategoriesReq {
                id: res.id,
                categories: vec![1, 3],
            })
            .await
            .unwrap();

        let res = pkg.get_package_by_id(res.id).await.unwrap();

        assert_eq!(res.categories, [1, 3]);
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
            .query_packages(PackageQueryReq {
                name: "2".to_string(),
                description: "".to_string(),
                categories: vec![],
                reason: "".to_string(),
                page: 1,
                page_size: 10,
            })
            .await
            .unwrap();

        assert_eq!(res.total, 1);
        assert_eq!(res.data[0].name, "test2");
    }

    #[tokio::test]
    async fn query_packages_when_empty_should_work() {
        let pool = get_test_sqlite_pool().await;
        let pkg = PackManager::new(pool.clone());
        let res = pkg
            .query_packages(PackageQueryReq {
                name: "2".to_string(),
                description: "".to_string(),
                categories: vec![],
                reason: "".to_string(),
                page: 1,
                page_size: 10,
            })
            .await
            .unwrap();

        assert_eq!(res.total, 0);
    }

    #[tokio::test]
    async fn query_packages_pagination_should_work() {
        let pool = get_test_sqlite_pool().await;

        let pkg = PackManager::new(pool.clone());

        for _ in 0..10 {
            add_package(
                &pkg,
                PackageAddReq {
                    name: get_rand_name(6),
                    description: "test".to_string(),
                    reason: "test".to_string(),
                    link: "test".to_string(),
                    categories: Some(vec![1, 2]),
                },
            )
            .await;
        }

        let res = pkg
            .query_packages(PackageQueryReq {
                name: "".to_string(),
                description: "".to_string(),
                categories: vec![],
                reason: "".to_string(),
                page: 2,
                page_size: 5,
            })
            .await
            .unwrap();

        assert_eq!(res.total, 10);
        assert_eq!(res.data.len(), 5);
    }

    #[tokio::test]
    async fn add_category_should_work() {
        let pool = get_test_sqlite_pool().await;

        let pkg = PackManager::new(pool.clone());

        pkg.add_category(PackageCategoryAddReq {
            name: "test".to_string(),
            parent_id: 0,
        })
        .await
        .unwrap();

        let res = pkg
            .add_category(PackageCategoryAddReq {
                name: "test".to_string(),
                parent_id: 1,
            })
            .await
            .unwrap();

        assert_eq!(res.id, 2);
    }

    #[tokio::test]
    async fn update_category_should_work() {
        let pool = get_test_sqlite_pool().await;

        let pkg = PackManager::new(pool.clone());

        let res = pkg
            .add_category(PackageCategoryAddReq {
                name: "test".to_string(),
                parent_id: 0,
            })
            .await
            .unwrap();

        assert_eq!(res.id, 1);

        let res = pkg
            .update_category(PackageCategoryUpdateReq {
                id: 1,
                name: "test2".to_string(),
                parent_id: 0,
            })
            .await
            .unwrap();

        assert_eq!(res.id, 1);
    }

    #[tokio::test]
    async fn delete_category_should_work() {
        let pool = get_test_sqlite_pool().await;

        let pkg = PackManager::new(pool.clone());

        let res = pkg
            .add_category(PackageCategoryAddReq {
                name: "test".to_string(),
                parent_id: 0,
            })
            .await
            .unwrap();

        assert_eq!(res.id, 1);

        pkg.delete_category(1).await.unwrap();

        let res = pkg
            .query_categories(PackageCategoryQueryReq {
                name: "".to_string(),
                parent_id: None,
                page: 1,
                page_size: 10,
            })
            .await
            .unwrap();

        assert_eq!(res.total, 0);
    }

    #[tokio::test]
    async fn delete_category_when_has_child_should_work() {
        let pool = get_test_sqlite_pool().await;

        let pkg = PackManager::new(pool.clone());

        let res = pkg
            .add_category(PackageCategoryAddReq {
                name: "test".to_string(),
                parent_id: 0,
            })
            .await
            .unwrap();

        assert_eq!(res.id, 1);

        pkg.add_category(PackageCategoryAddReq {
            name: "test2".to_string(),
            parent_id: 1,
        })
        .await
        .unwrap();

        let res = pkg.delete_category(1).await;

        assert!(res.is_err());
    }

    #[tokio::test]
    async fn delete_category_when_has_relation_should_work() {
        let pool = get_test_sqlite_pool().await;

        let pkg = PackManager::new(pool.clone());

        let res = pkg
            .add_category(PackageCategoryAddReq {
                name: "test".to_string(),
                parent_id: 0,
            })
            .await
            .unwrap();

        assert_eq!(res.id, 1);

        add_package(
            &pkg,
            PackageAddReq {
                name: "test".to_string(),
                description: "test".to_string(),
                reason: "test".to_string(),
                link: "test".to_string(),
                categories: Some(vec![1]),
            },
        )
        .await;

        let res = pkg.delete_category(1).await;

        assert!(res.is_err());
    }

    #[tokio::test]
    async fn query_categories_should_work() {
        let pool = get_test_sqlite_pool().await;

        let pkg = PackManager::new(pool.clone());

        pkg.add_category(PackageCategoryAddReq {
            name: "test".to_string(),
            parent_id: 0,
        })
        .await
        .unwrap();

        pkg.add_category(PackageCategoryAddReq {
            name: "test2".to_string(),
            parent_id: 0,
        })
        .await
        .unwrap();

        let res = pkg
            .query_categories(PackageCategoryQueryReq {
                name: "t2".to_string(),
                parent_id: None,
                page: 1,
                page_size: 10,
            })
            .await
            .unwrap();

        assert_eq!(res.total, 1);
        assert_eq!(res.data[0].name, "test2");
    }

    #[tokio::test]
    async fn query_categories_when_empty_should_work() {
        let pool = get_test_sqlite_pool().await;
        let pkg = PackManager::new(pool.clone());
        let res = pkg
            .query_categories(PackageCategoryQueryReq {
                name: "t2".to_string(),
                parent_id: None,
                page: 1,
                page_size: 10,
            })
            .await
            .unwrap();

        assert_eq!(res.total, 0);
    }

    #[tokio::test]
    async fn query_categories_pagination_should_work() {
        let pool = get_test_sqlite_pool().await;

        let pkg = PackManager::new(pool.clone());

        for _ in 0..10 {
            pkg.add_category(PackageCategoryAddReq {
                name: get_rand_name(6),
                parent_id: 0,
            })
            .await
            .unwrap();
        }

        let res = pkg
            .query_categories(PackageCategoryQueryReq {
                name: "".to_string(),
                parent_id: None,
                page: 2,
                page_size: 5,
            })
            .await
            .unwrap();

        assert_eq!(res.total, 10);
        assert_eq!(res.data.len(), 5);
    }

    #[tokio::test]
    async fn query_relations_by_pkg_id_should_work() {
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

        let res = pkg.query_relations_by_pkg_id(res.id).await.unwrap();

        assert_eq!(res.len(), 2);
    }

    async fn add_categories(pkg: &PackManager<Sqlite>, data: Vec<PackageCategoryAddReq>) {
        for d in data {
            pkg.add_category(d).await.unwrap();
        }
    }

    async fn add_package(pkg: &PackManager<Sqlite>, data: PackageAddReq) -> PackageAddRes {
        add_categories(
            &pkg,
            vec![
                PackageCategoryAddReq {
                    name: get_rand_name(6),
                    parent_id: 0,
                },
                PackageCategoryAddReq {
                    name: get_rand_name(6),
                    parent_id: 0,
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
