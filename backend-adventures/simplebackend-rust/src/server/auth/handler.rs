use axum::{Json, extract::State, http::StatusCode};

use crate::{
    db::queries::{create_user_query, query_user_by_email, query_user_by_username},
    server::{
        auth::{
            dto::{AuthResponse, LoginDTO},
            roles::Roles,
            token::create_jwt_token,
        },
        state::AppState,
        users::dto::{CreateUserDto, UserDTO},
    },
    utils::password::{hash_password, verify_hashed_password},
};

pub async fn handle_login(
    State(state): State<AppState>,
    Json(payload): Json<LoginDTO>,
) -> Result<Json<AuthResponse>, StatusCode> {
    let user = if let Some(email) = &payload.email {
        query_user_by_email(&state.db, email)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    } else if let Some(username) = &payload.username {
        query_user_by_username(&state.db, username)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    } else {
        return Err(StatusCode::BAD_REQUEST);
    };

    let user = user.ok_or(StatusCode::NOT_FOUND)?;

    let is_valid = verify_hashed_password(&user.password_hash, &payload.password);

    if !is_valid {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = create_jwt_token(&user).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AuthResponse {
        token,
        user: UserDTO::from(user),
    }))
}

pub async fn handle_register(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserDto>,
) -> Result<Json<UserDTO>, StatusCode> {
    if payload.password.len() < 8 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let password_hash =
        hash_password(&payload.password).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = create_user_query(
        &state.db,
        Roles::User,
        &payload.email,
        &payload.username,
        &password_hash,
    )
    .await
    .map_err(|_| StatusCode::CONFLICT)?;

    Ok(Json(UserDTO::from(user)))
}
