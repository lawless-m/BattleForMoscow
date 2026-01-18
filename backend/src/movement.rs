use crate::game_state::{GameState, Phase};
use crate::hex::Hex;
use crate::map::Map;
use crate::unit::{Side, Units};
use crate::zoc::{calculate_enemy_zoc, is_in_enemy_zoc};
use std::collections::{HashMap, HashSet, VecDeque};

/// Find all valid destination hexes for a unit from its current position
pub fn find_valid_moves(
    unit_id: &str,
    state: &GameState,
    units: &Units,
    map: &Map,
) -> Result<Vec<Hex>, String> {
    // Get the unit state
    let unit_state = state
        .get_unit(unit_id)
        .ok_or_else(|| format!("Unit {} not found", unit_id))?;

    // Get the unit definition
    let unit_def = units
        .get(unit_id)
        .ok_or_else(|| format!("Unit definition for {} not found", unit_id))?;

    // Get current position
    let start_hex = unit_state
        .hex()
        .ok_or_else(|| format!("Unit {} is eliminated", unit_id))?;

    // Calculate movement allowance based on phase and mud
    let mut movement_allowance = unit_def.movement;

    // Check if in mud turn
    if state.is_mud() {
        // Rail movement not affected by mud
        if !matches!(state.phase, Phase::SovietRailMovement) {
            movement_allowance = 1;
        }
    }

    // Check what type of movement is allowed based on phase
    let rail_movement_only = matches!(state.phase, Phase::SovietRailMovement);

    // Calculate enemy ZOC
    let enemy_zoc = calculate_enemy_zoc(state, units, unit_def.side);

    // BFS to find all reachable hexes
    let mut reachable = HashMap::new();
    let mut queue = VecDeque::new();

    // Start position has 0 cost
    reachable.insert(start_hex, 0);
    queue.push_back((start_hex, 0, false)); // (hex, mp_used, stopped_by_zoc)

    while let Some((current_hex, mp_used, stopped)) = queue.pop_front() {
        if stopped {
            continue;
        }

        // Explore neighbors
        for neighbor in current_hex.neighbors() {
            // Check if hex is in bounds
            if !map.is_in_bounds(&neighbor) {
                continue;
            }

            // Cannot enter hex with enemy unit
            if has_unit_of_side(state, units, &neighbor, opposite_side(unit_def.side)) {
                continue;
            }

            // For rail movement, must stay on rail
            if rail_movement_only {
                if let Some(map_hex) = map.get_hex(&neighbor) {
                    if !map_hex.rail {
                        continue;
                    }
                } else {
                    continue;
                }
            }

            // Calculate movement cost
            let move_cost = map
                .movement_cost(&neighbor, rail_movement_only)
                .unwrap_or(999);
            let new_mp_used = mp_used + move_cost;

            // Check if we have enough movement
            if new_mp_used > movement_allowance {
                continue;
            }

            // Check if this is a better path to this hex
            if let Some(&existing_cost) = reachable.get(&neighbor) {
                if new_mp_used >= existing_cost {
                    continue;
                }
            }

            // Check if we're entering enemy ZOC (stops movement)
            let entering_zoc = enemy_zoc.contains(&neighbor);
            let stopped_by_zoc = entering_zoc;

            // Update reachable hexes
            reachable.insert(neighbor, new_mp_used);
            queue.push_back((neighbor, new_mp_used, stopped_by_zoc));
        }
    }

    // Remove starting hex from valid moves
    reachable.remove(&start_hex);

    // Convert to vector and remove hexes with friendly units (can't end stacked)
    let valid_hexes: Vec<Hex> = reachable
        .keys()
        .filter(|hex| !has_unit_of_side(state, units, hex, unit_def.side))
        .copied()
        .collect();

    Ok(valid_hexes)
}

/// Check if there's a unit of a specific side at a hex
fn has_unit_of_side(state: &GameState, units: &Units, hex: &Hex, side: Side) -> bool {
    for unit_state in &state.units {
        if let Some(unit_hex) = unit_state.hex() {
            if unit_hex == *hex {
                if let Some(unit_def) = units.get(&unit_state.id) {
                    if unit_def.side == side {
                        return true;
                    }
                }
            }
        }
    }
    false
}

/// Get the opposite side
fn opposite_side(side: Side) -> Side {
    match side {
        Side::German => Side::Soviet,
        Side::Soviet => Side::German,
    }
}

/// Validate a specific move
pub fn validate_move(
    unit_id: &str,
    destination: &Hex,
    state: &GameState,
    units: &Units,
    map: &Map,
) -> Result<(), String> {
    // Check that it's the correct phase
    if !state.phase.is_movement_phase() {
        return Err("Not a movement phase".to_string());
    }

    // Get the unit
    let unit_def = units
        .get(unit_id)
        .ok_or_else(|| format!("Unit {} not found", unit_id))?;

    // Check that it's the correct player's turn
    if unit_def.side != state.active_player() {
        return Err("Not your turn".to_string());
    }

    // Check if unit can move in this phase
    match state.phase {
        Phase::GermanPanzerMovement => {
            if !unit_def.can_move_in_panzer_phase() {
                return Err("Only panzers can move in panzer phase".to_string());
            }
        }
        Phase::SovietRailMovement => {
            // Must start on rail
            if let Some(unit_state) = state.get_unit(unit_id) {
                if let Some(start_hex) = unit_state.hex() {
                    if let Some(map_hex) = map.get_hex(&start_hex) {
                        if !map_hex.rail {
                            return Err("Unit not on rail line".to_string());
                        }
                    }
                }
            }
        }
        _ => {}
    }

    // Check if unit has already moved (unless panzer in regular movement after panzer phase)
    if state.has_moved(unit_id) {
        match state.phase {
            Phase::GermanMovement => {
                // Panzers can move again
                if !unit_def.can_move_in_panzer_phase() {
                    return Err("Unit has already moved this phase".to_string());
                }
            }
            Phase::SovietMovement => {
                // Units can move again after rail movement
                // This is OK
            }
            _ => {
                return Err("Unit has already moved this phase".to_string());
            }
        }
    }

    // Find valid moves and check if destination is in the list
    let valid_moves = find_valid_moves(unit_id, state, units, map)?;

    if !valid_moves.contains(destination) {
        return Err(format!(
            "Cannot reach hex ({}, {})",
            destination.q, destination.r
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit::{UnitDefinition, UnitState, UnitStrength, UnitType};

    #[test]
    fn test_find_valid_moves_basic() {
        // This is a simplified test; full testing would require a complete map
        let mut state = GameState::new();
        state.phase = Phase::GermanMovement;

        state.units.push(UnitState::new(
            "V".to_string(),
            Some(Hex::new(1, 1)),
            UnitStrength::Full,
        ));

        let mut units = Units { units: Vec::new() };
        units.units.push(UnitDefinition {
            id: "V".to_string(),
            side: Side::German,
            unit_type: UnitType::Infantry,
            full_strength: 6,
            half_strength: 3,
            movement: 4,
            available_turn: None,
        });

        // Create a simple test map
        // Note: In a real scenario, you'd use an actual map
        // For now, this test is more of a structure verification
    }
}
