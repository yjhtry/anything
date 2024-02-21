mod manager;

use crate::abi::{
    types::{
        PackageAddReq, PackageAddRes, PackageCategoryAddReq, PackageCategoryAddRes,
        PackageCategoryQueryReq, PackageCategoryQueryRes, PackageCategoryUpdateReq,
        PackageCategoryUpdateRes, PackageGetByIdRes, PackageQueryReq, PackageQueryRes,
        PackageUpdateCategoryReq, PackageUpdateCategoryRes, PackageUpdateReq, PackageUpdateRes,
    },
    PackError,
};

#[allow(async_fn_in_trait)]
pub trait Pack {
    /// Add a package to the database
    async fn add_package(&self, data: PackageAddReq) -> Result<PackageAddRes, PackError>;

    /// Update a package in the database
    async fn update_package(&self, data: PackageUpdateReq) -> Result<PackageUpdateRes, PackError>;

    /// Delete a package from the database
    async fn delete_package(&self, id: i64) -> Result<(), PackError>;

    /// Update package category
    async fn update_package_category(
        &self,
        data: PackageUpdateCategoryReq,
    ) -> Result<PackageUpdateCategoryRes, PackError>;

    /// Get a package by id
    async fn get_package_by_id(&self, id: i64) -> Result<PackageGetByIdRes, PackError>;

    /// query packages in the database
    async fn query_packages(
        &self,
        query: PackageQueryReq,
    ) -> Result<Vec<PackageQueryRes>, PackError>;

    /// Add a category to the database
    async fn add_category(
        &self,
        data: PackageCategoryAddReq,
    ) -> Result<PackageCategoryAddRes, PackError>;

    /// Update a category in the database
    async fn update_category(
        &self,
        data: PackageCategoryUpdateReq,
    ) -> Result<PackageCategoryUpdateRes, PackError>;

    /// Delete a category from the database
    async fn delete_category(&self, id: i64) -> Result<(), PackError>;

    /// Query categories in the database
    async fn query_categories(
        &self,
        query: PackageCategoryQueryReq,
    ) -> Result<PackageCategoryQueryRes, PackError>;
}
