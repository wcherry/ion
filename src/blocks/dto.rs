use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

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
        modes -> VarChar,
    }
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    Queryable,
    Insertable,
    QueryableByName,
    PartialEq,
    ToSchema,
)]
#[diesel(table_name = blocks_dtos)]
pub struct BlockDto {
    pub id: String,
    #[serde(rename = "blockId")]
    //#[schema(example = "UUID of the parent page", value_type = uuid::Uuid)]
    pub block_id: String,
    pub version: i32,
    #[serde(rename = "displayOrder")]
    pub display_order: Option<i32>,
    #[serde(rename = "blockType")]
    pub block_type: String,
    pub content: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: NaiveDateTime,
    #[serde(rename = "updatedAt")]
    pub updated_at: NaiveDateTime,
    #[serde(rename = "createdBy")]
    pub created_by: i32,
    #[serde(rename = "updatedBy")]
    pub updated_by: i32,
    pub active: bool,
    pub modes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BlockRequest {
    #[serde(rename = "blockId")]
    pub block_id: Option<String>,
    pub version: Option<i32>,
    #[serde(rename = "pageVersionId")]
    pub page_version_id: Option<String>,
    #[serde(rename = "blockType")]
    pub block_type: String,
    pub content: Option<String>,
    #[serde(rename = "displayOrder")]
    pub display_order: i32,
}
