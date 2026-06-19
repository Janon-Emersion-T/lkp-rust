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
use tower_http::compression::CompressionLayer;
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

    println!(
        "Startup config: APP_ENV={app_env}, RUN_MIGRATIONS={run_migrations}, RUN_SEEDERS={run_seeders}, ENABLE_NEWSLETTER_WORKER={enable_newsletter_worker}, SESSION_COOKIE_SECURE={secure_cookies}"
    );

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

        if let Err(error) = seeds::portfolios::seed_default_portfolios(&db).await {
            eprintln!("Failed to seed default portfolios: {error}");
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
        .layer(CompressionLayer::new())
        .layer(axum::middleware::from_fn(set_response_headers))
        .layer(session_layer);

    // Server
    let listener = tokio::net::TcpListener::bind(format!("{bind_addr}:{port}"))
        .await
        .expect("Failed to bind server address");

    println!("Running on http://{bind_addr}:{port}");

    axum::serve(listener, app).await.expect("Server failed");
}

fn env_flag(name: &str, default: bool) -> bool {
    match env::var(name) {
        Ok(value) => {
            let normalized = value.trim().to_ascii_lowercase();

            match normalized.as_str() {
                "1" | "true" | "yes" | "on" | "enabled" => true,
                "0" | "false" | "no" | "off" | "disabled" => false,
                _ => {
                    eprintln!(
                        "Invalid boolean value for {name}: {value:?}. Using default: {default}."
                    );
                    default
                }
            }
        }
        Err(_) => default,
    }
}

async fn set_response_headers(
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    use axum::http::header::{
        CACHE_CONTROL, CONTENT_SECURITY_POLICY, HeaderName, HeaderValue, STRICT_TRANSPORT_SECURITY,
        VARY, X_CONTENT_TYPE_OPTIONS,
    };

    let path = request.uri().path().to_owned();
    let mut response = next.run(request).await;
    let headers = response.headers_mut();

    headers.insert(X_CONTENT_TYPE_OPTIONS, HeaderValue::from_static("nosniff"));
    headers.insert(
        HeaderName::from_static("x-frame-options"),
        HeaderValue::from_static("DENY"),
    );
    headers.insert(
        HeaderName::from_static("referrer-policy"),
        HeaderValue::from_static("strict-origin-when-cross-origin"),
    );
    headers.insert(
        HeaderName::from_static("permissions-policy"),
        HeaderValue::from_static("camera=(), microphone=(), geolocation=()"),
    );
    headers.insert(
        HeaderName::from_static("cross-origin-opener-policy"),
        HeaderValue::from_static("same-origin"),
    );
    headers.insert(
        HeaderName::from_static("cross-origin-resource-policy"),
        HeaderValue::from_static("same-site"),
    );
    headers.insert(
        HeaderName::from_static("origin-agent-cluster"),
        HeaderValue::from_static("?1"),
    );
    headers.insert(
        STRICT_TRANSPORT_SECURITY,
        HeaderValue::from_static("max-age=63072000; includeSubDomains; preload"),
    );
    headers.insert(
        CONTENT_SECURITY_POLICY,
        HeaderValue::from_static(
            "default-src 'self'; base-uri 'self'; object-src 'none'; frame-ancestors 'none'; form-action 'self'; connect-src 'self'; img-src 'self' data: https://images.unsplash.com https://lkprofessionals.com; style-src 'self' 'unsafe-inline' https://cdnjs.cloudflare.com; font-src 'self' https://cdnjs.cloudflare.com; script-src 'self'; worker-src 'self'; manifest-src 'self'; upgrade-insecure-requests; require-trusted-types-for 'script'; trusted-types default;",
        ),
    );
    headers.insert(VARY, HeaderValue::from_static("Accept-Encoding"));

    if !headers.contains_key(CACHE_CONTROL) {
        headers.insert(
            CACHE_CONTROL,
            HeaderValue::from_static(cache_control_value(&path)),
        );
    }

    response
}

fn cache_control_value(path: &str) -> &'static str {
    if path.starts_with("/static/") {
        if has_extension(path, &["png", "jpg", "jpeg", "webp", "avif", "svg", "ico"]) {
            "public, max-age=2592000, stale-while-revalidate=604800"
        } else if has_extension(path, &["css", "js"]) {
            "public, max-age=2592000, stale-while-revalidate=604800"
        } else {
            "public, max-age=604800, stale-while-revalidate=86400"
        }
    } else if matches!(path, "/robots.txt" | "/llms.txt" | "/sitemap.xml") {
        "public, max-age=86400, stale-while-revalidate=604800"
    } else {
        "public, max-age=600, stale-while-revalidate=3600"
    }
}

fn has_extension(path: &str, extensions: &[&str]) -> bool {
    path.rsplit('.')
        .next()
        .map(|extension| {
            extensions
                .iter()
                .any(|candidate| extension.eq_ignore_ascii_case(candidate))
        })
        .unwrap_or(false)
}
