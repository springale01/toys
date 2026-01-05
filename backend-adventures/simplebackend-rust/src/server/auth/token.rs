use std::time::{Duration, SystemTime, UNIX_EPOCH};

use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{db::models::User, server::auth::roles::Roles};
#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub role: Roles,
    pub exp: usize,
}

pub fn create_jwt_token(user: &User) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = std::env::var("JWT_SECRET").expect("JWT SECRET not set!");

    let expiration = SystemTime::now()
        .checked_add(Duration::from_secs(60 * 60 * 24))
        .unwrap()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;

    let claims = Claims {
        sub: user.id,
        role: user.role.clone(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}
