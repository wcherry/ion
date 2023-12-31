use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyDto {
    pub id: i32,
    pub name: String,
    pub active: bool,
}
