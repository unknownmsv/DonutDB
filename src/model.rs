use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Deserialize)]
pub struct TranslateRequest {
    pub dataset: String,
    pub slot: Vec<String>,
    pub data: Vec<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct ApiKeys {
    pub keys: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ColumnSchema {
    pub name: String,
    pub r#type: String, // مثلاً "u64", "string", "datetime"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableSchema {
    pub columns: Vec<ColumnSchema>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableMeta {
    pub table_name: String,
    pub record_count: usize,
    pub partition_count: usize,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexEntry {
    pub id: u64,
    pub offset: u64,
    pub partition: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableIndex {
    pub primary_key: String,
    pub indexes: Vec<IndexEntry>,
}