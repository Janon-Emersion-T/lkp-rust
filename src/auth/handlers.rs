use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
use askama::Template;
use axum::{
    extract::{Form, State},
    response::{Html, IntoResponse, Redirect},
};
use tower_sessions::Session;

use crate::{
    auth::{forms::LoginForm, models::User},
    state::AppState,
};

#[derive(Template)]
#[template(path = "auth/login.html")]
pub struct LoginTemplate {
    pub error: Option<String>,
}

pub async fn show_login(session: Session) -> impl IntoResponse {
    if let Ok(Some(_user_id)) = session.get::<String>("user_id").await {
        return Redirect::to("/dashboard").into_response();
    }

    Html(LoginTemplate { error: None }.render().unwrap()).into_response()
}

pub async fn process_login(
    State(state): State<AppState>,
    session: Session,
    Form(form): Form<LoginForm>,
) -> impl IntoResponse {
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT id, name, email, password_hash, role, is_active
        FROM users
        WHERE email = $1
        LIMIT 1
        "#,
    )
    .bind(form.email.trim().to_lowercase())
    .fetch_optional(&state.db)
    .await;

    let Ok(Some(user)) = user else {
        return Html(LoginTemplate {
            error: Some("Invalid email or password.".to_string()),
        }
        .render()
        .unwrap())
        .into_response();
    };

    if !user.is_active {
        return Html(LoginTemplate {
            error: Some("Your account is inactive.".to_string()),
        }
        .render()
        .unwrap())
        .into_response();
    }

    let parsed_hash = PasswordHash::new(&user.password_hash);

    let password_ok = parsed_hash
        .ok()
        .and_then(|hash| {
            Argon2::default()
                .verify_password(form.password.as_bytes(), &hash)
                .ok()
        })
        .is_some();

    if !password_ok {
        return Html(LoginTemplate {
            error: Some("Invalid email or password.".to_string()),
        }
        .render()
        .unwrap())
        .into_response();
    }

    let _ = session.insert("user_id", user.id.to_string()).await;
    let _ = session.insert("user_name", user.name).await;
    let _ = session.insert("user_email", user.email).await;
    let _ = session.insert("user_role", user.role).await;

    Redirect::to("/dashboard").into_response()
}

pub async fn logout(session: Session) -> impl IntoResponse {
    let _ = session.clear().await;
    Redirect::to("/login")
}