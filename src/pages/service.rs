use crate::shared::common::DbError;
use diesel::{prelude::*, sql_query, sql_types, PgConnection};
use super::dto::{PageCreateDto, PagePermissionCreateDto, PageDto};

const DEFAULT_BLOCK_TYPE: &str = "paragraph";

pub(super) fn find_page(conn: &mut PgConnection, page_id: String, user_id: i32) -> Result<PageDto, DbError> {
    let page: PageDto = sql_query("select 
        p.id::text id, 
        p.name,
        p.owner_id,
        p.company_id,
        p.team_id,
        p.parent_page_id::text,
        p.created_at,
        p.updated_at,
        p.active,
        v.version,
        v.id::text page_version_id
    from pages p 
        join page_versions v on p.id = v.page_id 
    where p.id = uuid($1)
    AND (p.owner_id = $2 OR p.id IN 
        (SELECT page_id FROM page_permission p JOIN users u ON p.team_id = u.team_id OR p.company_id = u.company_id OR  p.allow_all = TRUE WHERE user_id = $2 OR p.allow_all = true))
    order by v.version desc limit 1;
    ")
    .bind::<sql_types::VarChar, _>(page_id)
    .bind::<sql_types::Integer, _>(user_id)
    .get_result::<PageDto>(conn)?;

    Ok(page)
}

pub(crate) fn create_page(conn: &mut PgConnection, page: PageCreateDto, user_id: i32) -> Result<PageDto, DbError> {
    let page_id = page.page_id.unwrap_or(uuid::Uuid::new_v4());
    let page_version_id = uuid::Uuid::new_v4();
    let block_id = uuid::Uuid::new_v4();
    let parent_page_id = uuid::Uuid::parse_str(&page.parent_page_id)?;

    let _id = diesel::sql_query( "INSERT INTO pages (id, parent_page_id, name, owner_id, company_id, team_id) values($1,$2,$3,$4,$5,$6)")
        .bind::<sql_types::Uuid, _>(page_id)
        .bind::<sql_types::Uuid, _>(parent_page_id)
        .bind::<sql_types::VarChar, _>(page.name)
        .bind::<sql_types::Integer, _>(user_id)
        .bind::<sql_types::Integer, _>(0)
        .bind::<sql_types::Integer, _>(0)
        .execute(conn)?;
    
    let _id = diesel::sql_query( "INSERT INTO page_versions (id, page_id, version) VALUES ($1,$2,$3)")
        .bind::<sql_types::Uuid, _>(page_version_id)
        .bind::<sql_types::Uuid, _>(page_id)
        .bind::<sql_types::Integer, _>(1)
        .execute(conn)?;
    
    let _id = diesel::sql_query( "INSERT INTO blocks (id, block_id, version, block_type, content,created_by,updated_by) values($1,uuid_generate_v4(),$2,$3,$4,$5,$6)")
        .bind::<sql_types::Uuid, _>(block_id)
        .bind::<sql_types::Integer, _>(1)
        .bind::<sql_types::VarChar, _>(DEFAULT_BLOCK_TYPE)
        .bind::<sql_types::VarChar, _>(page.content.unwrap_or("".to_string()))
        .bind::<sql_types::Integer, _>(1)
        .bind::<sql_types::Integer, _>(1)
        .execute(conn)?;
    
    let _id = diesel::sql_query( "INSERT INTO page_block_index (id, page_version_id, display_order, block_id) values(uuid_generate_v4(),$1,$2,$3)")
        .bind::<sql_types::Uuid, _>(page_version_id)
        .bind::<sql_types::Integer, _>(1)
        .bind::<sql_types::Uuid, _>(block_id)
        .execute(conn)?;
    
    let page = find_page(conn, page_id.to_string(), user_id)?;
    Ok(page)
}

pub(super) fn create_page_permission(conn: &mut PgConnection, perm: PagePermissionCreateDto, user_id: i32) -> Result<usize, DbError> {
    let page_id = uuid::Uuid::parse_str(&perm.page_id)?;

    let _id = diesel::sql_query( "INSERT INTO page_permission (page_id, user_id, team_id, company_id, allow_all, active, mode, created_by, updated_by) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)")
        .bind::<sql_types::Uuid, _>(page_id)
        .bind::<sql_types::Nullable<sql_types::Integer>, _>(perm.user_id)
        .bind::<sql_types::Nullable<sql_types::Integer>, _>(perm.team_id)
        .bind::<sql_types::Nullable<sql_types::Integer>, _>(perm.company_id)
        .bind::<sql_types::Bool, _>(perm.allow_all)
        .bind::<sql_types::Bool, _>(perm.active)
        .bind::<sql_types::VarChar, _>(perm.mode)
        .bind::<sql_types::Integer, _>(user_id)
        .bind::<sql_types::Integer, _>(user_id)
        .execute(conn)?;
    Ok(_id)
}