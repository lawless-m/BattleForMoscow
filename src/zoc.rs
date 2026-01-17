use crate::game_state::GameState;
use crate::hex::Hex;
use crate::map::Map;
use crate::unit::{Side, Units};
use std::collections::HashSet;

/// Calculate all hexes that are in enemy ZOC for a given side
pub fn calculate_enemy_zoc(
    state: &GameState,
    units: &Units,
    friendly_side: Side,
) -> HashSet<Hex> {
    let mut zoc_hexes = HashSet::new();

    // For each unit on the board
    for unit_state in &state.units {
        if let Some(unit_hex) = unit_state.hex() {
            // Get the unit definition to check its side
            if let Some(unit_def) = units.get(&unit_state.id) {
                // If this is an enemy unit
                if unit_def.side != friendly_side {
                    // Add all neighboring hexes to enemy ZOC
                    for neighbor in unit_hex.neighbors() {
                        zoc_hexes.insert(neighbor);
                    }
                }
            }
        }
    }

    zoc_hexes
}

/// Check if a hex is in enemy ZOC
pub fn is_in_enemy_zoc(
    hex: &Hex,
    state: &GameState,
    units: &Units,
    friendly_side: Side,
) -> bool {
    let zoc = calculate_enemy_zoc(state, units, friendly_side);
    zoc.contains(hex)
}

/// Find all enemy units adjacent to a given hex
pub fn adjacent_enemies(
    hex: &Hex,
    state: &GameState,
    units: &Units,
    friendly_side: Side,
) -> Vec<String> {
    let mut enemies = Vec::new();

    for neighbor in hex.neighbors() {
        for unit_state in &state.units {
            if let Some(unit_hex) = unit_state.hex() {
                if unit_hex == neighbor {
                    if let Some(unit_def) = units.get(&unit_state.id) {
                        if unit_def.side != friendly_side {
                            enemies.push(unit_state.id.clone());
                        }
                    }
                }
            }
        }
    }

    enemies
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit::{UnitDefinition, UnitState, UnitStrength, UnitType};

    #[test]
    fn test_calculate_enemy_zoc() {
        let mut state = GameState::new();

        // Add a German unit at (5, 3)
        state.units.push(UnitState::new(
            "XLVII".to_string(),
            Some(Hex::new(5, 3)),
            UnitStrength::Full,
        ));

        // Create unit definitions
        let mut units = Units { units: Vec::new() };
        units.units.push(UnitDefinition {
            id: "XLVII".to_string(),
            side: Side::German,
            unit_type: UnitType::Panzer,
            full_strength: 9,
            half_strength: 4,
            movement: 6,
            available_turn: None,
        });

        // Calculate Soviet perspective (German is enemy)
        let zoc = calculate_enemy_zoc(&state, &units, Side::Soviet);

        // All 6 neighbors should be in ZOC
        assert_eq!(zoc.len(), 6);
        assert!(zoc.contains(&Hex::new(6, 3)));
        assert!(zoc.contains(&Hex::new(5, 4)));
        assert!(zoc.contains(&Hex::new(4, 4)));
        assert!(zoc.contains(&Hex::new(4, 3)));
        assert!(zoc.contains(&Hex::new(5, 2)));
        assert!(zoc.contains(&Hex::new(6, 2)));
    }

    #[test]
    fn test_adjacent_enemies() {
        let mut state = GameState::new();

        // Add two German units adjacent to (5, 3)
        state.units.push(UnitState::new(
            "XLVII".to_string(),
            Some(Hex::new(6, 3)),
            UnitStrength::Full,
        ));
        state.units.push(UnitState::new(
            "VII".to_string(),
            Some(Hex::new(5, 4)),
            UnitStrength::Full,
        ));

        let mut units = Units { units: Vec::new() };
        units.units.push(UnitDefinition {
            id: "XLVII".to_string(),
            side: Side::German,
            unit_type: UnitType::Panzer,
            full_strength: 9,
            half_strength: 4,
            movement: 6,
            available_turn: None,
        });
        units.units.push(UnitDefinition {
            id: "VII".to_string(),
            side: Side::German,
            unit_type: UnitType::Infantry,
            full_strength: 7,
            half_strength: 4,
            movement: 4,
            available_turn: None,
        });

        let enemies = adjacent_enemies(&Hex::new(5, 3), &state, &units, Side::Soviet);
        assert_eq!(enemies.len(), 2);
        assert!(enemies.contains(&"XLVII".to_string()));
        assert!(enemies.contains(&"VII".to_string()));
    }
}
