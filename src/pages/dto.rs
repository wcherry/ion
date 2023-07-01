use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

table! {
    pages_dtos (id) {
        id -> VarChar,
        name -> VarChar,
        owner_id -> Nullable<Integer>,
        company_id -> Nullable<Integer>,
        team_id -> Nullable<Integer>,
        parent_page_id -> Nullable<VarChar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        active -> Bool,
        version -> Integer,
        page_version_id -> VarChar
    }
}
  
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, QueryableByName, PartialEq,)]
#[diesel(table_name = pages_dtos)]
pub struct PageDto {
    id: String, 
	name: String,
    #[serde(rename = "ownerId")]
    owner_id: Option<i32>,
    #[serde(rename = "companyId")]
    company_id: Option<i32>,
    #[serde(rename = "teamId")]
    team_id: Option<i32>,
    #[serde(rename = "parentPageId")]
    parent_page_id: Option<String>,
    #[serde(rename = "createdAt")]
    created_at: NaiveDateTime,
    #[serde(rename = "updatedAt")]
    updated_at: NaiveDateTime,
    active: bool,
    version: i32,
    #[serde(rename = "pageVersionId")]
    page_version_id: String}

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq,)]
    pub struct PageCreateDto {
    pub name: String,
    #[serde(rename = "parentPageId")]
    pub parent_page_id: Option<String>,
    pub content: Option<String>
}
