use serde::{Deserialize, Serialize};

/// Package Categories
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageCategory {
    id: i32,
    name: String,
    parent_id: i32,
    created_at: String,
    updated_at: String,
}

/// Package category add request
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageCategoryAddReq {
    name: String,
    parent_id: i32,
}

/// Package category add response
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageCategoryAddRes {
    id: i32,
}

/// Package category update request
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageCategoryUpdateReq {
    id: i32,
    name: String,
    parent_id: i32,
}

/// Package category update response
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageCategoryUpdateRes {
    id: i32,
}

/// Package category delete request
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageCategoryDelReq {
    id: i32,
}

/// Package category delete response
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageCategoryDelRes {
    id: i32,
}

/// Query package category list request
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageCategoryListReq {
    name: Option<String>,
    parent_id: Option<i32>,
    page: i32,
    page_size: i32,
}

/// Query package category list response
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PackageCategoryListRes {
    total: i32,
    data: Vec<PackageCategory>,
}
