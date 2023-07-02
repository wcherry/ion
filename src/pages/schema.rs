use chrono::{NaiveDateTime,};
use serde::{Deserialize, Serialize};

table! {
    page_view (id) {
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

table! {
    pages (id) {
        id -> VarChar,
        name -> VarChar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        active -> Bool,
    }
  }
  
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, QueryableByName, PartialEq,)]
pub struct Page {
    pub id: String,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub active: bool,
}
