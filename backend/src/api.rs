use crate::combat::{apply_combat_result, resolve_battle};
use crate::game_state::{BattleDeclaration, GameState};
use crate::hex::Hex;
use crate::map::Map;
use crate::movement::{find_valid_moves, validate_move};
use crate::replacement::{apply_replacement, get_valid_replacement_hexes, validate_replacement_placement};
use crate::retreat::{execute_retreat, find_valid_retreat_hexes};
use crate::unit::{Units, UnitState, UnitStrength};
use axum::{
    extract::State as AxumState,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

/// Shared application state
#[derive(Clone)]
pub struct AppState {
    pub game: Arc<RwLock<GameState>>,
    pub units: Arc<Units>,
    pub map: Arc<Map>,
}

/// API response wrapper
#[derive(Serialize)]
pub struct ApiResponse<T> {
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

impl<T> ApiResponse<T> {
    fn ok(data: T) -> Self {
        ApiResponse {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    fn error(msg: String) -> Self {
        ApiResponse {
            success: false,
            data: None,
            error: Some(msg),
        }
    }
}

/// Request to create a new game
#[derive(Deserialize)]
pub struct NewGameRequest {
    // Could include options like scenario selection
}

/// Request to move a unit
#[derive(Deserialize)]
pub struct MoveRequest {
    pub unit_id: String,
    pub to_q: i32,
    pub to_r: i32,
}

/// Request to declare a battle
#[derive(Deserialize)]
pub struct DeclareBattleRequest {
    pub attacker_ids: Vec<String>,
    pub defender_id: String,
}

/// Request to resolve a battle
#[derive(Deserialize)]
pub struct ResolveBattleRequest {
    pub battle_index: usize,
}

/// Request to apply replacement
#[derive(Deserialize)]
pub struct ReplacementRequest {
    pub unit_id: String,
    pub hex_q: Option<i32>,
    pub hex_r: Option<i32>,
}

/// Request to retreat a unit
#[derive(Deserialize)]
pub struct RetreatRequest {
    pub unit_id: String,
    pub to_q: i32,
    pub to_r: i32,
}

/// Create the API router
pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/api/game", get(get_game_state))
        .route("/api/game/new", post(new_game))
        .route("/api/game/advance-phase", post(advance_phase))
        .route("/api/units/move", post(move_unit))
        .route("/api/units/:unit_id/valid-moves", get(get_valid_moves))
        .route("/api/battle/declare", post(declare_battle))
        .route("/api/battle/resolve", post(resolve_battle_endpoint))
        .route("/api/replacement/apply", post(apply_replacement_endpoint))
        .route("/api/replacement/valid-hexes", get(get_replacement_hexes))
        .route("/api/retreat/execute", post(retreat_unit))
        .route("/api/retreat/:unit_id/valid-hexes", get(get_retreat_hexes))
        .route("/api/map", get(get_map))
        .route("/api/units", get(get_units))
        .with_state(state)
}

/// Get current game state
async fn get_game_state(
    AxumState(state): AxumState<AppState>,
) -> impl IntoResponse {
    let game = state.game.read().unwrap();
    Json(ApiResponse::ok(game.clone()))
}

/// Create a new game
async fn new_game(
    AxumState(state): AxumState<AppState>,
    Json(_req): Json<NewGameRequest>,
) -> impl IntoResponse {
    let mut game = state.game.write().unwrap();
    *game = GameState::new();

    // Initialize unit positions from map setup markers
    for map_hex in &state.map.hexes {
        if let Some(ref setup) = map_hex.setup {
            // Find a matching unit for this setup location
            // This is simplified; in a real game we'd have a proper setup algorithm
            for unit_def in &state.units.units {
                let has_this_unit = game.units.iter().any(|u| u.id == unit_def.id);
                if !has_this_unit {
                    let unit_side_matches = match setup {
                        crate::map::SetupMarker::German => {
                            unit_def.side == crate::unit::Side::German
                        }
                        crate::map::SetupMarker::Soviet => {
                            unit_def.side == crate::unit::Side::Soviet
                        }
                    };

                    if unit_side_matches {
                        game.units.push(UnitState::new(
                            unit_def.id.clone(),
                            Some(map_hex.hex()),
                            UnitStrength::Full,
                        ));
                        break;
                    }
                }
            }
        }
    }

    // Initialize city control based on unit positions
    for map_hex in &state.map.hexes {
        if let Some(ref city) = map_hex.city {
            // Check which side has units here
            for unit_state in &game.units {
                if let Some(unit_hex) = unit_state.hex() {
                    if unit_hex == map_hex.hex() {
                        if let Some(unit_def) = state.units.get(&unit_state.id) {
                            game.city_control.insert(city.name.clone(), unit_def.side);
                            break;
                        }
                    }
                }
            }
        }
    }

    Json(ApiResponse::ok(game.clone()))
}

/// Advance to next phase
async fn advance_phase(
    AxumState(state): AxumState<AppState>,
) -> impl IntoResponse {
    let mut game = state.game.write().unwrap();
    game.advance_phase();
    Json(ApiResponse::ok(game.clone()))
}

/// Move a unit
async fn move_unit(
    AxumState(state): AxumState<AppState>,
    Json(req): Json<MoveRequest>,
) -> impl IntoResponse {
    let mut game = state.game.write().unwrap();
    let destination = Hex::new(req.to_q, req.to_r);

    // Validate the move
    match validate_move(&req.unit_id, &destination, &game, &state.units, &state.map) {
        Ok(_) => {
            // Execute the move
            if let Some(unit) = game.get_unit_mut(&req.unit_id) {
                unit.move_to(destination);
                game.mark_moved(&req.unit_id);

                // Update city control if moved into a city
                for map_hex in &state.map.hexes {
                    if map_hex.hex() == destination {
                        if let Some(ref city) = map_hex.city {
                            if let Some(unit_def) = state.units.get(&req.unit_id) {
                                game.update_city_control(&city.name, unit_def.side);
                            }
                        }
                    }
                }

                Json(ApiResponse::ok(game.clone()))
            } else {
                Json(ApiResponse::error(format!("Unit {} not found", req.unit_id)))
            }
        }
        Err(e) => Json(ApiResponse::error(e)),
    }
}

/// Get valid moves for a unit
async fn get_valid_moves(
    AxumState(state): AxumState<AppState>,
    axum::extract::Path(unit_id): axum::extract::Path<String>,
) -> impl IntoResponse {
    let game = state.game.read().unwrap();

    match find_valid_moves(&unit_id, &game, &state.units, &state.map) {
        Ok(hexes) => Json(ApiResponse::ok(hexes)),
        Err(e) => Json(ApiResponse::error(e)),
    }
}

/// Declare a battle
async fn declare_battle(
    AxumState(state): AxumState<AppState>,
    Json(req): Json<DeclareBattleRequest>,
) -> impl IntoResponse {
    let mut game = state.game.write().unwrap();

    let battle = BattleDeclaration {
        attackers: req.attacker_ids,
        defender: req.defender_id,
        resolved: false,
    };

    game.pending_battles.push(battle);
    Json(ApiResponse::ok(game.clone()))
}

/// Resolve a pending battle
async fn resolve_battle_endpoint(
    AxumState(state): AxumState<AppState>,
    Json(req): Json<ResolveBattleRequest>,
) -> impl IntoResponse {
    let mut game = state.game.write().unwrap();

    if req.battle_index >= game.pending_battles.len() {
        return Json(ApiResponse::error("Invalid battle index".to_string()));
    }

    let battle = game.pending_battles[req.battle_index].clone();

    match resolve_battle(&battle, &mut game, &state.units, &state.map) {
        Ok(resolution) => {
            // Apply the result
            if let Err(e) = apply_combat_result(&resolution.result, &battle, &mut game, &state.units) {
                return Json(ApiResponse::error(e));
            }

            // Mark battle as resolved
            game.pending_battles[req.battle_index].resolved = true;

            Json(ApiResponse::ok(resolution))
        }
        Err(e) => Json(ApiResponse::error(e)),
    }
}

/// Apply a replacement
async fn apply_replacement_endpoint(
    AxumState(state): AxumState<AppState>,
    Json(req): Json<ReplacementRequest>,
) -> impl IntoResponse {
    let mut game = state.game.write().unwrap();

    let hex = req.hex_q.zip(req.hex_r).map(|(q, r)| Hex::new(q, r));

    // Validate if hex is provided
    if let Some(ref h) = hex {
        if let Err(e) = validate_replacement_placement(&req.unit_id, h, &game, &state.units, &state.map) {
            return Json(ApiResponse::error(e));
        }
    }

    match apply_replacement(&req.unit_id, hex.as_ref(), &mut game, &state.units) {
        Ok(_) => Json(ApiResponse::ok(game.clone())),
        Err(e) => Json(ApiResponse::error(e)),
    }
}

/// Get valid replacement hexes
async fn get_replacement_hexes(
    AxumState(state): AxumState<AppState>,
) -> impl IntoResponse {
    let game = state.game.read().unwrap();
    let side = game.active_player();
    let hexes = get_valid_replacement_hexes(side, &game, &state.units, &state.map);
    Json(ApiResponse::ok(hexes))
}

/// Execute a retreat
async fn retreat_unit(
    AxumState(state): AxumState<AppState>,
    Json(req): Json<RetreatRequest>,
) -> impl IntoResponse {
    let mut game = state.game.write().unwrap();
    let to_hex = Hex::new(req.to_q, req.to_r);

    match execute_retreat(&req.unit_id, &to_hex, &mut game, &state.units, &state.map) {
        Ok(result) => Json(ApiResponse::ok(result)),
        Err(e) => Json(ApiResponse::error(e)),
    }
}

/// Get valid retreat hexes for a unit
async fn get_retreat_hexes(
    AxumState(state): AxumState<AppState>,
    axum::extract::Path(unit_id): axum::extract::Path<String>,
) -> impl IntoResponse {
    let game = state.game.read().unwrap();

    // Get unit position
    let from_hex = match game.get_unit(&unit_id).and_then(|u| u.hex()) {
        Some(hex) => hex,
        None => return Json(ApiResponse::error(format!("Unit {} has no position", unit_id))),
    };

    let hexes = find_valid_retreat_hexes(&unit_id, &from_hex, &game, &state.units, &state.map);
    Json(ApiResponse::ok(hexes))
}

/// Get map data
async fn get_map(
    AxumState(state): AxumState<AppState>,
) -> impl IntoResponse {
    Json(ApiResponse::ok(state.map.as_ref().clone()))
}

/// Get unit definitions
async fn get_units(
    AxumState(state): AxumState<AppState>,
) -> impl IntoResponse {
    Json(ApiResponse::ok(state.units.as_ref().clone()))
}
