use serde::{Deserialize, Serialize};

/// Package
#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    id: i32,
    name: String,
    description: String,
    link: String,
    category: Option<Vec<i32>>,
    reason: String,
    created_at: String,
    updated_at: String,
}

/// Package add request
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageAddReq {
    name: String,
    description: String,
    link: String,
    category: Option<Vec<i32>>,
    reason: String,
}

/// Package add response
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageAddRes {
    id: i32,
}

/// Package update request
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageUpdateReq {
    id: i32,
    name: String,
    description: String,
    link: String,
    category: Option<Vec<i32>>,
    reason: String,
}

/// Package update response
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageUpdateRes {
    id: i32,
}

/// Package delete request
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageDelReq {
    id: i32,
}

/// Package delete response
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageDelRes {
    id: i32,
}

/// Get package by id request
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageGetByIdReq {
    id: i32,
}

/// Get package by id response
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageGetByIdRes {
    id: i32,
    name: String,
    description: String,
    link: String,
    category: Option<Vec<i32>>,
    reason: String,
    created_at: String,
    updated_at: String,
}

/// Query package request
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageQueryReq {
    name: String,
    category: Option<Vec<i32>>,
    reason: String,
    page: i32,
    page_size: i32,
}

/// Query package response
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageQueryRes {
    total: i32,
    packages: Vec<Package>,
}

/// Update Package Category request
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageUpdateCategoryReq {
    id: i32,
    category: Vec<i32>,
}

/// Update Package Category response
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageUpdateCategoryRes {
    id: i32,
}
