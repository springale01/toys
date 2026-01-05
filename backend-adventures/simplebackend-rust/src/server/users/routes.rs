use axum::{
    Router, middleware,
    routing::{get, patch},
};

use crate::server::{
    auth,
    state::AppState,
    users::handlers::{
        create_user_handler, delete_user_handler, get_all_users_handler, get_user_handler,
        patch_user_handler, update_password_handler,
    },
};

pub fn router() -> Router<AppState> {
    let admin_routes = Router::new()
        .route(
            "/",
            get(get_all_users_handler)
                .post(create_user_handler)
                .delete(delete_user_handler)
                .patch(patch_user_handler),
        )
        .route("/:id", get(get_user_handler))
        .route("/:id/updatepassword", patch(update_password_handler))
        .layer(middleware::from_fn(auth::middleware::require_auth))
        .layer(middleware::from_fn(auth::middleware::require_admin));

    admin_routes
}
