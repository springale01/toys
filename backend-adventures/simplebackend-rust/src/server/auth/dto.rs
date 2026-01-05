use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::server::users::dto::UserDTO;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct LoginDTO {
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserDTO,
}
