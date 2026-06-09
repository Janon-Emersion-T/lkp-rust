mod handlers;
mod routes;

use routes::app_routes;

#[tokio::main]
async fn main() {
    let app = app_routes();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
