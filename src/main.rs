mod handlers;
mod routes;
mod db;
mod seeds;

use routes::app_routes;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

let db = db::connect_db().await;

seeds::users::seed_default_user(&db)
    .await
    .expect("Failed to seed default user");
    let app = app_routes();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
