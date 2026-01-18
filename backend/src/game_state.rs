use crate::hex::Hex;
use crate::unit::{Side, UnitState};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Phase {
    GermanReplacement,
    GermanPanzerMovement,
    GermanCombat,
    GermanMovement,
    SovietReplacement,
    SovietRailMovement,
    SovietCombat,
    SovietMovement,
}

impl Phase {
    /// Get the player who acts during this phase
    pub fn active_player(&self) -> Side {
        match self {
            Phase::GermanReplacement
            | Phase::GermanPanzerMovement
            | Phase::GermanCombat
            | Phase::GermanMovement => Side::German,
            Phase::SovietReplacement
            | Phase::SovietRailMovement
            | Phase::SovietCombat
            | Phase::SovietMovement => Side::Soviet,
        }
    }

    /// Check if this is a movement phase
    pub fn is_movement_phase(&self) -> bool {
        matches!(
            self,
            Phase::GermanPanzerMovement
                | Phase::GermanMovement
                | Phase::SovietRailMovement
                | Phase::SovietMovement
        )
    }

    /// Check if this is a combat phase
    pub fn is_combat_phase(&self) -> bool {
        matches!(self, Phase::GermanCombat | Phase::SovietCombat)
    }

    /// Check if this is a replacement phase
    pub fn is_replacement_phase(&self) -> bool {
        matches!(self, Phase::GermanReplacement | Phase::SovietReplacement)
    }

    /// Get the next phase in sequence
    pub fn next(&self) -> Phase {
        match self {
            Phase::GermanReplacement => Phase::GermanPanzerMovement,
            Phase::GermanPanzerMovement => Phase::GermanCombat,
            Phase::GermanCombat => Phase::GermanMovement,
            Phase::GermanMovement => Phase::SovietReplacement,
            Phase::SovietReplacement => Phase::SovietRailMovement,
            Phase::SovietRailMovement => Phase::SovietCombat,
            Phase::SovietCombat => Phase::SovietMovement,
            Phase::SovietMovement => Phase::GermanReplacement, // Wraps to next turn
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleDeclaration {
    pub attackers: Vec<String>,
    pub defender: String,
    pub resolved: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub turn: i32,
    pub phase: Phase,
    pub units: Vec<UnitState>,
    pub city_control: HashMap<String, Side>,
    pub pending_battles: Vec<BattleDeclaration>,
    pub moved_this_phase: HashSet<String>,
    pub german_replacement_used: bool,
    pub soviet_replacements_remaining: i32,
    pub first_shock_army_available: bool,
}

impl GameState {
    /// Create a new game state with initial setup
    pub fn new() -> Self {
        GameState {
            turn: 1,
            phase: Phase::GermanPanzerMovement, // Skip replacement on turn 1
            units: Vec::new(),
            city_control: HashMap::new(),
            pending_battles: Vec::new(),
            moved_this_phase: HashSet::new(),
            german_replacement_used: false,
            soviet_replacements_remaining: 0,
            first_shock_army_available: false,
        }
    }

    /// Check if the game is in mud turns (turns 3 and 4)
    pub fn is_mud(&self) -> bool {
        self.turn == 3 || self.turn == 4
    }

    /// Get the active player for the current phase
    pub fn active_player(&self) -> Side {
        self.phase.active_player()
    }

    /// Advance to the next phase
    pub fn advance_phase(&mut self) {
        let next_phase = self.phase.next();

        // Check if we're starting a new turn
        if matches!(self.phase, Phase::SovietMovement) {
            self.turn += 1;

            // Update 1st Shock Army availability
            if self.turn >= 4 {
                self.first_shock_army_available = true;
            }
        }

        self.phase = next_phase;
        self.moved_this_phase.clear();
        self.pending_battles.clear();

        // Reset replacement counters at the start of replacement phases
        match self.phase {
            Phase::GermanReplacement => {
                self.german_replacement_used = false;
            }
            Phase::SovietReplacement => {
                self.soviet_replacements_remaining = 5;
            }
            _ => {}
        }

        // Skip German replacement phase on turn 1
        if self.turn == 1 && self.phase == Phase::GermanReplacement {
            self.advance_phase();
        }
    }

    /// Get a unit by ID
    pub fn get_unit(&self, id: &str) -> Option<&UnitState> {
        self.units.iter().find(|u| u.id == id)
    }

    /// Get a mutable reference to a unit by ID
    pub fn get_unit_mut(&mut self, id: &str) -> Option<&mut UnitState> {
        self.units.iter_mut().find(|u| u.id == id)
    }

    /// Get all units at a given hex
    pub fn get_units_at(&self, hex: &Hex) -> Vec<&UnitState> {
        self.units
            .iter()
            .filter(|u| u.hex() == Some(*hex))
            .collect()
    }

    /// Check if a hex contains an enemy unit
    pub fn has_enemy_unit(&self, hex: &Hex, friendly_side: Side) -> bool {
        self.units.iter().any(|u| {
            if let Some(unit_hex) = u.hex() {
                // We need to check the unit's side from unit definitions
                // For now, we'll use a naming convention: German units use Roman numerals
                let is_german = u.id.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() && c > '9');
                let unit_side = if is_german {
                    Side::German
                } else {
                    Side::Soviet
                };

                unit_hex == *hex && unit_side != friendly_side
            } else {
                false
            }
        })
    }

    /// Mark a unit as having moved this phase
    pub fn mark_moved(&mut self, unit_id: &str) {
        self.moved_this_phase.insert(unit_id.to_string());
    }

    /// Check if a unit has moved this phase
    pub fn has_moved(&self, unit_id: &str) -> bool {
        self.moved_this_phase.contains(unit_id)
    }

    /// Update city control when a unit enters a city
    pub fn update_city_control(&mut self, city_name: &str, side: Side) {
        self.city_control.insert(city_name.to_string(), side);
    }

    /// Get the controlling side of a city
    pub fn get_city_control(&self, city_name: &str) -> Option<Side> {
        self.city_control.get(city_name).cloned()
    }

    /// Check victory condition (who controls Moscow)
    pub fn check_victory(&self) -> Option<Side> {
        if self.turn > 7 {
            self.get_city_control("Moscow")
        } else {
            None
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase_progression() {
        let mut state = GameState::new();
        assert_eq!(state.phase, Phase::GermanPanzerMovement);
        assert_eq!(state.turn, 1);

        state.advance_phase();
        assert_eq!(state.phase, Phase::GermanCombat);

        state.advance_phase();
        assert_eq!(state.phase, Phase::GermanMovement);

        state.advance_phase();
        assert_eq!(state.phase, Phase::SovietReplacement);
    }

    #[test]
    fn test_turn_progression() {
        let mut state = GameState::new();
        state.phase = Phase::SovietMovement;
        state.turn = 1;

        state.advance_phase();
        assert_eq!(state.turn, 2);
        assert_eq!(state.phase, Phase::GermanReplacement);
    }

    #[test]
    fn test_mud_turns() {
        let mut state = GameState::new();
        state.turn = 1;
        assert!(!state.is_mud());

        state.turn = 3;
        assert!(state.is_mud());

        state.turn = 4;
        assert!(state.is_mud());

        state.turn = 5;
        assert!(!state.is_mud());
    }

    #[test]
    fn test_first_shock_army_availability() {
        let mut state = GameState::new();
        assert!(!state.first_shock_army_available);

        state.turn = 3;
        state.phase = Phase::SovietMovement;
        state.advance_phase();

        assert_eq!(state.turn, 4);
        assert!(state.first_shock_army_available);
    }
}
