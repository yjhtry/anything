use crate::{abi, PackManager};

use super::Pack;

impl Pack for PackManager {
    async fn add_package(
        &self,
        data: abi::types::PackageAddReq,
    ) -> Result<abi::types::PackageAddRes, abi::PackError> {
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
        .fetch_one(&self.pool)
        .await?;

        Ok(abi::types::PackageAddRes { id: res.id })
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
        _data: abi::types::PackageCategoryAddReq,
    ) -> Result<abi::types::PackageCategoryAddRes, abi::PackError> {
        todo!()
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
    async fn test_add_package() {
        let pool = sqlx::sqlite::SqlitePool::connect("sqlite:anything.db")
            .await
            .unwrap();
        let pack = PackManager::new(pool);
        let data = PackageAddReq {
            name: "test11".to_string(),
            description: "test".to_string(),
            reason: "test".to_string(),
            link: "test".to_string(),
            categories: None,
        };
        let res = pack.add_package(data).await.unwrap();

        assert_eq!(res.id, 1);
    }
}
