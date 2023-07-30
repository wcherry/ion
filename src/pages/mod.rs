use crate::{shared::common::{ServiceError}, AppState};
use actix_web::{get, web, Error, HttpResponse, post};
use crate::auth::jwt_auth;
use self::dto::{PageCreateDto, PagePermissionCreateDto};

mod schema;
pub(crate) mod service;
pub mod dto;


///
/// Retrieve a page by id (UUID)
/// 
#[utoipa::path(
    get,
    path = "/page",
    params(("page_id",description = "UUID of page to retreive",),),
    tag = "Pages",
    responses(
        (status = 200, description = "Successfully retreived a page", body = [PageDto])
    )
)]
#[get("/{page_id}")]
pub async fn get_page_handler(
    app: web::Data<AppState>,
    jwt: jwt_auth::JwtMiddleware,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let user_id = jwt.user_id;
    println!("user_id: {}", user_id);
    let page_id = path.into_inner();
    let page = web::block(move || {
        let mut conn = app.pool.get()?;
        service::find_page(&mut conn, page_id, user_id)
    })
    .await?
    .map_err(|err| ServiceError::NotFound(err.to_string()))?;

    Ok(HttpResponse::Ok().json(page))
}

///
/// Retrieve all pages for a user
/// 
#[utoipa::path(
    get,
    path = "/pages",
    tag = "Pages",
    responses(
        (status = 200, description = "Successfully retreived all page", body = [Vec<PageDto>])
    )
)]
#[get("/pages")]
pub async fn get_pages_handler(
    app: web::Data<AppState>,
    jwt: jwt_auth::JwtMiddleware,
) -> Result<HttpResponse, Error> {
    let user_id = jwt.user_id;
    println!("user_id: {}", user_id);
    let pages = web::block(move || {
        let mut conn = app.pool.get()?;
        service::find_all_pages(&mut conn, user_id)
    })
    .await?
    .map_err(|err| ServiceError::NotFound(err.to_string()))?;
    Ok(HttpResponse::Ok().json(pages))
}

///
/// Creates a new page and an empty block
/// 
#[utoipa::path(
    post,
    path = "/page",
    request_body = PageCreateDto,
    tag = "Pages",
    responses(
        (status = 200, description = "Successfully created a new page", body = [PageDto])
    )
)]
#[post("")]
pub(super) async fn create_page_handler(
    app: web::Data<AppState>,
    jwt: jwt_auth::JwtMiddleware,
    web::Json(body): web::Json<PageCreateDto>,
) -> Result<HttpResponse, Error> {
    let page = web::block(move || {
        let mut conn = app.pool.get()?;
        service::create_page(&mut conn, body, jwt.user_id)
    })
    .await?
    .map_err(|err| ServiceError::BadRequest(err.to_string()))?;

    Ok(HttpResponse::Ok().json(page))
}

///
/// Creates permissions for a page
/// 
#[utoipa::path(
    post,
    path = "page/{page_id}/permission",
    request_body = PagePermissionCreateDto,
    tag = "Pages",
    responses(
        (status = 200, description = "Successfully created a new page", body = [i32])
    )
)]
#[post("/{page_id}/permission")]
pub(super) async fn create_page_permission_handler(
    app: web::Data<AppState>,
    jwt: jwt_auth::JwtMiddleware,
    web::Json(body): web::Json<PagePermissionCreateDto>,
) -> Result<HttpResponse, Error> {
    let id = web::block(move || {
        let mut conn = app.pool.get()?;
        service::create_page_permission(&mut conn, body, jwt.user_id)
    })
    .await?
    .map_err(|err| ServiceError::BadRequest(err.to_string()))?;

    Ok(HttpResponse::Ok().json(id))
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("")
        .service(get_pages_handler)
        .service(web::scope("/page")
            .service(get_page_handler)
            .service(create_page_handler)
            .service(create_page_permission_handler))    
            ;

    conf.service(scope);
}
