use serde::{Deserialize, Serialize};

/// Package Category Relations
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageCategoryRelation {
    id: i32,
    name: String,
    parent_id: i32,
    category_id: String,
    created_at: String,
    updated_at: String,
}

/// Package category relation create request
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageCategoryRelationAddReq {
    parent_id: i32,
    category_id: String,
}

/// Package category relation create response
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageCategoryRelationAddRes {
    id: i32,
}

/// Package category relation update request
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageCategoryRelationUpdateReq {
    id: i32,
    parent_id: i32,
    category_id: String,
}

/// Package category relation update response
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageCategoryRelationUpdateRes {
    id: i32,
}

/// Package category relation delete request
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageCategoryRelationDelReq {
    id: i32,
}

/// Package category relation delete response
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageCategoryRelationDelRes {
    id: i32,
}
