use chrono::prelude::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct PageDetails{
    pub id: i32,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub deleted: bool,
    pub raw_content: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Page {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub deleted: bool,
    pub versions: Vec<PageDetails>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PageRequest {
    pub id: Option<i32>,        // used to save a version
    pub name: Option<String>,   // used when we create a new page
    pub raw_content: String,
}