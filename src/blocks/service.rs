use super::dto::{BlockDto, BlockRequest};
use super::schema::{blocks, page_block_index, Block, PageBlockIndex};
use crate::shared::common::{Connection, DbError};
use diesel::{prelude::*, sql_query, sql_types::*, PgConnection};

pub fn get_block(conn: &mut PgConnection, id: String) -> Result<Block, DbError> {
    let block = sql_query(
        "select 
        b.id,
        b.block_id,
        b.version,
        b.block_type,
        b.content,
        b.created_at,
        b.updated_at,
        b.created_by,
        b.updated_by,
        b.active
    from blocks b
    where b.block_id = uuid($1) order by b.version desc limit 1",
    )
    .bind::<VarChar, _>(id)
    .get_result::<Block>(conn)?;

    Ok(block)
}

fn create_block(conn: &mut PgConnection, block_req: BlockRequest) -> Result<BlockDto, DbError> {
    let uuid = uuid::Uuid::new_v4();

    let block = Block {
        id: uuid,
        block_id: if let Some(v) = block_req.block_id {
            uuid::Uuid::parse_str(&v)?
        } else {
            uuid
        },
        version: if let Some(v) = block_req.version {
            v
        } else {
            1
        },
        block_type: block_req.block_type,
        content: block_req.content,
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
        created_by: 1,
        updated_by: 1,
        active: true,
    };

    let block = diesel::insert_into(blocks::table)
        .values(block)
        .get_result::<Block>(conn)?;

    let block_dto = BlockDto {
        id: block.id.to_string(),
        block_id: block.block_id.to_string(),
        version: block.version,
        block_type: block.block_type,
        display_order: None,
        content: block.content,
        created_at: block.created_at,
        updated_at: block.updated_at,
        created_by: block.created_by,
        updated_by: block.updated_by,
        active: block.active,
        modes: "owner".to_string(),
    };

    Ok(block_dto)
}

pub fn update_block(
    conn: &mut PgConnection,
    id: String,
    block_req: BlockRequest,
) -> Result<Block, DbError> {
    let uuid = uuid::Uuid::parse_str(&id)?;
    let block = get_block(conn, id.clone())?;
    let block = Block {
        id: uuid,
        block_id: if let Some(v) = block_req.block_id {
            uuid::Uuid::parse_str(&v)?
        } else {
            uuid::Uuid::new_v4()
        },
        version: block_req.version.unwrap() + 1,
        block_type: block_req.block_type,
        content: block_req.content,
        created_at: block.created_at,
        updated_at: chrono::Utc::now().naive_utc(),
        created_by: block.created_by,
        updated_by: block.updated_by, // Change this to the current user
        active: block.active,
    };

    diesel::update(blocks::table).set(block).execute(conn)?;
    get_block(conn, id)
}

pub fn create_block_and_attach_to_page(
    conn: &mut PgConnection,
    block_req: BlockRequest,
) -> Result<BlockDto, DbError> {
    let display_order = block_req.display_order;
    let page_version_id = block_req.page_version_id.clone();
    let mut block_dto = create_block(conn, block_req)?;
    let block_id = uuid::Uuid::parse_str(&block_dto.id)?;

    if let Some(page_version_id) = page_version_id {
        let page_version_id = uuid::Uuid::parse_str(&page_version_id)?;
        shift_blocks(conn, page_version_id, display_order)?;

        let page_block_index = PageBlockIndex {
            id: uuid::Uuid::new_v4(),
            page_version_id,
            block_id,
            display_order,
            created_at: chrono::Utc::now().naive_utc(),
        };

        let page_block_index = diesel::insert_into(page_block_index::table)
            .values(page_block_index)
            .get_result::<PageBlockIndex>(conn)?;

        block_dto.display_order = Some(page_block_index.display_order);
    }

    Ok(block_dto)
}

pub fn find_blocks_by_page_version(
    conn: &mut Connection,
    page_version_id: String,
    user_id: i32,
) -> Result<Vec<BlockDto>, DbError> {
    let blocks: Vec<BlockDto> = sql_query(
        "select 
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
    b.active,
    v.id,
    p.id page_id,
    CASE WHEN p.owner_id = $2 THEN 'owner' ELSE string_agg(DISTINCT r.MODE::text,',') END modes
from blocks b
join page_block_index i on b.id = i.block_id
JOIN page_versions v ON i.page_version_id = v.id
JOIN pages p ON v.page_id = p.id
LEFT JOIN page_permission r ON v.page_id = r.page_id
LEFT JOIN users u ON p.team_id = u.team_id OR p.company_id = u.company_id OR r.allow_all = TRUE
where i.page_version_id = uuid($1) AND (p.owner_id = $2 OR r.allow_all = TRUE OR r.user_id = $2)
GROUP BY b.id, i.display_order, v.id,p.owner_id,p.id 
order by i.display_order asc
    ",
    )
    .bind::<VarChar, _>(page_version_id)
    .bind::<Integer, _>(user_id)
    .get_results::<BlockDto>(conn)?;

    Ok(blocks)
}

pub fn find_blocks_by_page(
    conn: &mut Connection,
    page_id: String,
    user_id: i32,
) -> Result<Vec<BlockDto>, DbError> {
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
    b.active,
    v.id,
    p.id page_id,
    CASE WHEN p.owner_id = $2 THEN 'owner' ELSE string_agg(DISTINCT r.MODE::text,',') END modes
from blocks b
join page_block_index i on b.id = i.block_id
JOIN (SELECT v.* FROM page_versions v WHERE page_id = uuid($1) ORDER BY v.VERSION LIMIT 1) v ON i.page_version_id = v.id
JOIN pages p ON v.page_id = p.id
LEFT JOIN page_permission r ON v.page_id = r.page_id
LEFT JOIN users u ON p.team_id = u.team_id OR p.company_id = u.company_id OR r.allow_all = TRUE
where v.page_id = uuid($1) AND (p.owner_id = $2 OR r.allow_all = TRUE OR r.user_id = $2)
GROUP BY b.id, i.display_order, v.id,p.owner_id,p.id 
order by i.display_order asc
    ")
        .bind::<VarChar, _>(page_id)
        .bind::<Integer, _>(user_id)
        .get_results::<BlockDto>(conn)?;

    Ok(blocks)
}

fn shift_blocks(
    conn: &mut PgConnection,
    page_version_id: uuid::Uuid,
    display_order: i32,
) -> Result<(), DbError> {
    diesel::sql_query("update page_block_index set display_order = display_order + 1 where page_version_id = uuid($1) and display_order >= $2") 
            .bind::<Uuid, _>(page_version_id)
            .bind::<Integer, _>(display_order) 
            .execute(conn)?;
    Ok(())
}
