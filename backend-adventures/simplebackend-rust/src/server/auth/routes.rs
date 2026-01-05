use axum::{Router, routing::post};

use crate::server::{
    auth::handler::{handle_login, handle_register},
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", post(handle_login))
        .route("/register", post(handle_register))
}
