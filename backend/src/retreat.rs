use crate::game_state::GameState;
use crate::hex::Hex;
use crate::map::Map;
use crate::unit::{Side, Units};
use crate::zoc::is_in_enemy_zoc;

/// Find valid retreat hexes for a unit (2 hexes away from starting position)
pub fn find_valid_retreat_hexes(
    unit_id: &str,
    from_hex: &Hex,
    state: &GameState,
    units: &Units,
    map: &Map,
) -> Vec<Hex> {
    let unit_def = match units.get(unit_id) {
        Some(def) => def,
        None => return Vec::new(),
    };

    let mut valid_hexes = Vec::new();

    // Get all hexes exactly 2 distance away
    for map_hex in &map.hexes {
        let candidate = map_hex.hex();

        // Must be exactly 2 hexes away
        if from_hex.distance(&candidate) != 2 {
            continue;
        }

        // Must be in bounds
        if !map.is_in_bounds(&candidate) {
            continue;
        }

        // Cannot retreat to hex with any unit (friendly or enemy)
        if has_any_unit(&candidate, state) {
            continue;
        }

        // Cannot retreat to hex in enemy ZOC (will be eliminated instead)
        // But we still include it in the list so the player knows the option
        // The elimination happens during retreat resolution

        // Must be able to trace a valid 2-hex path
        // (not through enemy units or off-map)
        if can_retreat_through_path(from_hex, &candidate, state, units, map) {
            valid_hexes.push(candidate);
        }
    }

    valid_hexes
}

/// Check if there's any unit at a hex
fn has_any_unit(hex: &Hex, state: &GameState) -> bool {
    state.units.iter().any(|u| u.hex() == Some(*hex))
}

/// Check if a unit can retreat from `from` to `to` through a valid path
fn can_retreat_through_path(
    from: &Hex,
    to: &Hex,
    state: &GameState,
    units: &Units,
    map: &Map,
) -> bool {
    // For a 2-hex retreat, we need to check intermediate hexes
    // Find all hexes that are 1 away from both start and end

    for neighbor in from.neighbors() {
        if !map.is_in_bounds(&neighbor) {
            continue;
        }

        // Check if this neighbor is also adjacent to the destination
        if neighbor.is_adjacent(to) {
            // This is a valid intermediate hex
            // Check it's not occupied by an enemy
            if !has_enemy_unit_at(&neighbor, state, units) {
                return true;
            }
        }
    }

    false
}

/// Check if there's an enemy unit at a hex (from any perspective)
fn has_enemy_unit_at(hex: &Hex, state: &GameState, units: &Units) -> bool {
    for unit_state in &state.units {
        if unit_state.hex() == Some(*hex) {
            return true;
        }
    }
    false
}

/// Execute a retreat move
pub fn execute_retreat(
    unit_id: &str,
    to_hex: &Hex,
    state: &mut GameState,
    units: &Units,
    map: &Map,
) -> Result<RetreatResult, String> {
    // Get unit
    let unit_def = units
        .get(unit_id)
        .ok_or_else(|| format!("Unit {} not found", unit_id))?;

    // Get current position
    let from_hex = state
        .get_unit(unit_id)
        .and_then(|u| u.hex())
        .ok_or_else(|| format!("Unit {} has no position", unit_id))?;

    // Validate retreat hex
    let valid_hexes = find_valid_retreat_hexes(unit_id, &from_hex, state, units, map);
    if !valid_hexes.contains(to_hex) {
        return Err(format!(
            "Invalid retreat hex ({}, {})",
            to_hex.q, to_hex.r
        ));
    }

    // Check if retreating into enemy ZOC (unit is eliminated)
    if is_in_enemy_zoc(to_hex, state, units, unit_def.side) {
        // Eliminate the unit
        if let Some(unit) = state.get_unit_mut(unit_id) {
            unit.strength = crate::unit::UnitStrength::Eliminated;
            unit.position = None;
        }
        return Ok(RetreatResult::EliminatedInZoc);
    }

    // Execute the retreat
    if let Some(unit) = state.get_unit_mut(unit_id) {
        unit.move_to(*to_hex);
    }

    Ok(RetreatResult::Success)
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum RetreatResult {
    Success,
    EliminatedInZoc,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit::{UnitDefinition, UnitState, UnitStrength, UnitType};

    #[test]
    fn test_retreat_distance() {
        // Test that retreat hexes are exactly 2 away
        let from = Hex::new(5, 5);
        let candidate1 = Hex::new(7, 5); // 2 away
        let candidate2 = Hex::new(6, 5); // 1 away
        let candidate3 = Hex::new(8, 5); // 3 away

        assert_eq!(from.distance(&candidate1), 2);
        assert_eq!(from.distance(&candidate2), 1);
        assert_eq!(from.distance(&candidate3), 3);
    }

    #[test]
    fn test_has_any_unit() {
        let mut state = GameState::new();
        state.units.push(UnitState::new(
            "5".to_string(),
            Some(Hex::new(5, 3)),
            UnitStrength::Full,
        ));

        assert!(has_any_unit(&Hex::new(5, 3), &state));
        assert!(!has_any_unit(&Hex::new(5, 4), &state));
    }
}
