use crate::common::{DbError, DbPool, ServiceError};
use actix_web::{get, web, Error, HttpResponse};
use diesel::{prelude::*, sql_query, sql_types::VarChar, PgConnection};
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

pub struct PageDetails {
    pub id: i32,
    pub version: i32,
    pub created_at: String,
    pub deleted: bool,
    pub raw_content: Option<String>,
}


fn find_page(conn: &mut PgConnection, page_id: String) -> Result<PageDto, DbError> {
    let page: PageDto = sql_query("select 
	p.id::text id, 
	p.name,
    p.owner_id,
    p.company_id,
    p.team_id,
    p.created_at,
    p.updated_at,
    p.active,
    v.version,
    v.id::text page_version_id
from pages p 
	join page_versions v on p.id = v.page_id 
where p.id = uuid($1)
order by v.version desc limit 1;
")
        .bind::<VarChar, _>(page_id)
        .get_result::<PageDto>(conn)?;

    Ok(page)
}

#[get("/page/{page_id}")]
pub async fn get_pages_handler(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let page_id = path.into_inner();
    let page = web::block(move || {
        let mut conn = pool.get()?;
        find_page(&mut conn, page_id)
    })
    .await?
    .map_err(|err| ServiceError::NotFound(err.to_string()))?;

    Ok(HttpResponse::Ok().json(page))
}
