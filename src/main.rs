use mobc::{Connection, Pool};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use std::convert::Infallible;
use tokio_postgres::NoTls;
use warp::{Filter, Rejection};

use crate::data::PageRequest;

mod data;
mod db;
mod error;
mod handler;
mod services;

pub type Result<T> = std::result::Result<T, Rejection>;
type DBCon = Connection<PgConnectionManager<NoTls>>;
type DBPool = Pool<PgConnectionManager<NoTls>>;

// Add DBPool to warp filter
fn with_db(db_pool: DBPool) -> impl Filter<Extract = (DBPool,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}


#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // set up database connection pool
    let db_pool = db::create_pool().expect("database pool can be created");

    db::init_db(&db_pool)
        .await
        .expect("database can be initialized");

    let health_route = warp::path!("health")
        .and(with_db(db_pool.clone()))
        .and_then(handler::health_handler);

    let page = warp::path("/api/page");
    let page_routes = page
        .and(warp::get())
        .and(warp::query())
        .and(with_db(db_pool.clone()))
        .and_then(handler::get_page_handler)
        .or(page
            .and(warp::post())
            .and(warp::body::json())
            .and(with_db(db_pool.clone()))
            .and_then(handler::create_page_handler));
        // .or(page
        //     .and(warp::put())
        //     .and(warp::path::param())
        //     .and(warp::body::json())
        //     .and(with_db(db_pool.clone()))
        //     .and_then(handler::update_todo_handler))
        // .or(page
        //     .and(warp::delete())
        //     .and(warp::path::param())
        //     .and(with_db(db_pool.clone()))
        //     .and_then(handler::delete_todo_handler));

    let page_content = warp::path("/api/page/content");
    let page_conent_routes = page_content
        .and(warp::get())
        .and(warp::query())
        .and(with_db(db_pool.clone()))
        .and_then(handler::get_page_content_handler)
        .or(page
            .and(warp::post())
            .and(warp::body::json())
            .and(with_db(db_pool.clone()))
            .and_then(handler::create_page_contenthandler));



    let routes = health_route
        .or(page_routes)
        .with(warp::cors().allow_any_origin())
        .recover(error::handle_rejection);

    warp::serve(routes).run(([0,0,0,0], 8000)).await;
}

