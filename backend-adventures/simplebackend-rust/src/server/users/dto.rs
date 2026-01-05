use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::{db::models::User, server::auth::roles::Roles};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserDTO {
    pub email: String,
    pub role: Roles,
    pub username: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<User> for UserDTO {
    fn from(value: User) -> Self {
        Self {
            email: value.email,
            role: value.role,
            username: value.username,
            created_at: value.created_at,
        }
    }
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CreateUserDto {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UpdateUserDto {
    pub email: Option<String>,
    pub username: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UpdatePasswordUserDTO {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct MeDTO {
    pub id: Uuid,
    pub role: Roles,
    pub email: String,
    pub username: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<User> for MeDTO {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            role: Roles::User,
            email: value.email,
            username: value.username,
            created_at: value.created_at,
        }
    }
}
