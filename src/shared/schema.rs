// use diesel::{prelude::*};
use ::utoipa::ToSchema;
use serde::{Deserialize, Serialize};


table! {
  user_profiles (id) {
      id -> Integer,
      name -> VarChar,
      password -> VarChar,
      email_address -> VarChar,
      role  -> VarChar,
      profile_id -> Nullable<Integer>,
      avatar_url -> Nullable<Text>,
      bio -> Nullable<Text>,
      default_page_id -> Nullable<VarChar>,
      page_version_id -> Nullable<VarChar>,
      company_id -> Nullable<Integer>,
      created_at -> Timestamp,
      updated_at -> Timestamp,
      created_by -> Integer,
      updated_by -> Integer,
      active -> Bool
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
pub struct UserProfile {
    pub id: i32,
    pub name: String,
    pub password: String,
    pub email_address: String,
    pub role: String,
    pub profile_id: Option<i32>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub default_page_id: Option<String>,
    pub page_version_id: Option<String>,
    pub company_id: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub created_by: i32,
    pub updated_by: i32,
    pub active: bool,
}

table! {
  users (id) {
      id -> Integer,
      name -> VarChar,
      password -> VarChar,
      email_address -> VarChar,
      role  -> VarChar,
      profile_id -> Nullable<Integer>,
      // avatar_url -> Nullable<VarChar>,
      // bio -> Nullable<Text>,
      // default_page_id -> Uuid,
      // page_version_id -> Uuid,
      company_id -> Nullable<Integer>,
      created_at -> Timestamp,
      updated_at -> Timestamp,
      created_by -> Integer,
      updated_by -> Integer,
      active -> Bool
  }
}

#[derive(
    Debug, Clone, Serialize, Deserialize, Queryable, Insertable, QueryableByName, PartialEq,
)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub password: String,
    pub email_address: String,
    pub role: String,
    pub profile_id: Option<i32>,
    // pub avatar_url: Option<String>,
    // pub bio: Option<String>,
    // pub default_page_id: uuid::Uuid,
    // pub page_version_id: uuid::Uuid,
    pub company_id: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub created_by: i32,
    pub updated_by: i32,
    pub active: bool,
}

table! {
  profile (id) {
    id -> Integer,
    avatar_url -> Nullable<VarChar>,
    bio -> Nullable<Text>,
    default_page_id -> Uuid,
    created_at -> Timestamp,
    updated_at -> Timestamp,
    created_by -> Integer,
    updated_by -> Integer,
    active -> Bool,
  }
}

#[derive(
    Debug, Clone, Serialize, Deserialize, Queryable, Insertable, QueryableByName, PartialEq,
)]
#[table_name = "profile"]
pub struct Profile {
    pub id: i32,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub default_page_id: uuid::Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub created_by: i32,
    pub updated_by: i32,
    pub active: bool,
}
