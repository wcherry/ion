use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::schema::{User, UserProfile};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserDto {
    pub id: i32,
    pub name: String,
    pub active: bool,
}

impl std::convert::From<User> for UserDto {
    fn from(user: User) -> Self {
        UserDto {
            id: user.id,
            name: user.name,
            active: user.active,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserProfileDto {
    pub id: i32,
    pub name: String,
    #[serde(rename = "emailAddress")]
    pub email_address: String,
    pub role: String,
    #[serde(rename = "profileId")]
    pub profile_id: Option<i32>,
    #[serde(rename = "avatarUrl")]
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    #[serde(rename = "defaultPageId")]
    pub default_page_id: Option<String>,
    #[serde(rename = "pageVersionId")]
    pub page_version_id: Option<String>,
    #[serde(rename = "companyId")]
    pub company_id: Option<i32>,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::NaiveDateTime,
    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::NaiveDateTime,
    #[serde(rename = "createdBy")]
    pub created_by: i32,
    #[serde(rename = "updatedBy")]
    pub updated_by: i32,
    pub active: bool,
}

impl std::convert::From<UserProfile> for UserProfileDto {
    fn from(user: UserProfile) -> Self {
        UserProfileDto {
            id: user.id,
            name: user.name,
            email_address: user.email_address,
            role: user.role,
            profile_id: user.profile_id,
            avatar_url: user.avatar_url,
            bio: user.bio,
            default_page_id: user.default_page_id,
            page_version_id: user.page_version_id,
            company_id: user.company_id,
            created_at: user.created_at,
            updated_at: user.updated_at,
            created_by: user.created_by,
            updated_by: user.updated_by,
            active: user.active,
        }
    }
}
