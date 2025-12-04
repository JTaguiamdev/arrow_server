use serde::{Deserialize, Serialize};
use crate::api::controllers::dto::role_dto::RoleDTO;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserDTO {
    pub username: String,
    pub role: RoleDTO,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewUserDTO {
    pub username: String,
    pub password: String,
}
