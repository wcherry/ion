use crate::{common::{DbError, DbPool, ServiceError}};
use actix_web::{get, post, web, Error, HttpResponse};
use diesel::{insert_into, prelude::*, sql_query, sql_types::Integer, PgConnection};
use log::info;
use serde::{Deserialize, Serialize};
use crate::model::{Company, User};
use crate::model::users::dsl::users;


#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserDto {
    pub id: i32,
    pub name: String,
    pub active: bool,
    pub companies: Vec<Company>,
}


fn insert_user(conn: &mut PgConnection, user: User) -> Result<usize, DbError> {
    // FORNOW: The Mysql Rust connection doesn't support the RETURNING clause
    // so there is no good way to return the record just inserted and not even
    // a good way to return the id of the last record inserted. Need to
    // investigate creating a transaction to
    // 1. insert a new user
    // 2. query the LAST_INSERT_ID() to get its id
    // 3. query the DB to get the newly inserted user
    // ref: https://github.com/diesel-rs/diesel/issues/1011
    // Code left in to denote the way we originally tried to solve this,
    // supposed to work on PG but not MySQL or SqlLite.
    // fn insert_user(conn: &mut MysqlConnection, user: User) -> Result<User, DbError> {
    // let result = insert_into(users).values(user).get_result(conn);
    // Ok(result.unwrap())

    let result = insert_into(users).values(user).execute(conn)?;

    info!("result {}", &result);
    Ok(result)
}

fn find_all_users(conn: &mut PgConnection) -> Result<Vec<User>, DbError> {
    let user = sql_query("SELECT * FROM users").get_results(conn)?;
    Ok(user)
}

fn find_user_with_companies(conn: &mut PgConnection, user_id: i32) -> Result<UserDto, DbError> {
    let user = sql_query("SELECT * FROM users WHERE id=?")
        .bind::<Integer, _>(user_id)
        .get_result::<User>(conn)?;
    let companies = sql_query(
        r#"select unique c.name name, c.id, c.active 
  from companies c 
  join user_company_permissions ucp on c.id=ucp.company_id and ucp.user_id=?
  union select unique c.name name, c.id, c.active 
  from companies c 
  join user_roles ur on c.id=ur.company_id and ur.user_id=?;
  "#,
    )
    .bind::<Integer, _>(user_id)
    .bind::<Integer, _>(user_id)
    .get_results(conn)?;
    return Ok(UserDto {
        id: user.id.unwrap(), // TODO: Proper handling of this error that should never happen
        name: user.name,
        active: user.active,
        companies: companies,
    });
}

#[get("/users")]
pub async fn get_users(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let all_users = web::block(move || {
        let mut conn = pool.get()?;
        find_all_users(&mut conn)
    })
    .await?
    .map_err(|err| ServiceError::InternalServerError(err.to_string()))?;

    // if all_users.len() > 0 {
    info!("Returning {} users", all_users.len());
    Ok(HttpResponse::Ok().json(all_users))
    // } else {
    //     Err("Users".to_string()).map_err(|err| ServiceError::NotFound(err))?
    // }
}

#[get("/user/{user_id}")]
pub async fn get_user(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let user_id = path.into_inner();
    let user = web::block(move || {
        let mut conn = pool.get()?;
        find_user_with_companies(&mut conn, user_id)
    })
    .await?
    .map_err(|err| ServiceError::NotFound(err.to_string()))?;

    Ok(HttpResponse::Ok().json(user))
}

#[post("/user")]
pub async fn create_user(
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
