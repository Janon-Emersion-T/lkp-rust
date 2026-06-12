use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use tower_sessions::Session;

pub async fn require_auth(session: Session, request: Request, next: Next) -> Response {
    if let Ok(Some(_user_id)) = session.get::<String>("user_id").await {
        return next.run(request).await;
    }

    Redirect::to("/login").into_response()
}
