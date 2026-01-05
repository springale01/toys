use axum::Router;

use crate::server::{state::AppState, users::me};

pub mod auth;
pub mod state;
pub mod users;
// note to self use what the function does in general first before where it came from...
pub fn router(app_state: AppState) -> Router<()> {
    Router::new()
        .nest("/api", users::routes::router())
        .nest("/me", me::routes::router())
        .nest("/auth", auth::routes::router())
        .with_state(app_state)
}
