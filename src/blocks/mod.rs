use actix_web::{get, post, put, web, Error, HttpResponse};
use self::service::{find_blocks_by_page_version, create_block_and_attach_to_page, update_block};
use super::common::{DbPool, ServiceError};
use self::dto::BlockRequest;

mod service;
mod schema;
mod dto;


#[get("/page-version/{page_version_id}/blocks")]
pub async fn get_blocks(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let page_version_id = path.into_inner();
    let blocks = web::block(move || {
        let mut conn = pool.get()?;
        find_blocks_by_page_version(&mut conn, page_version_id)
    })
    .await?
    .map_err(|err| ServiceError::NotFound(err.to_string()))?;

    Ok(HttpResponse::Ok().json(blocks))
}


#[post("/block")]
pub async fn create_block(
    pool: web::Data<DbPool>,
    web::Json(body): web::Json<BlockRequest>,
) -> Result<HttpResponse, Error> {
    let blocks = web::block(move || {
        let mut conn = pool.get()?;
        create_block_and_attach_to_page(&mut conn, body)
    })
    .await?
    .map_err(|err| ServiceError::BadRequest(err.to_string()))?;

    Ok(HttpResponse::Ok().json(blocks))
}

#[put("/block/{block_id}")]
pub async fn update_block_handler(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
    web::Json(body): web::Json<BlockRequest>,
) -> Result<HttpResponse, Error> {
    let block_id: String = path.into_inner();
    let blocks = web::block(move || {
        let mut conn = pool.get()?;
        update_block(&mut conn, block_id, body)
    })
    .await?
    .map_err(|err| ServiceError::BadRequest(err.to_string()))?;

    Ok(HttpResponse::Ok().json(blocks))
}

