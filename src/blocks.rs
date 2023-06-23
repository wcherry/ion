use crate::{common::{DbError, DbPool, ServiceError}};
use actix_web::{get, post, web, Error, HttpResponse};
use diesel::{prelude::*, sql_query, sql_types::*, PgConnection};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

table! {
    blocks (id) {
        id -> Uuid,
        block_id -> Uuid,
        version -> Integer,
        block_type -> VarChar,
        content -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        created_by -> Integer,
        updated_by -> Integer,
        active -> Bool,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, QueryableByName, PartialEq,)]
#[diesel(table_name = blocks)]
pub struct Block {
    id: uuid::Uuid,
    block_id: uuid::Uuid,
    version: i32,
    block_type: String,
    content: Option<String>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    created_by: i32,
    updated_by: i32,
    active: bool,
}

table! {
    page_block_index (id) {
        id -> Uuid,
        page_version_id -> Uuid,
        block_id -> Uuid,
        display_order -> Integer,
        created_at -> Timestamp,
    }
}

 #[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, QueryableByName, PartialEq,)]
 #[diesel(table_name = page_block_index)] 
pub struct PageBlockIndex {
    pub id: uuid::Uuid,
    pub page_version_id: uuid::Uuid,
    pub block_id: uuid::Uuid,
    pub display_order: i32,
    pub created_at: NaiveDateTime,
}

// TODO: This is not a real table, but a view.  Is there a way to make this work better?
table! {
    blocks_dtos (id) {
        id -> VarChar,
        block_id -> VarChar,
        version -> Integer,
        display_order -> Integer,
        block_type -> VarChar,
        content -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        created_by -> Integer,
        updated_by -> Integer,
        active -> Bool,
    }
}
  
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, QueryableByName, PartialEq,)]
#[diesel(table_name = blocks_dtos)]
pub struct BlockDto {
    id: String, 
    block_id: String,
    version: i32,
    display_order: i32,
    block_type: String,
    content: Option<String>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    created_by: i32,
    updated_by: i32,
    active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockRequest {
    pub block_id: Option<String>,
    pub version: Option<i32>,
    pub block_type: String,
    pub content: Option<String>,
    pub display_order: i32,
}


fn find_blocks(conn: &mut PgConnection, page_version_id: String) -> Result<Vec<BlockDto>, DbError> {
    let blocks: Vec<BlockDto> = sql_query("select 
	b.id::text,
	b.block_id::text,
	b.version,
	i.display_order,
	b.block_type,
	b.content,
	b.created_at,
	b.updated_at,
	b.created_by,
	b.updated_by,
	b.active
from blocks b
join page_block_index i on b.id = i.block_id
where i.page_version_id = uuid($1)
order by i.display_order asc
")
        .bind::<VarChar, _>(page_version_id)
        .get_results::<BlockDto>(conn)?;

    Ok(blocks)
}

fn create_block(conn: &mut PgConnection, page_version_id: String, block_req: BlockRequest) -> Result<BlockDto, DbError> {
    let page_version_id = uuid::Uuid::parse_str(&page_version_id)?;
    
    let block_id = uuid::Uuid::new_v4();
    let block = Block {
        id: block_id.clone(),
        block_id: if let Some(v) = block_req.block_id { uuid::Uuid::parse_str(&v)? } else { block_id.clone() },
        version: if let Some(v) = block_req.version { v } else { 1 },
        block_type: block_req.block_type,
        content: block_req.content,
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
        created_by: 1,
        updated_by: 1,
        active: true,
    };

    shift_blocks(conn, page_version_id, block_req.display_order)?;

    let block = diesel::insert_into(blocks::table)
        .values(block)
        .get_result::<Block>(conn)?;

    let page_block_index = PageBlockIndex {
        id: uuid::Uuid::new_v4(),
        page_version_id: page_version_id,
        block_id: block_id,
        display_order: block_req.display_order,
        created_at: chrono::Utc::now().naive_utc(),
    };

    let page_block_index = diesel::insert_into(page_block_index::table)
        .values(page_block_index)
        .get_result::<PageBlockIndex>(conn)?;

    let block_dto = BlockDto {
        id: block.id.to_string(),
        block_id: block.block_id.to_string(),
        version: block.version,
        display_order: page_block_index.display_order,
        block_type: block.block_type,
        content: block.content,
        created_at: block.created_at,
        updated_at: block.updated_at,
        created_by: block.created_by,
        updated_by: block.updated_by,
        active: block.active,
    };

    Ok(block_dto)
}

fn shift_blocks(conn: &mut PgConnection, page_version_id: uuid::Uuid, display_order: i32) -> Result<(), DbError> {
    diesel::sql_query("update page_block_index set display_order = display_order + 1 where page_version_id = uuid($1) and display_order >= $2") 
            .bind::<Uuid, _>(page_version_id)
            .bind::<Integer, _>(display_order) 
            .execute(conn)?;
    Ok(())
}

#[get("/page-version/{page_version_id}/blocks")]
pub async fn get_blocks_handler(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let page_version_id = path.into_inner();
    let blocks = web::block(move || {
        let mut conn = pool.get()?;
        find_blocks(&mut conn, page_version_id)
    })
    .await?
    .map_err(|err| ServiceError::NotFound(err.to_string()))?;

    Ok(HttpResponse::Ok().json(blocks))
}


#[post("/page-version/{page_version_id}/block")]
pub async fn create_block_handler(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
    web::Json(body): web::Json<BlockRequest>,
) -> Result<HttpResponse, Error> {
    let page_version_id: String = path.into_inner();
    let blocks = web::block(move || {
        let mut conn = pool.get()?;
        create_block(&mut conn, page_version_id, body)
    })
    .await?
    .map_err(|err| ServiceError::BadRequest(err.to_string()))?;

    Ok(HttpResponse::Ok().json(blocks))
}


