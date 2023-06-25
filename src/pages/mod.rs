use crate::common::{DbError, DbPool, ServiceError};
use actix_web::{get, web, Error, HttpResponse};
use diesel::{prelude::*, sql_query, sql_types::VarChar, PgConnection};
use self::dto::PageDto;

mod schema;
mod dto;

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
pub async fn get_pages(
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
