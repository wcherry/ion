
mod auth;
mod blocks;
pub mod pages;
mod shared;
pub mod users;
mod api;
pub mod helper;
mod swagger;

#[macro_use]
extern crate diesel;
extern crate diesel_migrations;

use actix_files::NamedFile;
use actix_web::{get, http::header, web, App, HttpRequest, HttpServer, Responder, Result};
use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use actix_web::middleware::Logger;
use std::{io, path::PathBuf};

use shared::common::{AppState, Config, CountResult};

use actix_cors::Cors;

use diesel::{prelude::*, sql_query};

const CLIENT_PATH: &str = "./public/";

async fn index(req: HttpRequest, app: web::Data<AppState>) -> Result<NamedFile> {
    let mut filename: &str = req.match_info().query("filename");
    let mut path: PathBuf = PathBuf::new();
    path.push(CLIENT_PATH);
    if filename.is_empty() {
        filename = "index.html";
    }

    if filename == "index.html" && !app.is_init_completed() {
        filename = "init.html";
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

    let mut conn: r2d2::PooledConnection<ConnectionManager<PgConnection>> = pool.get().expect("Failed to get connection from pool");
    
    let result: CountResult = sql_query("SELECT COUNT(*) AS count FROM information_schema.tables WHERE table_name = 'users'")
        .get_result::<CountResult>(&mut conn)
        .expect("Failed to execute test query");
    
    let is_db_ready = result.count == 1;
    if !is_db_ready {
        println!("Database not initialized. Please run the migrations in production before starting the server.");
    }

    println!("Starting server at: http://localhost:8090");

    // Start HTTP server
    HttpServer::new(move || {
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
            .app_data(web::Data::new(AppState::new(
                pool.clone(),
                config.clone(),
                is_db_ready,
            )))
            .wrap(cors)
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .configure(api::config)
                    .configure(auth::config)
                    .configure(users::config)
                    .configure(blocks::config)
                    .configure(pages::config),
            )
            .service(health_check)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", swagger::ApiDoc::openapi()),
            )
            .route("/{filename:.*}", web::get().to(index))
    })
    .bind(("127.0.0.1", 8090))?
    .run()
    .await
}
