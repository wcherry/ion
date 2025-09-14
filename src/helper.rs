use crate::shared::common::{Connection};
use crate::pages::service as page_service;
use crate::shared::dto::NewUserDto;
use crate::users::service as user_service;
use crate::shared::schema::User;
use std::error::Error;

pub fn create_user_with_initial_data(
    conn: &mut Connection,
    username: String,
    password: String,
    email: String,
    role: String,
    default_page_set: String,
) -> Result<User, Box<dyn Error>> {
    // Create page-set
    // let page_set = page_service::create_page_set(conn, &username, default_page_set)?;

    // Create profile (pointing to default page freo page-set)
    // let profile_id = user_service::create_profile(
    //     conn,
    //     page_set.0,
    // )?;

    // // Create user (pointing to profile)
    // let user_id = user_service::create_user(conn,
    //         NewUserDto{username,
    //         password,
    //         email_address: email,
    //         role,
    //         profile_id: profile_id as i32,
    //     })?;

    // let user = user_service::load_user(conn, user_id)?;
    // Ok(user)
    Err(Box::new(crate::shared::common::IonError { message: "Not implemented".to_string() }))
}