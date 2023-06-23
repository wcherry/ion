use chrono::{NaiveDateTime,};
use diesel::sql_types::Timestamp;
use serde::{Deserialize, Serialize};


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

table! {
    permissions (id) {
      id -> Integer,
      name -> Text,
      active -> Bool,
    }
  }
  

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, QueryableByName, PartialEq,)]
pub struct Permission {
    pub id: i32,
    pub name: String,
    pub active: bool,
}

#[derive(Queryable)]
pub struct RolePermission {
    pub id: Option<i32>,
    pub role_id: i32,
    pub permission_id: i32,
}

table! {
    users (id) {
        id -> Nullable<Integer>,
        name -> Text,
        active -> Bool,
    }
  }
  
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, QueryableByName, PartialEq,)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub active: bool,
}

table! {
    companys (id) {
      id -> Integer,
      name -> Text,
      active -> Bool,
    }
  }
  
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, QueryableByName, PartialEq,)]
pub struct Company {
    pub id: i32,
    pub name: String,
    pub active: bool,
}

table! {
    roles (id) {
        id -> Integer,
        name -> Text,
        company_name -> Text,
        active -> Bool,
    }
  }
  
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, QueryableByName, PartialEq,)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub company_name: String,
    pub active: bool,
}
