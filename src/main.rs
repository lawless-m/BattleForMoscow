mod hex;
mod map;
mod unit;
mod game_state;
mod zoc;
mod movement;
mod combat;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    println!("Battle for Moscow - Starting server...");

    // Build the application with routes
    let app = Router::new()
        .route("/", get(|| async { "Battle for Moscow API Server" }))
        .nest_service("/static", ServeDir::new("static"));

    // Run the server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
