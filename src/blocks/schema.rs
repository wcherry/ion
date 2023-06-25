use diesel::prelude::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

table! {
    blocks (id) {
        id -> Uuid,
        block_id -> Uuid,
        version -> Integer,
        block_type -> VarChar,
        content -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        created_by -> Integer,
        updated_by -> Integer,
        active -> Bool,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, QueryableByName, PartialEq,AsChangeset)]
#[diesel(table_name = blocks)]
pub struct Block {
    pub id: uuid::Uuid,
    pub block_id: uuid::Uuid,
    pub version: i32,
    pub block_type: String,
    pub content: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_by: i32,
    pub updated_by: i32,
    pub active: bool,
}

table! {
    page_block_index (id) {
        id -> Uuid,
        page_version_id -> Uuid,
        block_id -> Uuid,
        display_order -> Integer,
        created_at -> Timestamp,
    }
}

 #[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, QueryableByName, PartialEq,)]
 #[diesel(table_name = page_block_index)] 
pub struct PageBlockIndex {
    pub id: uuid::Uuid,
    pub page_version_id: uuid::Uuid,
    pub block_id: uuid::Uuid,
    pub display_order: i32,
    pub created_at: NaiveDateTime,
}