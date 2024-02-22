mod manager;

use crate::abi::{
    types::{
        Package, PackageAddReq, PackageAddRes, PackageCategoryAddReq, PackageCategoryAddRes,
        PackageCategoryQueryReq, PackageCategoryQueryRes, PackageCategoryRelation,
        PackageCategoryUpdateReq, PackageCategoryUpdateRes, PackageQueryReq, PackageQueryRes,
        PackageUpdateCategoriesReq, PackageUpdateCategoriesRes, PackageUpdateReq, PackageUpdateRes,
    },
    PkgError,
};

#[allow(async_fn_in_trait)]
pub trait Pkg {
    /// Add a package to the database
    async fn add_package(&self, data: PackageAddReq) -> Result<PackageAddRes, PkgError>;

    /// Update a package in the database
    async fn update_package(&self, data: PackageUpdateReq) -> Result<PackageUpdateRes, PkgError>;

    /// Delete a package from the database
    async fn delete_package(&self, id: i64) -> Result<(), PkgError>;

    /// Update package category
    async fn update_package_categories(
        &self,
        data: PackageUpdateCategoriesReq,
    ) -> Result<PackageUpdateCategoriesRes, PkgError>;

    /// Get a package by id
    async fn get_package_by_id(&self, id: i64) -> Result<Package, PkgError>;

    /// query packages in the database
    async fn query_packages(&self, query: PackageQueryReq) -> Result<PackageQueryRes, PkgError>;

    /// Add a category to the database
    async fn add_category(
        &self,
        data: PackageCategoryAddReq,
    ) -> Result<PackageCategoryAddRes, PkgError>;

    /// Update a category in the database
    async fn update_category(
        &self,
        data: PackageCategoryUpdateReq,
    ) -> Result<PackageCategoryUpdateRes, PkgError>;

    /// Delete a category from the database
    async fn delete_category(&self, id: i64) -> Result<(), PkgError>;

    /// Query categories in the database
    async fn query_categories(
        &self,
        query: PackageCategoryQueryReq,
    ) -> Result<PackageCategoryQueryRes, PkgError>;

    /// Query package category relations
    async fn query_relations(&self, id: i64) -> Result<Vec<PackageCategoryRelation>, PkgError>;
}
