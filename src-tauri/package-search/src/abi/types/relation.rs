use serde::{Deserialize, Serialize};

/// Package Category Relations
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageCategoryRelation {
    pub id: i64,
    pub name: String,
    pub parent_id: i64,
    pub category_id: String,
    pub created_at: String,
    pub updated_at: String,
}

/// Package category relation create request
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageCategoryRelationAddReq {
    pub parent_id: i64,
    pub category_id: String,
}

/// Package category relation create response
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageCategoryRelationAddRes {
    pub id: i64,
}

/// Package category relation update request
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageCategoryRelationUpdateReq {
    pub id: i64,
    pub parent_id: i64,
    pub category_id: String,
}

/// Package category relation update response
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageCategoryRelationUpdateRes {
    pub id: i64,
}
