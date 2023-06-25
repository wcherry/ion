use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

// TODO: This is not a real table, but a view.  Is there a way to make this work better?
table! {
    blocks_dtos (id) {
        id -> VarChar,
        block_id -> VarChar,
        version -> Integer,
        display_order -> Nullable<Integer>,
        block_type -> VarChar,
        content -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        created_by -> Integer,
        updated_by -> Integer,
        active -> Bool,
    }
}
  
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, QueryableByName, PartialEq,)]
#[diesel(table_name = blocks_dtos)]
pub struct BlockDto {
    pub id: String, 
    pub block_id: String,
    pub version: i32,
    pub display_order: Option<i32>,
    pub block_type: String,
    pub content: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_by: i32,
    pub updated_by: i32,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockRequest {
    pub block_id: Option<String>,
    pub version: Option<i32>,
    pub page_version_id: Option<String>,
    pub block_type: String,
    pub content: Option<String>,
    pub display_order: i32,
}
