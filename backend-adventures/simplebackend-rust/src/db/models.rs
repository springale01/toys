use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::server::auth::roles::Roles;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub role: Roles,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Posts {
    pub id: Uuid,
    pub user_id: i32,
    pub title: String,
    pub content: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}
