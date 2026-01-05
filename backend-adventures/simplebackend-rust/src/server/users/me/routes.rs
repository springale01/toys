use axum::{Router, middleware, routing::get};

use crate::server::{auth, state::AppState, users::me::handler::handler_whoisme};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/whohtis", get(handler_whoisme))
        .layer(middleware::from_fn(auth::middleware::require_auth))
}
