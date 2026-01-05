use serde::{Deserialize, Serialize};
use sqlx::Type;

//just going to be these two for now
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Type)]
#[sqlx(type_name = "text")]
#[sqlx(rename_all = "lowercase")]
pub enum Roles {
    Admin,
    User,
}

impl std::fmt::Display for Roles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let role = match self {
            Self::Admin => "admin",
            Self::User => "user",
        };

        write!(f, "{}", role)
    }
}

impl From<String> for Roles {
    fn from(value: String) -> Self {
        match value.as_str() {
            "user" => Self::User,
            "admin" => Self::Admin,
            _ => Self::User,
        }
    }
}
