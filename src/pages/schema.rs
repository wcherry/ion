use chrono::{NaiveDateTime,};
use diesel::sql_types::Timestamp;
use serde::{Deserialize, Serialize};

// pub struct PageDetails {
//     pub id: i32,
//     pub version: i32,
//     pub created_at: String,
//     pub deleted: bool,
//     pub raw_content: Option<String>,
// }



#[derive(Queryable)]
pub struct PageDetails{
    pub id: i32,
    pub version: i32,
    pub created_at: Timestamp,
    pub deleted: bool,
    pub raw_content: Option<String>,
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
    //pub versions: Vec<PageDetails>,
}

