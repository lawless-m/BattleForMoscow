use crate::game_state::{BattleDeclaration, GameState};
use crate::hex::{Direction, Hex};
use crate::map::{Map, Terrain};
use crate::unit::{Side, UnitStrength, Units};
use crate::zoc::is_in_enemy_zoc;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CombatResult {
    NE,  // No Effect
    DR,  // Defender Retreat
    DRL, // Defender Retreat with Loss
    AL,  // Attacker Loss
    DE,  // Defender Eliminated
    EX,  // Exchange
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleResolution {
    pub die_roll: i32,
    pub raw_odds: String,
    pub modified_odds: String,
    pub terrain_modifiers: Vec<String>,
    pub result: CombatResult,
    pub result_description: String,
}

/// Combat Results Table
/// Returns the result for a given die roll (1-6) and odds column
fn combat_results_table(die_roll: i32, odds_level: i32) -> CombatResult {
    match (die_roll, odds_level) {
        (1, 0) => CombatResult::AL,  // 1:1
        (2, 0) => CombatResult::AL,
        (3, 0) => CombatResult::AL,
        (4, 0) => CombatResult::NE,
        (5, 0) => CombatResult::NE,
        (6, 0) => CombatResult::DR,

        (1, 1) => CombatResult::AL,  // 2:1
        (2, 1) => CombatResult::AL,
        (3, 1) => CombatResult::NE,
        (4, 1) => CombatResult::NE,
        (5, 1) => CombatResult::DR,
        (6, 1) => CombatResult::DRL,

        (1, 2) => CombatResult::AL,  // 3:1
        (2, 2) => CombatResult::NE,
        (3, 2) => CombatResult::NE,
        (4, 2) => CombatResult::DR,
        (5, 2) => CombatResult::DR,
        (6, 2) => CombatResult::DRL,

        (1, 3) => CombatResult::NE,  // 4:1
        (2, 3) => CombatResult::NE,
        (3, 3) => CombatResult::DR,
        (4, 3) => CombatResult::DR,
        (5, 3) => CombatResult::DRL,
        (6, 3) => CombatResult::DE,

        (1, 4) => CombatResult::NE,  // 5:1
        (2, 4) => CombatResult::DR,
        (3, 4) => CombatResult::DR,
        (4, 4) => CombatResult::DRL,
        (5, 4) => CombatResult::DRL,
        (6, 4) => CombatResult::DE,

        (1, 5) => CombatResult::DR,  // 6:1
        (2, 5) => CombatResult::DR,
        (3, 5) => CombatResult::DRL,
        (4, 5) => CombatResult::DRL,
        (5, 5) => CombatResult::DE,
        (6, 5) => CombatResult::DE,

        _ => CombatResult::NE, // Invalid or out of range
    }
}

/// Convert odds ratio to odds level (0 = 1:1, 1 = 2:1, ..., 5 = 6:1)
fn odds_to_level(odds_ratio: f32) -> i32 {
    if odds_ratio < 1.0 {
        -1 // Below 1:1 = no effect
    } else if odds_ratio >= 6.0 {
        5 // 6:1 max
    } else {
        (odds_ratio.floor() as i32) - 1
    }
}

/// Format odds level as string
fn level_to_odds_string(level: i32) -> String {
    match level {
        -1 => "Below 1:1".to_string(),
        0 => "1:1".to_string(),
        1 => "2:1".to_string(),
        2 => "3:1".to_string(),
        3 => "4:1".to_string(),
        4 => "5:1".to_string(),
        5 => "6:1".to_string(),
        _ => "Invalid".to_string(),
    }
}

/// Resolve a single battle
pub fn resolve_battle(
    battle: &BattleDeclaration,
    state: &mut GameState,
    units: &Units,
    map: &Map,
) -> Result<BattleResolution, String> {
    // Get defender
    let defender_state = state
        .get_unit(&battle.defender)
        .ok_or("Defender not found")?
        .clone();

    let defender_def = units
        .get(&battle.defender)
        .ok_or("Defender definition not found")?;

    let defender_hex = defender_state
        .hex()
        .ok_or("Defender is eliminated")?;

    // Calculate total attack strength
    let mut total_attack = 0;
    let mut attacker_side = None;
    let mut attacker_hexes = Vec::new();

    for attacker_id in &battle.attackers {
        let attacker_state = state
            .get_unit(attacker_id)
            .ok_or(format!("Attacker {} not found", attacker_id))?
            .clone();

        let attacker_def = units
            .get(attacker_id)
            .ok_or(format!("Attacker definition for {} not found", attacker_id))?;

        if attacker_side.is_none() {
            attacker_side = Some(attacker_def.side.clone());
        }

        let mut strength = attacker_def.get_combat_strength(&attacker_state.strength);

        // Halve attack strength in mud turns
        if state.is_mud() {
            strength = strength / 2;
        }

        total_attack += strength;

        if let Some(hex) = attacker_state.hex() {
            attacker_hexes.push(hex);
        }
    }

    // Get defender strength
    let defender_strength = defender_def.get_combat_strength(&defender_state.strength);

    // Calculate raw odds
    let raw_odds_ratio = total_attack as f32 / defender_strength as f32;
    let mut odds_level = odds_to_level(raw_odds_ratio);

    // Apply terrain modifiers
    let mut modifiers = Vec::new();

    if let Some(map_hex) = map.get_hex(&defender_hex) {
        // Forest defense
        if map_hex.terrain == Terrain::Forest {
            odds_level -= 1;
            modifiers.push("Forest".to_string());
        }

        // Moscow defense
        if let Some(ref city) = map_hex.city {
            if city.is_moscow {
                odds_level -= 1;
                modifiers.push("Moscow".to_string());
            }
        }

        // Fortification (Soviet only)
        if map_hex.fortification && defender_def.side == Side::Soviet {
            odds_level -= 1;
            modifiers.push("Fortification".to_string());
        }

        // River crossing - check if ALL attackers are across river from defender
        let mut all_across_river = true;
        for attacker_hex in &attacker_hexes {
            if let Some(direction) = attacker_hex.direction_to(&defender_hex) {
                if !map_hex.has_river_edge(direction) {
                    all_across_river = false;
                    break;
                }
            }
        }

        if all_across_river && !attacker_hexes.is_empty() {
            odds_level -= 1;
            modifiers.push("River".to_string());
        }
    }

    // Below 1:1 after modifiers = no effect
    if odds_level < 0 {
        return Ok(BattleResolution {
            die_roll: 0,
            raw_odds: level_to_odds_string(odds_to_level(raw_odds_ratio)),
            modified_odds: "Below 1:1".to_string(),
            terrain_modifiers: modifiers,
            result: CombatResult::NE,
            result_description: "Odds too low - No Effect".to_string(),
        });
    }

    // Roll die
    let mut rng = rand::thread_rng();
    let die_roll = rng.gen_range(1..=6);

    // Look up result
    let result = combat_results_table(die_roll, odds_level);

    // Create resolution
    let resolution = BattleResolution {
        die_roll,
        raw_odds: level_to_odds_string(odds_to_level(raw_odds_ratio)),
        modified_odds: level_to_odds_string(odds_level),
        terrain_modifiers: modifiers,
        result: result.clone(),
        result_description: format!("{:?}", result),
    };

    // Apply result (this would be done in a separate function in full implementation)
    // For now, just return the resolution

    Ok(resolution)
}

/// Apply combat result to game state
pub fn apply_combat_result(
    result: &CombatResult,
    battle: &BattleDeclaration,
    state: &mut GameState,
    units: &Units,
) -> Result<(), String> {
    match result {
        CombatResult::NE => {
            // Nothing happens
            Ok(())
        }
        CombatResult::AL => {
            // Attacker chooses one unit to take a loss
            // For now, just take the first attacker
            if let Some(attacker_id) = battle.attackers.first() {
                if let Some(unit) = state.get_unit_mut(attacker_id) {
                    unit.take_loss();
                }
            }
            Ok(())
        }
        CombatResult::DR => {
            // Defender retreats (path must be chosen)
            // This requires additional logic for retreat path selection
            Ok(())
        }
        CombatResult::DRL => {
            // Defender takes loss, then retreats
            if let Some(unit) = state.get_unit_mut(&battle.defender) {
                unit.take_loss();
            }
            // Then retreat (requires path selection)
            Ok(())
        }
        CombatResult::DE => {
            // Defender eliminated
            if let Some(unit) = state.get_unit_mut(&battle.defender) {
                unit.strength = UnitStrength::Eliminated;
                unit.position = None;
            }
            Ok(())
        }
        CombatResult::EX => {
            // Exchange - defender takes loss, attacker must match
            // This is complex and requires UI interaction
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combat_results_table() {
        // Test 1:1 odds
        assert_eq!(combat_results_table(1, 0), CombatResult::AL);
        assert_eq!(combat_results_table(6, 0), CombatResult::DR);

        // Test 6:1 odds
        assert_eq!(combat_results_table(1, 5), CombatResult::DR);
        assert_eq!(combat_results_table(6, 5), CombatResult::DE);
    }

    #[test]
    fn test_odds_conversion() {
        assert_eq!(odds_to_level(0.5), -1); // Below 1:1
        assert_eq!(odds_to_level(1.0), 0);  // 1:1
        assert_eq!(odds_to_level(2.5), 1);  // 2:1
        assert_eq!(odds_to_level(6.0), 5);  // 6:1
        assert_eq!(odds_to_level(10.0), 5); // Capped at 6:1
    }
}
