mod dto;
mod schema;
mod service;

use std::collections::HashMap;

use crate::shared::{
    common::{DbPool, ServiceError},
    schema::User,
};
use actix_web::{get, post, web, Error, HttpResponse};
use log::info;
use service::{
    find_all_permissions, find_all_permissions_for_role, find_all_roles, find_all_users,
    find_permissions_for_user_and_company, find_role, insert_user,
};

#[get("/")]
async fn get_users(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let all_users = web::block(move || {
        let mut conn = pool.get()?;
        find_all_users(&mut conn)
    })
    .await?
    .map_err(|err| ServiceError::InternalServerError(err.to_string()))?;

    info!("Returning {} users", all_users.len());
    Ok(HttpResponse::Ok().json(all_users))
}

#[post("/")]
async fn create_user(
    pool: web::Data<DbPool>,
    web::Json(body): web::Json<User>,
) -> Result<HttpResponse, Error> {
    web::block(move || {
        let mut conn = pool.get()?;
        insert_user(&mut conn, body)
    })
    .await?
    .map_err(|err| ServiceError::BadRequest(err.to_string()))?;

    Ok(HttpResponse::Ok().json("Saved User"))
}

#[get("/permissions")]
async fn get_permissions(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let permissions = web::block(move || {
        let mut conn = pool.get()?;
        find_all_permissions(&mut conn)
    })
    .await?
    .map_err(|err| ServiceError::InternalServerError(err.to_string()))?;

    Ok(HttpResponse::Ok().json(permissions))
}

#[get("/{user_id}/company/{company_id}/permissions")]
async fn get_permissions_for_user_and_company(
    pool: web::Data<DbPool>,
    path: web::Path<(i32, i32)>,
    web::Query(query): web::Query<HashMap<String, String>>,
    //web::Json(thing): web::Json<Thing> // web::Json extractor for json body.
) -> Result<HttpResponse, Error> {
    let (user_id, company_id) = path.into_inner();
    let application = query.get("application");
    let application = match application {
        Some(application) => format!("{}%", application),
        None => "%".to_string(),
    };

    let _permissions = web::block(move || {
        let mut conn = pool.get()?;
        find_permissions_for_user_and_company(&mut conn, user_id, company_id, application)
    })
    .await?
    .map_err(|err| ServiceError::InternalServerError(err.to_string()))?;

    let result = _permissions
        .into_iter()
        .map(|it| it.name)
        .collect::<Vec<String>>();

    // let filtered = match application_op {
    //     Some(application) => result
    //         .into_iter()
    //         .filter(|it| it.starts_with(application))
    //         .collect::<Vec<String>>(),
    //     None => result,
    // };

    Ok(HttpResponse::Ok().json(result))
}

#[get("/role/{role_id}/permissions")]
async fn get_permissions_for_roles(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    web::Query(_query): web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, Error> {
    let role_id = path.into_inner();
    let permissions = web::block(move || {
        let mut conn = pool.get()?;
        find_all_permissions_for_role(&mut conn, role_id)
    })
    .await?
    .map_err(|err| ServiceError::InternalServerError(err.to_string()))?;

    Ok(HttpResponse::Ok().json(permissions))
}

// #[put("/role/{role_id}/permissions")]
// pub async fn save_permissions_for_roles(
//     pool: web::Data<DbPool>,
//     path: web::Path<i32>,
//     web::Query(_query): web::Query<HashMap<String, String>>,
//     web::Json(body): web::Json<Vec<Permission>>, // web::Json extractor for json body.
// ) -> Result<HttpResponse, Error> {
//     let role_id = path.into_inner();
//     let mut rp: Vec<RolePermission> = vec![];
//     for p in body {
//         let p: RolePermission = RolePermission {
//             id: None,
//             role_id: role_id,
//             permission_id: p.id,
//         };
//         rp.push(p);
//     }

//     let status = web::block(move || {
//         let mut conn = pool.get()?;
//         upsert_role_permission(&mut conn, rp)
//     })
//     .await?;

//     Ok(HttpResponse::Ok().json(format!("added {} new permissions", status.unwrap())))
// }

#[get("/roles")]
async fn get_all_roles(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let roles = web::block(move || {
        let mut conn = pool.get()?;
        find_all_roles(&mut conn)
    })
    .await?
    .map_err(|err| ServiceError::InternalServerError(err.to_string()))?;

    info!("Returning {} roles", roles.len());
    Ok(HttpResponse::Ok().json(roles))
}

#[get("/role/{role_id}")]
async fn get_role(pool: web::Data<DbPool>, path: web::Path<i32>) -> Result<HttpResponse, Error> {
    let role_id = path.into_inner();
    let role = web::block(move || {
        let mut conn = pool.get()?;
        find_role(&mut conn, role_id)
    })
    .await?
    .map_err(|err| ServiceError::InternalServerError(err.to_string()))?;

    Ok(HttpResponse::Ok().json(role))
}

// #[post("/login")]
// pub async fn login_user_handler(
//     pool: web::Data<DbPool>,
//     web::Json(body): web::Json<LoginRequestDto>,
// ) -> Result<HttpResponse, Error> {
//     let user = web::block(move || {
//         let mut conn = pool.get()?;
//         find_user(&mut conn, body.username.clone(), body.password.clone())
//     })
//     .await?
//     .map_err(|err| ServiceError::InternalServerError(err.to_string()))?;

//     Ok(HttpResponse::Ok().json(user))
// }

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/user")
        .service(get_all_roles)
        .service(get_permissions)
        .service(get_permissions_for_roles)
        .service(get_permissions_for_user_and_company)
        .service(get_role)
        // .service(get_user)
        .service(get_users);
    conf.service(scope);
}
