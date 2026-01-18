mod hex;
mod map;
mod unit;
mod game_state;
mod zoc;
mod movement;
mod combat;
mod replacement;
mod retreat;
mod api;

use api::{create_router, AppState};
use game_state::GameState;
use map::Map;
use unit::Units;
use std::sync::{Arc, RwLock};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    println!("Battle for Moscow - Starting server...");

    // Load data files
    let units_json = std::fs::read_to_string("data/units.json")
        .expect("Failed to read units.json");
    let units = Units::load_from_json(&units_json)
        .expect("Failed to parse units.json");

    let map_json = std::fs::read_to_string("data/map.json")
        .expect("Failed to read map.json");
    let map = Map::load_from_json(&map_json)
        .expect("Failed to parse map.json");

    // Create initial game state
    let game = GameState::new();

    // Create shared application state
    let app_state = AppState {
        game: Arc::new(RwLock::new(game)),
        units: Arc::new(units),
        map: Arc::new(map),
    };

    // Build the application with routes
    let app = create_router(app_state)
        .nest_service("/", ServeDir::new("static"));

    // Run the server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");
    println!("Open http://127.0.0.1:3000 in your browser");

    axum::serve(listener, app).await.unwrap();
}
