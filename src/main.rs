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
use tower_sessions::{MemoryStore, SessionManagerLayer};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // Database connection
    let db = db::connect_db().await;

    // Run migrations automatically
    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .expect("Failed to run migrations");

    // Seed default admin user
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

    start_newsletter_worker(db.clone()).await;

    // Shared app state
    let state = AppState { db };

    // Session store
    let session_store = MemoryStore::default();

    let session_layer = SessionManagerLayer::new(session_store).with_secure(false);

    // App
    let app = app_routes(state).layer(session_layer);

    // Server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind server address");

    println!("Running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.expect("Server failed");
}
