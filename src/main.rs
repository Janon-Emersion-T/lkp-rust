mod auth;
mod db;
mod handlers;
mod models;
mod routes;
mod seeds;
mod services;
mod state;

use routes::app_routes;
use services::newsletter::start_newsletter_worker;
use state::AppState;
use std::env;
use tower_sessions::{MemoryStore, SessionManagerLayer};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let app_env = env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());
    let is_production = app_env.eq_ignore_ascii_case("production");
    let bind_addr = env::var("APP_BIND").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(3000);
    let run_migrations = env_flag("RUN_MIGRATIONS", true);
    let run_seeders = env_flag("RUN_SEEDERS", true);
    let enable_newsletter_worker = env_flag("ENABLE_NEWSLETTER_WORKER", true);
    let secure_cookies = env_flag("SESSION_COOKIE_SECURE", is_production);

    // Database connection
    let db = db::connect_db().await;

    if run_migrations {
        sqlx::migrate!("./migrations")
            .run(&db)
            .await
            .expect("Failed to run migrations");
    }

    if run_seeders {
        seeds::users::seed_default_user(&db)
            .await
            .expect("Failed to seed default user");

        seeds::industries::seed_default_industries(&db)
            .await
            .expect("Failed to seed default industries");

        if let Err(error) = seeds::careers::seed_default_careers(&db).await {
            eprintln!("Failed to seed default careers: {error}");
        }

        if let Err(error) = seeds::insights::seed_default_insights(&db).await {
            eprintln!("Failed to seed default insights: {error}");
        }

        if let Err(error) =
            seeds::newsletter_subscribers::seed_default_newsletter_subscribers(&db).await
        {
            eprintln!("Failed to seed default newsletter subscribers: {error}");
        }

        if let Err(error) = seeds::service_areas::seed_default_service_areas(&db).await {
            eprintln!("Failed to seed default service areas: {error}");
        }
    }

    if enable_newsletter_worker {
        start_newsletter_worker(db.clone()).await;
    }

    // Shared app state
    let state = AppState { db };

    // Session store
    let session_store = MemoryStore::default();

    let session_layer = SessionManagerLayer::new(session_store).with_secure(secure_cookies);

    // App
    let app = app_routes(state)
        .layer(axum::extract::DefaultBodyLimit::max(1024 * 1024))
        .layer(axum::middleware::map_response(set_security_headers))
        .layer(session_layer);

    // Server
    let listener = tokio::net::TcpListener::bind(format!("{bind_addr}:{port}"))
        .await
        .expect("Failed to bind server address");

    println!("Running on http://{bind_addr}:{port}");

    axum::serve(listener, app).await.expect("Server failed");
}

fn env_flag(name: &str, default: bool) -> bool {
    env::var(name)
        .ok()
        .map(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes"
            )
        })
        .unwrap_or(default)
}

async fn set_security_headers(mut response: axum::response::Response) -> axum::response::Response {
    use axum::http::header::{HeaderName, HeaderValue};

    response.headers_mut().insert(
        HeaderName::from_static("x-content-type-options"),
        HeaderValue::from_static("nosniff"),
    );
    response.headers_mut().insert(
        HeaderName::from_static("x-frame-options"),
        HeaderValue::from_static("DENY"),
    );
    response.headers_mut().insert(
        HeaderName::from_static("referrer-policy"),
        HeaderValue::from_static("strict-origin-when-cross-origin"),
    );
    response.headers_mut().insert(
        HeaderName::from_static("permissions-policy"),
        HeaderValue::from_static("camera=(), microphone=(), geolocation=()"),
    );

    response
}
