use crate::{shared::common::{DbError, ServiceError}, AppState};
use actix_web::{get, web, Error, HttpResponse, post};
use diesel::{prelude::*, sql_query, sql_types, PgConnection};
use self::dto::{PageDto,PageCreateDto};

mod schema;
mod dto;

const DEFAULT_BLOCK_TYPE: &str = "paragraph";

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
        .bind::<sql_types::VarChar, _>(page_id)
        .get_result::<PageDto>(conn)?;

    Ok(page)
}

fn create_page(conn: &mut PgConnection, page: PageCreateDto) -> Result<PageDto, DbError> {
    let page_id = uuid::Uuid::new_v4();
    let page_version_id = uuid::Uuid::new_v4();
    let block_id = uuid::Uuid::new_v4();

    let _id = diesel::sql_query( "INSERT INTO pages (id, name, owner_id, company_id, team_id) values($1,$2,$3,$4,$5)")
        .bind::<sql_types::Uuid, _>(page_id)
        .bind::<sql_types::VarChar, _>(page.name)
        .bind::<sql_types::Integer, _>(0)
        .bind::<sql_types::Integer, _>(0)
        .bind::<sql_types::Integer, _>(0)
        .execute(conn)?;
    
    let _id = diesel::sql_query( "INSERT INTO page_versions (id, page_id, version) VALUES ($1,$2,$3)")
        .bind::<sql_types::Uuid, _>(page_version_id)
        .bind::<sql_types::Uuid, _>(page_id)
        .bind::<sql_types::Integer, _>(1)
        .execute(conn)?;
    
    let _id = diesel::sql_query( "INSERT INTO blocks (id, block_id, `version`, block_type, CONTENT,created_by,updated_by) values($1,uuid_generate_v4(),$2,$3,$4,$5,$6)")
        .bind::<sql_types::Uuid, _>(block_id)
        .bind::<sql_types::Integer, _>(1)
        .bind::<sql_types::VarChar, _>(DEFAULT_BLOCK_TYPE)
        .bind::<sql_types::VarChar, _>(page.content.unwrap_or("".to_string()))
        .bind::<sql_types::Integer, _>(1)
        .bind::<sql_types::Integer, _>(1)
        .execute(conn)?;
    
    let _id = diesel::sql_query( "INSERT INTO page_block_index (id, page_version_id, display_order, block_id) values(uuid_generate_v4(),$1,$2,$3)")
        .bind::<sql_types::Uuid, _>(page_version_id)
        .bind::<sql_types::Integer, _>(1)
        .bind::<sql_types::Uuid, _>(block_id)
        .execute(conn)?;
    
    let page = find_page(conn, page_id.to_string())?;
    Ok(page)
}

#[get("/page/{page_id}")]
pub async fn get_pages(
    app: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let page_id = path.into_inner();
    let page = web::block(move || {
        let mut conn = app.pool.get()?;
        find_page(&mut conn, page_id)
    })
    .await?
    .map_err(|err| ServiceError::NotFound(err.to_string()))?;

    Ok(HttpResponse::Ok().json(page))
}

#[post("/page")]
pub async fn create_page_handler(
    app: web::Data<AppState>,
    web::Json(body): web::Json<PageCreateDto>,
) -> Result<HttpResponse, Error> {
    let page = web::block(move || {
        let mut conn = app.pool.get()?;
        create_page(&mut conn, body)
    })
    .await?
    .map_err(|err| ServiceError::BadRequest(err.to_string()))?;

    Ok(HttpResponse::Ok().json(page))
}
