pub mod auth;
pub mod dashboard;
pub mod legal;
pub mod public;
pub mod services;

use axum::Router;
use tower_http::services::ServeDir;

use crate::state::AppState;

pub fn app_routes(state: AppState) -> Router {
    Router::new()
        .merge(auth::auth_routes())
        .merge(public::public_routes())
        .merge(legal::legal_routes())
        .merge(services::service_routes())
        .merge(dashboard::dashboard_routes())
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state)
}
