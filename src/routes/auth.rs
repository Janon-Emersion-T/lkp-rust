use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    auth::handlers::{logout, process_login, show_login},
    state::AppState,
};

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/login", get(show_login).post(process_login))
        .route("/logout", post(logout))
}
