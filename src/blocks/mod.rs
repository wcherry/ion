pub mod dto;
mod schema;
mod service;

use crate::AppState;
use actix_web::{get, post, put, web, Error, HttpResponse};

use self::dto::BlockRequest;
use self::service::{
    create_block_and_attach_to_page, find_blocks_by_page, find_blocks_by_page_version, update_block,
};
use super::shared::common::ServiceError;
use crate::auth::jwt_auth;

///
/// Gets a list of blocks for a page version
///
#[utoipa::path(
    get,
    path = "/page-version/{page_version_id}/blocks",
    tag = "Blocks",
    responses(
        (status = 200, description = "Successfully retreived a list of blocks ", body = [Vec<BlockDto>])
    )
)]
#[get("/page-version/{page_version_id}/blocks")]
pub async fn get_blocks_by_version_handler(
    app: web::Data<AppState>,
    jwt: jwt_auth::JwtMiddleware,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let page_version_id = path.into_inner();
    let blocks = web::block(move || {
        let mut conn = app.get_connection()?;
        find_blocks_by_page_version(&mut conn, page_version_id, jwt.user_id)
    })
    .await?
    .map_err(|err| ServiceError::NotFound(err.to_string()))?;

    Ok(HttpResponse::Ok().json(blocks))
}

///
/// Gets a list of blocks for a page
///
#[utoipa::path(
    get,
    path = "/page/{page_id}/blocks",
    tag = "Blocks",
    responses(
        (status = 200, description = "Successfully retreived a list of blocks ", body = [Vec<BlockDto>])
    )
)]
#[get("/page/{page_id}/blocks")]
pub async fn get_blocks_for_page_handler(
    app: web::Data<AppState>,
    jwt: jwt_auth::JwtMiddleware,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let page_id = path.into_inner();
    let blocks = web::block(move || {
        let mut conn = app.get_connection()?;
        find_blocks_by_page(&mut conn, page_id, jwt.user_id)
    })
    .await?
    .map_err(|err| ServiceError::NotFound(err.to_string()))?;

    Ok(HttpResponse::Ok().json(blocks))
}

///
/// Creates a new block and attaches it to a page version
///
#[utoipa::path(
    post,
    path = "/page-version/{page_version_id}/block",
    request_body = BlockRequest,
    tag = "Blocks",
    responses(
        (status = 200, description = "Successfully created a new block", body = [BlockDto])
    )
)]
#[post("/page-version/{page_version_id}/block")]
pub async fn create_block_handler(
    app: web::Data<AppState>,
    web::Json(body): web::Json<BlockRequest>,
) -> Result<HttpResponse, Error> {
    let block = web::block(move || {
        let mut conn = app.get_connection()?;
        create_block_and_attach_to_page(&mut conn, body)
    })
    .await?
    .map_err(|err| ServiceError::BadRequest(err.to_string()))?;

    Ok(HttpResponse::Ok().json(block))
}

///
/// Updates a block
///
#[utoipa::path(
    put,
    path = "/page-version/{page_version_id}/block/{block_id}",
    request_body = BlockRequest,
    tag = "Blocks",
    responses(
        (status = 200, description = "Successfully update an existing block", body = [BlockDto])
    )
)]
#[put("/page-version/{page_version_id}/block/{block_id}")]
pub async fn update_block_handler(
    app: web::Data<AppState>,
    path: web::Path<(String, String)>,
    web::Json(body): web::Json<BlockRequest>,
) -> Result<HttpResponse, Error> {
    let (_page_version_id, block_id) = path.into_inner();
    let block = web::block(move || {
        let mut conn = app.get_connection()?;
        update_block(&mut conn, block_id, body)
    })
    .await?
    .map_err(|err| ServiceError::BadRequest(err.to_string()))?;

    Ok(HttpResponse::Ok().json(block))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_blocks_by_version_handler);
    cfg.service(get_blocks_for_page_handler);
    cfg.service(create_block_handler);
    cfg.service(update_block_handler);
}
