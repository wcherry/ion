mod dto;
mod jwt_auth;
mod service;

use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    get, post, web, HttpResponse, Error,};

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{prelude::*, Duration};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;

use dto::RegisterUserSchema;

use crate::AppState;
//use crate::shared::schema::User;
use crate::shared::common::ServiceError;
use crate::auth::dto::{LoginRequestDto, TokenClaims};
use service::{find_user, is_exists, create_user};

// fn filter_user_record(user: &User) -> FilteredUser {
//     FilteredUser {
//         id: user.id.to_string(),
//         email: user.email_address.to_owned(),
//         name: user.name.to_owned(),
//         role: user.role.to_owned(),
//         createdAt: user.created_at,
//         updatedAt: user.updated_at,
//     }
// }

#[post("/register")]
async fn register_user_handler(
    body: web::Json<RegisterUserSchema>,
    app: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .expect("Error while hashing password")
        .to_string();
    let mut conn = app.pool.get()
    .map_err(|err| ServiceError::NotFound(err.to_string()))?;

    let exists = is_exists(&mut conn, body.name.to_owned())
    .map_err(|err| ServiceError::NotFound(err.to_string()))?;
    
    if exists {
        return Ok(HttpResponse::Conflict().json(
            serde_json::json!({"status": "fail","message": "User with that email already exists"}),
        ));
    }

    let user = web::block(move || {
        create_user(&mut conn, body.name.to_owned(), body.email.to_owned(), hashed_password)
    }).await?
    .map_err(|err| ServiceError::NotFound(err.to_string()))?;
    
    Ok(HttpResponse::Ok().json(user))
}

#[post("/login")]
async fn login_user_handler(
    app: web::Data<AppState>,
    web::Json(body): web::Json<LoginRequestDto>,
) -> Result<HttpResponse, Error> {
    let secret = app.config.jwt_secret.clone();
    let user = web::block(move || {
        let mut conn = app.pool.get()?;
        find_user(&mut conn, body.username)
    })
    .await?
    .map_err(|err| ServiceError::InternalServerError(err.to_string()))?;


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

    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({"status": "success", "token": token})))
}

#[get("/logout")]
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

// #[get("/users/me")]
// async fn get_me_handler(
//     req: HttpRequest,
//     data: web::Data<AppState>,
//     _: jwt_auth::JwtMiddleware,
// ) -> impl Responder {
//     let ext = req.extensions();
//     let user_id = ext.get::<uuid::Uuid>().unwrap();

//     let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
//         .fetch_one(&data.db)
//         .await
//         .unwrap();

//     let json_response = serde_json::json!({
//         "status":  "success",
//         "data": serde_json::json!({
//             "user": filter_user_record(&user)
//         })
//     });

//     HttpResponse::Ok().json(json_response)
// }

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/auth")
        .service(register_user_handler)
        .service(login_user_handler)
        .service(logout_handler)
        ;
    conf.service(scope);
}
