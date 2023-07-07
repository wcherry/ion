use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::schema::User;

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