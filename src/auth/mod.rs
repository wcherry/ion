pub mod dto;
pub mod jwt_auth;
pub mod service;

use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    get, post, web, Error, HttpResponse,
};

use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
use chrono::{prelude::*, Duration};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;

use dto::RegisterUserDto;

use crate::auth::dto::{LoginRequestDto, TokenClaims};
use crate::shared::common::ServiceError;
use crate::{
    auth::dto::LoginResponseDto,
    shared::dto::{UserDto, UserProfileDto},
    AppState,
};
use service::{create_user, find_user, is_exists};

///
/// Registers a new user
///
/// Creates a default page and block for the user
///
#[utoipa::path(
    post,
    tag = "Authentication",
    path = "/register",
    request_body = RegisterUserDto,
    responses(
        (status = 200, description = "Successfully registered a new user ", body = [UserDto])
    )
)]
#[post("/register")]
pub async fn register_user_handler(
    body: web::Json<RegisterUserDto>,
    app: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let mut conn = app
        .get_connection()
        .map_err(|err| ServiceError::NotFound(err.to_string()))?;

    let exists = is_exists(&mut conn, body.name.to_owned())
        .map_err(|err| ServiceError::NotFound(err.to_string()))?;

    if exists {
        return Ok(HttpResponse::Conflict().json(
            serde_json::json!({"status": "fail","message": "User with that email already exists"}),
        ));
    }

    let user = web::block(move || {
        create_user(
            &mut conn,
            body.name.to_owned(),
            body.email.to_owned(),
            body.password.to_owned(),
        )
    })
    .await?
    .map_err(|err| ServiceError::NotFound(err.to_string()))?;
    let user: UserDto = user.into();
    Ok(HttpResponse::Ok().json(user))
}

///
///  Login a user
///
#[utoipa::path(
    post,
    tag = "Authentication",
    path = "/login",
    request_body = LoginRequestDto,
    responses(
        (status = 200, description = "Successfully registered a new user ", body = [LoginResponseDto])
    )
)]
#[post("/login")]
async fn login_user_handler(
    app: web::Data<AppState>,
    web::Json(body): web::Json<LoginRequestDto>,
) -> Result<HttpResponse, Error> {
    let secret = app.get_config().jwt_secret.clone();
    let mut user = web::block(move || {
        let mut conn = app.get_connection()?;
        find_user(&mut conn, body.username)
    })
    .await?
    .map_err(|err| ServiceError::BadRequest(err.to_string()))?;

    let parsed_hash = PasswordHash::new(&user.password).unwrap();
    let is_valid = Argon2::default()
        .verify_password(body.password.as_bytes(), &parsed_hash)
        .map_or(false, |_| true);

    if !is_valid {
        return Ok(HttpResponse::BadRequest()
            .json(json!({"status": "fail", "message": "Invalid email or password"})));
    }

    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user.id.to_string(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap();

    let cookie = Cookie::build("token", token.to_owned())
        .path("/")
        .max_age(ActixWebDuration::new(60 * 60, 0))
        .http_only(true)
        .finish();
    user.password = "".to_string();

    Ok(HttpResponse::Ok().cookie(cookie).json(LoginResponseDto {
        status: String::from("success"),
        token,
        user: UserProfileDto::from(user),
    }))
}

///
/// Logout a user
///
#[utoipa::path(
    post,
    tag = "Authentication",
    path = "/logout",    
    responses(
        (status = 200, description = "Successfully logged out the current user ")
    )
)]
#[post("/logout")]
async fn logout_handler(_: jwt_auth::JwtMiddleware) -> Result<HttpResponse, Error> {
    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(ActixWebDuration::new(-1, 0))
        .http_only(true)
        .finish();

    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({"status": "success"})))
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/auth")
        // .service(register_user_handler)
        .service(login_user_handler)
        .service(logout_handler);
    conf.service(scope);
}
