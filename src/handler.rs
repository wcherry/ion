use crate::{db, DBPool};
use warp::{http::StatusCode, reject, reply::json, Reply, Rejection};
use crate::error::Error::DBQueryError;
use crate::services::{get_page, create_page};
use crate::data::PageRequest;

pub async fn health_handler(db_pool: DBPool) -> std::result::Result<impl Reply, Rejection> {
    let db = db::get_db_con(&db_pool)
            .await
            .map_err(|e| reject::custom(e))?;

    db.execute("SELECT 1", &[])
            .await
            .map_err(|e| reject::custom(DBQueryError(e)))?;
    Ok(StatusCode::OK)
}


pub async fn get_page_handler(page_id : i32, db_pool: DBPool) -> std::result::Result<impl Reply, Rejection> {
    let result = get_page(page_id, db_pool).await.map_err(|e| reject::custom(e))?;
    
    Ok(json(&result))
}

pub async fn create_page_handler(body: PageRequest, db_pool: DBPool) -> std::result::Result<impl Reply, Rejection> {
    let result = create_page(body, db_pool).await.map_err(|e| reject::custom(e))?;
    
    Ok(json(&result))
}

pub async fn get_page_content_handler(page_id : i32, db_pool: DBPool) -> std::result::Result<impl Reply, Rejection> {
    let result = get_page_content(page_id, db_pool).await.map_err(|e| reject::custom(e))?;
    
    Ok(json(&result))
}

pub async fn create_page_content_handler(body: PageRequest, db_pool: DBPool) -> std::result::Result<impl Reply, Rejection> {
    let result = create_page_content(body, db_pool).await.map_err(|e| reject::custom(e))?;
    
    Ok(json(&result))
}
