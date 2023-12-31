use utoipa::OpenApi;

use crate::pages;
use crate::auth;
use crate::shared;
use crate::blocks;

#[derive(OpenApi)]
#[openapi(
    paths(
        auth::register_user_handler, 
        auth::login_user_handler,
        auth::logout_handler,
        pages::create_page_handler, 
        pages::get_pages_handler,
        pages::create_page_permission_handler, 
        blocks::get_blocks_by_version_handler,
        blocks::get_blocks_for_page_handler,
        blocks::create_block_handler,
        blocks::update_block_handler,
    ),
    components(
        schemas(pages::dto::PageCreateDto, 
            pages::dto::PageDto, 
            pages::dto::PagePermissionCreateDto, 
            shared::dto::UserDto, 
            auth::dto::RegisterUserDto, 
            auth::dto::LoginRequestDto, 
            auth::dto::LoginResponseDto,
            shared::schema::UserProfile,
            blocks::dto::BlockDto, 
            blocks::dto::BlockRequest, 
        ),
    ),
    security(
        (),
        ("my_auth" = ["read:items", "edit:items"]),
        ("token_jwt" = [])
    ),
    tags(
        (name = "ion::api", description = "Ion API"),
    ),
    external_docs(url = "http://more.about.our.apis", description = "More about our APIs")
)]
pub struct ApiDoc; 
