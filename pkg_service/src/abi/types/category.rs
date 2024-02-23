use serde::{Deserialize, Serialize};

/// Package Categories
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageCategory {
    pub id: i64,
    pub name: String,
    pub parent_id: i64,
    pub created_at: String,
    pub updated_at: String,
}

/// Package category add request
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageCategoryAddReq {
    pub name: String,
    #[serde(default)]
    pub parent_id: i64,
}

/// Package category add response
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageCategoryAddRes {
    pub id: i64,
}

/// Package category update request
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageCategoryUpdateReq {
    pub id: i64,
    pub name: String,
    pub parent_id: i64,
}

/// Package category update response
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageCategoryUpdateRes {
    pub id: i64,
}

/// Query package category list request
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageCategoryQueryReq {
    pub name: String,
    pub parent_id: Option<i64>,
    pub page: i64,
    pub page_size: i64,
}

/// Query package category list response
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PackageCategoryQueryRes {
    pub total: i64,
    pub data: Vec<PackageCategory>,
}
