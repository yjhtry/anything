use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    id: i32,
    name: String,
    description: String,
    link: String,
    category: String,
    reason: String,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageCategory {
    id: i32,
    name: String,
    parent_id: i32,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageCategoryRelation {
    id: i32,
    name: String,
    parent_id: i32,
    category_id: String,
    created_at: String,
    updated_at: String,
}
