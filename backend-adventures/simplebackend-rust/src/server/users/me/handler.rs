use axum::{Extension, Json, extract::State, http::StatusCode};
use uuid::Uuid;

use crate::{
    db::queries::get_user_single_query,
    server::{state::AppState, users::dto::MeDTO},
};
//I think this is supposed to get a signed jwt tho
pub async fn handler_whoisme(
    State(state): State<AppState>,
    Extension(id): Extension<Uuid>,
) -> Result<Json<MeDTO>, StatusCode> {
    let me = get_user_single_query(&state.db, id).await;

    match me {
        Ok(Some(u)) => {
            tracing::info!("Found user with id, role: {}", u.id);
            return Ok(Json(MeDTO::from(u)));
        }
        Ok(None) => {
            return Err(StatusCode::NOT_FOUND);
        }
        Err(_) => {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
}
