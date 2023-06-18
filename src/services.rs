use mobc_postgres::tokio_postgres::Row;

use crate::services::Error::DBQueryError;
use crate::error::Error;
use crate::{db, DBPool};
use crate::data::{Page, PageDetails, PageRequest};

pub async fn get_page(page_id : i32, db_pool: DBPool) -> std::result::Result<Page, Error> {
    let conn = db::get_db_con(&db_pool).await?;
    let query =  "SELECT id, name, created_at, deleted FROM pages WHERE id = $1";
    let result = conn.query_one(query, &[&page_id]).await?;
    let mut page = map_row_to_page(&result);

    page.versions.append(get_page_history(page_id, conn).await.unwrap().as_mut());
    return Ok(page);
}

async fn get_page_history(page_id : i32, conn: mobc::Connection<mobc_postgres::PgConnectionManager<mobc_postgres::tokio_postgres::NoTls>> ) -> std::result::Result<Vec<PageDetails>, Error> {
    let query =  "SELECT id, version, created_at, deleted FROM page_details WHERE page_id = $1";
    let results = conn.query(query, &[&page_id]).await.map_err(DBQueryError)?;
    // let versions = results.iter().map(|row| { map_row_to_page_details(row)}).collect();
    Ok(results.iter().map(|r| map_row_to_page_details(&r)).collect())
}

pub async fn create_page(request: PageRequest, db_pool: DBPool) -> std::result::Result<i32, Error> {
    let conn = db::get_db_con(&db_pool).await?;
    let results = conn.query_one("INSERT INTO pages (name) VALUES ($1) RETURNING id", &[&request.name])
        .await
        .map_err(DBQueryError)?;
    let id:i32 = results.get(0);
    let version = String::from("1");
    
    let query =  "INSERT INTO page_details (page_id, version, raw_content) VALUES ($1, $2, $3) RETURNING id, version, created_at, deleted";
    let results = conn.query(query, &[&id, &version, &request.raw_content]).await.map_err(DBQueryError)?;
    
    return Ok(id);
}



fn map_row_to_page(row: &Row) -> Page {
    let id: i32 = row.get(0);
    let name: String = row.get(1);
    let created_at: chrono::DateTime<chrono::Utc> = row.get(2);
    let deleted: bool = row.get(3);
    Page { id, name, created_at, deleted, versions: Vec::new()}
}

fn map_row_to_page_details(row: &Row) -> PageDetails {
    let id: i32 = row.get(0);
    let version: i32 = row.get(1);
    let created_at: chrono::DateTime<chrono::Utc> = row.get(2);
    let deleted: bool = row.get(3);
    PageDetails { id, version, created_at, deleted, raw_content: None} 
}

