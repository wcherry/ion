use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, QueryableByName, ToSchema)]
#[diesel(table_name = super::schema::page_view)]
pub struct PageDto {
    id: String,
    #[schema(example = "My Page")] 
	name: String,
    #[serde(rename = "ownerId")]
    owner_id: Option<i32>,
    #[serde(rename = "parentPageId")]
    parent_page_id: Option<String>,
    #[serde(rename = "createdAt")]
    created_at: NaiveDateTime,
    #[serde(rename = "updatedAt")]
    updated_at: NaiveDateTime,
    active: bool,
    version: i32,
    #[serde(rename = "pageVersionId")]
    page_version_id: String
}

#[derive(Debug,Deserialize,ToSchema)]
pub struct PageCreateDto {
    #[schema(example = "My New Page")]
    pub name: String,
    #[serde(rename = "parentPageId")]
    #[schema(example = "UUID of the parent page", value_type = uuid::Uuid)]
    pub parent_page_id: String,
    #[schema(example = "The content of the first block on the page")]
    pub content: Option<String>
}
