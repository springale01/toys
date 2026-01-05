use crate::{db::models::User, server::auth::roles::Roles};
use sqlx::{PgPool, query_as};
use uuid::Uuid;

pub async fn get_users_query(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    query_as!(
        User,
        r#"
        SELECT id, role, email, username, created_at, password_hash
        FROM users
        "#
    )
    .fetch_all(pool)
    .await
}

pub async fn get_user_single_query(pool: &PgPool, id: Uuid) -> Result<Option<User>, sqlx::Error> {
    query_as!(
        User,
        r#"
        SELECT id, role, email, username, password_hash, created_at
        FROM users
        WHERE id = ($1)
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}

pub async fn create_user_query(
    pool: &PgPool,
    role: Roles,
    email: &str,
    password_hash: &str,
    username: &str,
) -> Result<User, sqlx::Error> {
    query_as!(
        User,
        r#"
        INSERT INTO users (id, role, email, username, password_hash)
        VALUES ($1, $2 ,$3, $4, $5)
        RETURNING id, role, email, username, password_hash, created_at
        "#,
        uuid::Uuid::new_v4(),
        role.to_string(),
        email,
        username,
        password_hash,
    )
    .fetch_one(pool)
    .await
}

pub async fn delete_user_query(pool: &PgPool, id: Uuid) -> Result<Option<User>, sqlx::Error> {
    query_as!(
        User,
        r#"
        DELETE FROM users
        WHERE id = ($1)
        RETURNING id, role, email, username, password_hash, created_at
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}

pub async fn patch_user_query(
    pool: &PgPool,
    id: Uuid,
    email: Option<&str>,
    username: Option<&str>,
) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
        UPDATE users
        SET
            email = COALESCE($2, email),
            username = COALESCE($3, username)
        WHERE id = $1
        RETURNING id, role, email, username, password_hash, created_at
        "#,
        id,
        email,
        username,
    )
    .fetch_optional(pool)
    .await
}

pub async fn update_password_query(
    pool: &PgPool,
    id: Uuid,
    password_hash: &str,
) -> Result<Option<User>, sqlx::Error> {
    query_as!(
        User,
        r#"
        UPDATE users
        SET password_hash = $2
        WHERE id = $1
        RETURNING id, role, email, username, password_hash, created_at
        "#,
        id,
        password_hash
    )
    .fetch_optional(pool)
    .await
}

pub async fn query_user_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
    query_as!(
        User,
        r#"
        SELECT id, role, email, username, password_hash, created_at
        FROM users
        WHERE email = $1
        "#,
        email
    )
    .fetch_optional(pool)
    .await
}

pub async fn query_user_by_username(
    pool: &PgPool,
    username: &str,
) -> Result<Option<User>, sqlx::Error> {
    query_as!(
        User,
        r#"
        SELECT id, role, email, username, password_hash, created_at
        FROM users
        WHERE username = $1
        "#,
        username
    )
    .fetch_optional(pool)
    .await
}

pub async fn query_promote_user(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
    query_as!(
        User,
        r#"
        UPDATE users
        SET role = $1
        WHERE email = $2
        RETURNING id, role, email, username, password_hash, created_at
        "#,
        "admin",
        email
    )
    .fetch_optional(pool)
    .await
}
