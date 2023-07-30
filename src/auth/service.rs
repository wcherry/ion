use crate::pages::dto::PageCreateDto;
use crate::shared::common::DbError;
use diesel::sql_types::{Integer, Uuid, VarChar};
use diesel::{prelude::*, sql_query, PgConnection};
// use log::info;

use crate::pages::service::create_page;
use crate::shared::schema::{Profile, User, UserProfile};
// use schema::{Role, Permission, Company};

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub fn find_user(conn: &mut PgConnection, username: String) -> Result<UserProfile, DbError> {
    let user = sql_query(
        "SELECT 
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
WHERE name = $1",
    )
    .bind::<VarChar, _>(username)
    .get_result::<UserProfile>(conn)?;
    Ok(user)
}

pub fn is_exists(conn: &mut PgConnection, username: String) -> Result<bool, DbError> {
    // let exists: i64 = sql_query("SELECT count(*) id FROM users WHERE name = $1")
    // .bind::<VarChar, _>(username)
    // .get_result(conn)?;
    use crate::shared::schema::users::dsl::*;
    let exists: i64 = users.filter(name.eq(username)).count().get_result(conn)?; // Result<i64, Error>

    Ok(exists == 1)
}

pub fn create_user(
    conn: &mut PgConnection,
    username: String,
    email: String,
    password: String,
) -> Result<User, DbError> {
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .expect("Error while hashing password")
        .to_string();
    let page_id = uuid::Uuid::new_v4();

    // Create profile object
    // let _profile = sql_query( "INSERT INTO profile (default_page_id) VALUES (uuid($1)) RETURNING *")
    //     .bind::<VarChar, _>(&page_id)
    //     .get_result::<Profile>(conn)?;
    let profile = sql_query("INSERT INTO profile (default_page_id) VALUES (uuid($1)) RETURNING *")
        .bind::<Uuid, _>(&page_id)
        .get_result::<Profile>(conn)?;

    // Create user object
    let user = sql_query( "INSERT INTO users (name,email_address,password,role,profile_id) VALUES ($1, $2, $3,'admin', $4) RETURNING *")
        .bind::<VarChar, _>(&username)
        .bind::<VarChar, _>(email)
        .bind::<VarChar, _>(hashed_password)
        .bind::<Integer, _>(&profile.id)
        .get_result::<User>(conn)?;

    // Create page object
    let page: PageCreateDto = PageCreateDto {
        page_id: Some(page_id),
        name: format!("{username} Home"),
        parent_page_id: uuid::Uuid::new_v4().to_string(),
        content: Some("".to_string()),
    };
    let _page = create_page(conn, page, user.id)?;

    Ok(user)
}
