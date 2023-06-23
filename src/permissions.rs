use crate::common::{DbError, DbPool, ServiceError};
use actix_web::{get, web, Error, HttpResponse};
use diesel::sql_types::{Integer, Text};
use diesel::{insert_or_ignore_into, prelude::*, sql_query, PgConnection};
use std::collections::HashMap;
use crate::model::Permission;

fn find_all_permissions(conn: &mut PgConnection) -> Result<Vec<Permission>, DbError> {
    let _permissions = sql_query("SELECT * FROM permissions").get_results(conn)?;

    Ok(_permissions)
}

fn find_all_permissions_for_role(
    conn: &mut PgConnection,
    role_id: i32,
) -> Result<Vec<Permission>, DbError> {
    let _permissions = sql_query("SELECT p.* FROM role_permissions rp join permissions p on rp.permission_id = p.id where rp.role_id=?")
    .bind::<Integer, _>(role_id)
    .get_results(conn)?;

    Ok(_permissions)
}

fn find_permissions_for_user_and_company(
    conn: &mut PgConnection,
    user_id: i32,
    company_id: i32,
    application: String,
) -> Result<Vec<Permission>, DbError> {
    let _permissions = sql_query(
        r#"select p.* 
        from 
          users u 
            join user_company_permissions ucp on u.id = ucp.user_id 
            join permissions p on p.id = ucp.permission_id and p.active = true
        where 
          u.id=? and u.active = true and ucp.company_id=? and p.name like ?
        union select p.*
        from 
          users u 
            join user_roles ur on u.id = ur.user_id
            join role_permissions rp on rp.role_id = ur.role_id 
            join permissions p on p.id = rp.permission_id and p.active = true
        where 
          u.id=? and u.active = true and p.name like ?
        "#,
    )
    .bind::<Integer, _>(user_id)
    .bind::<Integer, _>(company_id)
    .bind::<Text, _>(&application)
    .bind::<Integer, _>(user_id)
    .bind::<Text, _>(&application)
    .get_results(conn)?;

    Ok(_permissions)
}

#[get("/permissions")]
pub async fn get_permissions(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let permissions = web::block(move || {
        let mut conn = pool.get()?;
        find_all_permissions(&mut conn)
    })
    .await?
    .map_err(|err| ServiceError::InternalServerError(err.to_string()))?;

    Ok(HttpResponse::Ok().json(permissions))
}

#[get("/user/{user_id}/company/{company_id}/permissions")]
pub async fn get_permissions_for_user_and_company(
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
pub async fn get_permissions_for_roles(
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
