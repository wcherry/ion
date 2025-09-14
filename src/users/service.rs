use diesel::sql_types::{self, Integer, VarChar};
use diesel::{prelude::*, sql_query};
// use log::info;

use super::schema::{Permission, Role};
use crate::shared::schema::{User};
use crate::shared::common::{Connection, DbError};

// pub fn insert_user(conn: &mut Connection, user: NewUserDto) -> Result<usize, DbError> {
//     // FORNOW: The Mysql Rust connection doesn't support the RETURNING clause
//     // so there is no good way to return the record just inserted and not even
//     // a good way to return the id of the last record inserted. Need to
//     // investigate creating a transaction to
//     // 1. insert a new user
//     // 2. query the LAST_INSERT_ID() to get its id
//     // 3. query the DB to get the newly inserted user
//     // ref: https://github.com/diesel-rs/diesel/issues/1011
//     // Code left in to denote the way we originally tried to solve this,
//     // supposed to work on PG but not MySQL or SqlLite.
//     // fn insert_user(conn: &mut MysqlConnection, user: User) -> Result<User, DbError> {
//     // let result = insert_into(users).values(user).get_result(conn);
//     // Ok(result.unwrap())

//     let new_user = User {
//         id: None, // Assuming `id` is auto-generated
//         name: user.name,
//         email: user.email,
//         password: user.password,
//         active: true, // Default value for active
//     };

//     let result = insert_into(users::table).values(new_user).execute(conn)?;

//     info!("result {}", &result);
//     Ok(result)
// }

pub fn find_all_users(conn: &mut Connection) -> Result<Vec<User>, DbError> {
    let user = sql_query("SELECT * FROM users").get_results::<User>(conn)?;
    Ok(user)
}


pub fn load_user(conn: &mut Connection, user_id: i32) -> Result<User, DbError> {
    let user: User = sql_query("SELECT * FROM users u WHERE id = $1 AND active = true")
        .bind::<Integer, _>(user_id)
        .get_result::<User>(conn)?;
    Ok(user)
}

// pub fn find_user_with_companies(conn: &mut Connection, user_id: i32) -> Result<UserDto, DbError> {
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

pub fn find_all_permissions(conn: &mut Connection) -> Result<Vec<Permission>, DbError> {
    let _permissions = sql_query("SELECT * FROM permissions").get_results(conn)?;

    Ok(_permissions)
}

pub fn find_all_permissions_for_role(
    conn: &mut Connection,
    role_id: i32,
) -> Result<Vec<Permission>, DbError> {
    let _permissions = sql_query("SELECT p.* FROM role_permissions rp join permissions p on rp.permission_id = p.id where rp.role_id=?")
    .bind::<Integer, _>(role_id)
    .get_results(conn)?;

    Ok(_permissions)
}

pub fn find_permissions_for_user_and_company(
    conn: &mut Connection,
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

pub fn find_all_roles(conn: &mut Connection) -> Result<Vec<Role>, DbError> {
    let roles = sql_query("SELECT r.id, r.name, r.active, c.name company_name FROM roles r join companies c on r.company_id = c.id")
        .get_results(conn)?;
    Ok(roles)
}

pub fn find_role(conn: &mut Connection, role_id: i32) -> Result<Role, DbError> {
    let role = sql_query("SELECT r.id, r.name, r.active, c.name company_name FROM roles r join companies c on r.company_id = c.id where r.id=?")
    .bind::<Integer, _>(role_id)
        .get_result(conn)?;
    Ok(role)
}

pub fn create_profile(
    conn: &mut Connection,
    page_set_id: uuid::Uuid,
) -> Result<usize, DbError> {
    // Example implementation for creating a profile
    let profile_id = diesel::sql_query( 
        "INSERT INTO public.profile(id, avatar_url, bio, default_page_id, created_at, updated_at, created_by, updated_by, active)
VALUES(nextval('profile_id_seq'::regclass), '', '', $1, timezone('utc'::text, now()), timezone('utc'::text, now()), 0, 0, true)")        
        .bind::<sql_types::Uuid, _>(page_set_id)
        .execute(conn)?;

    Ok(profile_id)
}

// pub fn create_user(conn: &mut Connection, user: NewUserDto) -> Result<i32, DbError> {
//     let hashed_password = format!("hashed-{}", user.password); // Replace with actual hashing logic

//     let user_id = insert_user(conn, user)?;

//     Ok(user_id as i32)
// }
