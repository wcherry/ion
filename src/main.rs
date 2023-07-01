mod auth;
mod blocks;
mod pages;
mod shared;
mod users;

#[macro_use]
extern crate diesel;

use actix_files::NamedFile;
use actix_web::{web, App, http::header, HttpRequest, HttpServer, Result, Responder, get};
use diesel::{
    r2d2::{self, Pool, ConnectionManager},
    PgConnection,
};
use std::{io, path::PathBuf};
use actix_web::middleware::Logger;

use pages::{get_pages,};
use blocks::{get_blocks, update_block_handler as update_block, create_block};
use shared::config::Config;

use actix_cors::Cors;

pub struct AppState {
    pool: Pool<ConnectionManager<PgConnection>> ,
    config: Config,
}

const CLIENT_PATH: &str = "./public/";

async fn index(req: HttpRequest) -> Result<NamedFile> {
    let mut filename: &str = req.match_info().query("filename");
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
async fn health_check(_name: web::Path<String>) -> impl Responder {
    format!("WebServer Status: {}\nDatabase Status {}\n", "Ok", "Ok")
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    let config = Config::init();
    
    // set up database connection pool
    let manager = ConnectionManager::<PgConnection>::new(&config.database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    println!("Starting server at: http://localhost:8090");

    // Start HTTP server
    HttpServer::new( move|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8090")
            .allowed_methods(vec!["GET", "PUT", "POST"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .app_data(web::Data::new(AppState {
                pool: pool.clone(),
                config: config.clone(),
            }))
            .wrap(cors)
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                .configure(auth::config)
                .configure(users::config)
                    .service(get_pages)
                    .service(get_blocks)
                    .service(create_block)
                    .service(update_block)
                    // .app_data(web::Data::new(pool.clone())),
            )
            .service(health_check)
            .route("/{filename:.*}", web::get().to(index))
    })
    .bind(("127.0.0.1", 8090))?
    .run()
    .await
}
