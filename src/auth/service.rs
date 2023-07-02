use crate::shared::common::DbError;
use diesel::sql_types::VarChar;
use diesel::{prelude::*, sql_query, PgConnection};
// use log::info;

use crate::shared::schema::User;
// use schema::{Role, Permission, Company};


pub fn find_user(conn: &mut PgConnection, username: String) -> Result<User, DbError> {
    let user = sql_query("SELECT 
    u.id,
    u.name,
    u.password,
    u.email_address,
    u.role,
    u.profile_id,
    p.avatar_url,
    p.bio,
    p.default_page_id::text,
    pv.id::text page_version_id,
    u.company_id,
    u.created_at,
    u.updated_at,
    u.created_by,
    u.updated_by,
    u.active
FROM users u 
LEFT JOIN profile p ON u.profile_id = p.id
LEFT JOIN page_versions pv ON p.default_page_id = pv.page_id 
WHERE name = $1")
    .bind::<VarChar, _>(username)
    .get_result::<User>(conn)?;
    Ok(user)
}

pub fn is_exists(conn: &mut PgConnection, username: String) -> Result<bool, DbError> {
    // let exists: i64 = sql_query("SELECT count(*) id FROM users WHERE name = $1")
    // .bind::<VarChar, _>(username)
    // .get_result(conn)?;
    use crate::shared::schema::users::dsl::*;
    let exists: i64= users.filter(name.eq(username)).count().get_result(conn)?; // Result<i64, Error>

    Ok(exists == 1)
}
//body.name.to_owned(), body.email.to_owned(), hashed_password
pub fn create_user(conn: &mut PgConnection, username: String, email: String, hashed_password: String) -> Result<User, DbError> {
    
    let user = sql_query( "INSERT INTO users (name,email_address,password,role) VALUES ($1, $2, $3,'admin') RETURNING *")
    .bind::<VarChar, _>(username)
    .bind::<VarChar, _>(email)
    .bind::<VarChar, _>(hashed_password)
    .get_result::<User>(conn)?;
    Ok(user)
}