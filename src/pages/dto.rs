use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

table! {
    pages_dtos (id) {
        id -> VarChar,
        name -> VarChar,
        owner_id -> Nullable<Integer>,
        company_id -> Nullable<Integer>,
        team_id -> Nullable<Integer>,
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
    owner_id: Option<i32>,
    company_id: Option<i32>,
    team_id: Option<i32>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    active: bool,
    version: i32,
    page_version_id: String}
