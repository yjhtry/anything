use serde::{Deserialize, Serialize};

/// Package
#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub link: String,
    pub categories: Vec<i64>,
    pub reason: String,
    pub created_at: String,
    pub updated_at: String,
}

/// Package add request
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageAddReq {
    pub name: String,
    pub description: String,
    pub link: String,
    pub categories: Option<Vec<i64>>,
    pub reason: String,
}

/// Package add response
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageAddRes {
    pub id: i64,
}

/// Package update request
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageUpdateReq {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub link: String,
    // pub categories: Option<Vec<i64>>,
    pub reason: String,
}

/// Package update response
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageUpdateRes {
    pub id: i64,
}

/// Query package request
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageQueryReq {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub categories: Vec<i64>,
    pub reason: String,
    pub page: i64,
    pub page_size: i64,
}

/// Query package response
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageQueryRes {
    pub total: i64,
    pub packages: Vec<Package>,
}

/// Update Package Category request
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageUpdateCategoriesReq {
    pub id: i64,
    pub categories: Vec<i64>,
}

/// Update Package Category response
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageUpdateCategoriesRes {
    pub id: i64,
}
