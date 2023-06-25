#[macro_use]
extern crate diesel;

use actix_files::NamedFile;
use actix_web::{web, App, HttpRequest, HttpServer, Result, Responder, get};
use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};
use std::{io, path::PathBuf};

use crate::users::{get_all_roles, get_permissions, get_permissions_for_roles,get_permissions_for_user_and_company,get_role,get_user,get_users,};
use crate::pages::{get_pages,};
use crate::blocks::{get_blocks, update_block_handler as update_block, create_block};

mod blocks;
mod common;
mod pages;
mod users;

const CLIENT_PATH: &str = "./public/";

async fn index(req: HttpRequest) -> Result<NamedFile> {
    let mut filename: &str = req.match_info().query("filename"); //.parse().unwrap();
    let mut path: PathBuf = PathBuf::new();
    path.push(CLIENT_PATH);
    if filename.is_empty() {
        filename = "index.html";
    }
    path.push(filename);
    println!("{:?}", &path);
    Ok(NamedFile::open(path)?)
}

#[get("/healthcheck")]
async fn health_check(name: web::Path<String>) -> impl Responder {
    format!("WebServer Status: {}\nDatabase Status {}\n", "Ok", "Ok")
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    // set up database connection pool
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    println!("{}", conn_spec);
    let manager = ConnectionManager::<PgConnection>::new(conn_spec);
    // Create connection pool
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    println!("Starting server at: http://localhost:8080");

    // Start HTTP server
    HttpServer::new( move|| {
        App::new()
            .service(
                web::scope("/api")
                    .service(get_user)
                    .service(get_users)
                    .service(get_permissions)
                    .service(get_permissions_for_user_and_company)
                    .service(get_role)
                    .service(get_all_roles)
                    .service(get_permissions_for_roles)
                    .service(get_pages)
                    .service(get_blocks)
                    .service(create_block)
                    .service(update_block)
                    .app_data(web::Data::new(pool.clone())),
            )
            .service(health_check)
            .route("/{filename:.*}", web::get().to(index))
    })
    .bind(("127.0.0.1", 8090))?
    .run()
    .await
}
