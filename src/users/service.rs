use crate::shared::common::DbError;
use diesel::sql_types::{Integer, VarChar};
use diesel::{insert_into, prelude::*, sql_query, PgConnection};
use log::info;

use crate::shared::schema::{User, users};
use super::schema::{Role, Permission};


pub fn insert_user(conn: &mut PgConnection, user: User) -> Result<usize, DbError> {
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

    let result = insert_into(users::table).values(user).execute(conn)?;

    info!("result {}", &result);
    Ok(result)
}

pub fn find_all_users(conn: &mut PgConnection) -> Result<Vec<User>, DbError> {
    let user = sql_query("SELECT * FROM users").get_results(conn)?;
    Ok(user)
}

// pub fn find_user(conn: &mut PgConnection, username: String, password: String) -> Result<User, DbError> {
//     let user = sql_query("SELECT 
//     u.id,
//     u.name,
//     u.email_address,
//     u.role,
//     u.profile_id,
//     p.avatar_url,
//     p.bio,
//     p.default_page_id::text,
//     pv.id::text page_version_id,
//     u.company_id,
//     u.created_at,
//     u.updated_at,
//     u.created_by,
//     u.updated_by,
//     u.active
// FROM users u 
// LEFT JOIN profile p ON u.profile_id = p.id
// LEFT JOIN page_versions pv ON p.default_page_id = pv.page_id 
// WHERE name = $1 AND password = $2")
//     .bind::<VarChar, _>(username)
//     .bind::<VarChar, _>(password)
//     .get_result::<User>(conn)?;
//     Ok(user)
// }

// pub fn find_user_with_companies(conn: &mut PgConnection, user_id: i32) -> Result<UserDto, DbError> {
//     let user = sql_query("SELECT * FROM users WHERE id=?")
//         .bind::<Integer, _>(user_id)
//         .get_result::<User>(conn)?;
//     let companies: Vec<Company> = sql_query(
//         r#"select unique c.name name, c.id, c.active 
//   from companies c 
//   join user_company_permissions ucp on c.id=ucp.company_id and ucp.user_id=?
//   union select unique c.name name, c.id, c.active 
//   from companies c 
//   join user_roles ur on c.id=ur.company_id and ur.user_id=?;
//   "#,
//     )
//     .bind::<Integer, _>(user_id)
//     .bind::<Integer, _>(user_id)
//     .get_results(conn)?;
//     return Ok(UserDto {
//         id: user.id, // TODO: Proper handling of this error that should never happen
//         name: user.name,
//         active: user.active,
//         companies: companies.into_iter().map(|it| CompanyDto::from(it)).collect(),
//     });
// }

pub fn find_all_permissions(conn: &mut PgConnection) -> Result<Vec<Permission>, DbError> {
    let _permissions = sql_query("SELECT * FROM permissions").get_results(conn)?;

    Ok(_permissions)
}

pub fn find_all_permissions_for_role(
    conn: &mut PgConnection,
    role_id: i32,
) -> Result<Vec<Permission>, DbError> {
    let _permissions = sql_query("SELECT p.* FROM role_permissions rp join permissions p on rp.permission_id = p.id where rp.role_id=?")
    .bind::<Integer, _>(role_id)
    .get_results(conn)?;

    Ok(_permissions)
}

pub fn find_permissions_for_user_and_company(
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
    .bind::<VarChar, _>(&application)
    .bind::<Integer, _>(user_id)
    .bind::<VarChar, _>(&application)
    .get_results(conn)?;

    Ok(_permissions)
}

pub fn find_all_roles(conn: &mut PgConnection) -> Result<Vec<Role>, DbError> {
    let roles = sql_query("SELECT r.id, r.name, r.active, c.name company_name FROM roles r join companies c on r.company_id = c.id")
        .get_results(conn)?;
    Ok(roles)
}

pub fn find_role(conn: &mut PgConnection, role_id: i32) -> Result<Role, DbError> {
    let role = sql_query("SELECT r.id, r.name, r.active, c.name company_name FROM roles r join companies c on r.company_id = c.id where r.id=?")
    .bind::<Integer, _>(role_id)
        .get_result(conn)?;
    Ok(role)
}

