use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyDto {
    pub id: i32,
    pub name: String,
    pub active: bool,
}

impl CompanyDto {
    pub fn from(co : super::schema::Company) -> Self {
        CompanyDto { id: co.id, name: co.name, active: co.active }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDto {
    pub id: i32,
    pub name: String,
    pub active: bool,
    pub companies: Vec<CompanyDto>,
}

