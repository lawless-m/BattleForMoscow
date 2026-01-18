use crate::game_state::GameState;
use crate::hex::Hex;
use crate::map::Map;
use crate::unit::{Side, Units};
use std::collections::{HashSet, VecDeque};

/// Check if a hex can trace a path to the friendly map edge (communication line)
pub fn can_trace_communication(
    hex: &Hex,
    friendly_side: Side,
    state: &GameState,
    units: &Units,
    map: &Map,
) -> bool {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    visited.insert(*hex);
    queue.push_back(*hex);

    while let Some(current) = queue.pop_front() {
        // Check if we reached the friendly edge
        match friendly_side {
            Side::German => {
                if map.is_west_edge(&current) {
                    return true;
                }
            }
            Side::Soviet => {
                if map.is_east_edge(&current) {
                    return true;
                }
            }
        }

        // Explore neighbors
        for neighbor in current.neighbors() {
            if visited.contains(&neighbor) {
                continue;
            }

            if !map.is_in_bounds(&neighbor) {
                continue;
            }

            // Cannot trace through enemy-occupied hexes
            if has_enemy_unit(&neighbor, friendly_side, state, units) {
                continue;
            }

            visited.insert(neighbor);
            queue.push_back(neighbor);
        }
    }

    false
}

/// Check if a hex has an enemy unit
fn has_enemy_unit(hex: &Hex, friendly_side: Side, state: &GameState, units: &Units) -> bool {
    for unit_state in &state.units {
        if let Some(unit_hex) = unit_state.hex() {
            if unit_hex == *hex {
                if let Some(unit_def) = units.get(&unit_state.id) {
                    if unit_def.side != friendly_side {
                        return true;
                    }
                }
            }
        }
    }
    false
}

/// Get valid replacement hexes for a given side
/// Must be a city controlled by the friendly side with communication
pub fn get_valid_replacement_hexes(
    friendly_side: Side,
    state: &GameState,
    units: &Units,
    map: &Map,
) -> Vec<Hex> {
    let mut valid_hexes = Vec::new();

    for map_hex in &map.hexes {
        // Must be a city
        if let Some(ref city) = map_hex.city {
            let hex = map_hex.hex();

            // Check control
            if let Some(controller) = state.get_city_control(&city.name) {
                if controller != friendly_side {
                    continue;
                }
            } else {
                // City not yet controlled, skip
                continue;
            }

            // For Germans: Any controlled city with communication
            // For Soviets: Moscow only (unless Moscow is lost)
            match friendly_side {
                Side::German => {
                    // Any city with communication
                    if can_trace_communication(&hex, friendly_side, state, units, map) {
                        valid_hexes.push(hex);
                    }
                }
                Side::Soviet => {
                    // Only Moscow
                    if city.is_moscow {
                        if can_trace_communication(&hex, friendly_side, state, units, map) {
                            valid_hexes.push(hex);
                        }
                    }
                }
            }
        }
    }

    valid_hexes
}

/// Validate a replacement placement
pub fn validate_replacement_placement(
    unit_id: &str,
    hex: &Hex,
    state: &GameState,
    units: &Units,
    map: &Map,
) -> Result<(), String> {
    // Get the unit
    let unit_state = state
        .get_unit(unit_id)
        .ok_or_else(|| format!("Unit {} not found", unit_id))?;

    let unit_def = units
        .get(unit_id)
        .ok_or_else(|| format!("Unit definition for {} not found", unit_id))?;

    // Unit must be eliminated or at half strength
    if unit_state.strength == crate::unit::UnitStrength::Full {
        return Err("Unit is at full strength and cannot receive replacements".to_string());
    }

    // For Germans: must have replacement available
    if unit_def.side == Side::German {
        if state.german_replacement_used {
            return Err("German replacement already used this turn".to_string());
        }
    } else {
        // For Soviets: must have replacements remaining
        if state.soviet_replacements_remaining <= 0 {
            return Err("No Soviet replacements remaining".to_string());
        }
    }

    // Check if this hex is a valid replacement hex
    let valid_hexes = get_valid_replacement_hexes(unit_def.side, state, units, map);
    if !valid_hexes.contains(hex) {
        return Err("Invalid replacement hex - must be a controlled city with communication".to_string());
    }

    // If unit is eliminated, hex must be unoccupied
    if unit_state.strength == crate::unit::UnitStrength::Eliminated {
        // Check for other units at this hex
        for other_unit in &state.units {
            if let Some(other_hex) = other_unit.hex() {
                if other_hex == *hex && other_unit.id != unit_id {
                    return Err("Hex is occupied by another unit".to_string());
                }
            }
        }
    } else {
        // If unit is at half strength, it can be replaced in place
        // or must be in a valid replacement hex
        if let Some(current_hex) = unit_state.hex() {
            if current_hex != *hex && !valid_hexes.contains(&current_hex) {
                return Err("Unit must be in a valid replacement hex or be placed in one".to_string());
            }
        }
    }

    Ok(())
}

/// Apply a replacement to a unit
pub fn apply_replacement(
    unit_id: &str,
    hex: Option<&Hex>,
    state: &mut GameState,
    units: &Units,
) -> Result<(), String> {
    let unit_def = units
        .get(unit_id)
        .ok_or_else(|| format!("Unit definition for {} not found", unit_id))?;

    // Get the unit
    let unit_state = state
        .get_unit_mut(unit_id)
        .ok_or_else(|| format!("Unit {} not found", unit_id))?;

    // Restore one step
    unit_state.restore();

    // If a hex is provided and unit was eliminated, place it there
    if let Some(placement_hex) = hex {
        if unit_state.position.is_none() {
            unit_state.move_to(*placement_hex);
        }
    }

    // Update replacement counters
    match unit_def.side {
        Side::German => {
            state.german_replacement_used = true;
        }
        Side::Soviet => {
            state.soviet_replacements_remaining -= 1;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit::{UnitDefinition, UnitState, UnitStrength, UnitType};

    #[test]
    fn test_can_trace_communication_simple() {
        // Create a simple test setup
        let mut state = GameState::new();
        let units = Units { units: Vec::new() };

        // This would need a proper map setup to test fully
        // For now, it's a structure test
    }

    #[test]
    fn test_validate_replacement_full_strength_unit() {
        let mut state = GameState::new();
        state.units.push(UnitState::new(
            "5".to_string(),
            Some(Hex::new(5, 3)),
            UnitStrength::Full,
        ));

        let mut units = Units { units: Vec::new() };
        units.units.push(UnitDefinition {
            id: "5".to_string(),
            side: Side::Soviet,
            unit_type: UnitType::Infantry,
            full_strength: 8,
            half_strength: 4,
            movement: 4,
            available_turn: None,
        });

        // Create a minimal map
        let map = Map {
            hexes: vec![],
            map_bounds: crate::map::MapBounds {
                min_q: 0,
                max_q: 10,
                min_r: 0,
                max_r: 10,
            },
            edges: crate::map::MapEdges {
                west: "german_communication".to_string(),
                east: "soviet_communication".to_string(),
            },
        };

        // Should fail because unit is at full strength
        let result = validate_replacement_placement("5", &Hex::new(5, 3), &state, &units, &map);
        assert!(result.is_err());
    }
}
