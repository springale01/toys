use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    db::queries::{
        create_user_query, delete_user_query, get_user_single_query, get_users_query,
        patch_user_query, update_password_query,
    },
    server::{
        state::AppState,
        users::dto::{CreateUserDto, UpdatePasswordUserDTO, UpdateUserDto, UserDTO},
    },
    utils::password::{hash_password, verify_hashed_password},
};

pub async fn get_all_users_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<UserDTO>>, StatusCode> {
    let users = get_users_query(&state.db)
        .await
        .inspect(|u| tracing::info!("fetched {} users", u.len()))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user_dtoed = users
        .into_iter()
        .map(|u| UserDTO::from(u))
        .collect::<Vec<UserDTO>>();

    Ok(Json(user_dtoed))
}

pub async fn get_user_handler(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<UserDTO>, StatusCode> {
    match get_user_single_query(&state.db, user_id).await {
        Ok(Some(user)) => Ok(Json(UserDTO::from(user))),
        Ok(None) => {
            return Err(StatusCode::NOT_FOUND);
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn create_user_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserDto>,
) -> Result<Json<UserDTO>, StatusCode> {
    let hashed_password =
        hash_password(&payload.password).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let created_user = create_user_query(
        &state.db,
        crate::server::auth::roles::Roles::User,
        &payload.email,
        &hashed_password,
        &payload.username,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(UserDTO::from(created_user)))
}

pub async fn delete_user_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<UserDTO>, StatusCode> {
    match delete_user_query(&state.db, id).await {
        Ok(Some(user)) => {
            tracing::info!("Deleted user {} (email: {})", user.username, user.email);
            Ok(Json(UserDTO::from(user)))
        }
        Ok(None) => {
            tracing::info!("No user found with id {}", id);
            Err(StatusCode::NOT_FOUND)
        }
        Err(err) => {
            tracing::error!("Failed to delete user {}: {:?}", id, err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn patch_user_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserDto>,
) -> Result<Json<UserDTO>, StatusCode> {
    if payload.email.is_none() && payload.username.is_none() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let user = patch_user_query(
        &state.db,
        id,
        payload.email.as_deref(),
        payload.username.as_deref(),
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match user {
        Some(u) => {
            tracing::info!("Sucessfully changed the user info with id: {}", id);
            Ok(Json(UserDTO::from(u)))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn update_password_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdatePasswordUserDTO>,
) -> Result<Json<UserDTO>, StatusCode> {
    let user_in_question = get_user_single_query(&state.db, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    if !verify_hashed_password(&user_in_question.password_hash, &payload.old_password) {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let new_hash =
        hash_password(&payload.new_password).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let updated_user = update_password_query(&state.db, id, &new_hash)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    tracing::info!("updated password for user id: {}", updated_user.id);

    Ok(Json(UserDTO::from(updated_user)))
}
