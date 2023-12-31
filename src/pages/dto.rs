use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, QueryableByName, ToSchema)]
#[diesel(table_name = super::schema::page_view)]
pub struct PageDto {
    pub id: String,
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
    pub page_id: Option<uuid::Uuid>,
    #[schema(example = "My New Page")]
    pub name: String,
    #[serde(rename = "parentPageId")]
    #[schema(example = "UUID of the parent page", value_type = uuid::Uuid)]
    pub parent_page_id: String,
    #[schema(example = "The content of the first block on the page")]
    pub content: Option<String>
}

#[derive(Debug,Deserialize,ToSchema)]
pub struct PagePermissionCreateDto {
    pub page_id: String,
    pub user_id: Option<i32>,
    pub team_id: Option<i32>,
    pub company_id: Option<i32>,
    pub allow_all: bool,
    pub active: bool,
    pub mode: String
}


#[derive(Debug,Deserialize,ToSchema)]
pub struct PageTreeDto {
    id: String,
    name: String,
    children: Vec<PageTreeDto>
}
