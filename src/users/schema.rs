// use diesel::{prelude::*};
use serde::{Deserialize, Serialize};

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
    companys (id) {
      id -> Integer,
      name -> VarChar,
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
        name -> VarChar,
        company_name -> VarChar,
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
