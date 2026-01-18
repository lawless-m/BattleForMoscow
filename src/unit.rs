use crate::hex::Hex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Side {
    German,
    Soviet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UnitType {
    Infantry,
    Panzer,
}

/// Static unit definition (from units.json)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitDefinition {
    pub id: String,
    pub side: Side,
    #[serde(rename = "type")]
    pub unit_type: UnitType,
    pub full_strength: i32,
    pub half_strength: i32,
    pub movement: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_turn: Option<i32>, // For 1st Shock Army
}

impl UnitDefinition {
    /// Check if this unit can move in the German Panzer Movement phase
    pub fn can_move_in_panzer_phase(&self) -> bool {
        self.side == Side::German && self.unit_type == UnitType::Panzer
    }

    /// Get the combat strength for the given unit strength level
    pub fn get_combat_strength(&self, strength: &UnitStrength) -> i32 {
        match strength {
            UnitStrength::Full => self.full_strength,
            UnitStrength::Half => self.half_strength,
            UnitStrength::Eliminated => 0,
        }
    }

    /// Calculate the strength loss when reducing from full to half
    pub fn loss_full_to_half(&self) -> i32 {
        self.full_strength - self.half_strength
    }

    /// Calculate the strength loss when reducing from half to eliminated
    pub fn loss_half_to_eliminated(&self) -> i32 {
        self.half_strength
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UnitStrength {
    Full,
    Half,
    Eliminated,
}

/// Runtime unit state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitState {
    pub id: String,
    pub position: Option<[i32; 2]>, // [q, r] or null if eliminated
    pub strength: UnitStrength,
}

impl UnitState {
    pub fn new(id: String, position: Option<Hex>, strength: UnitStrength) -> Self {
        UnitState {
            id,
            position: position.map(|h| [h.q, h.r]),
            strength,
        }
    }

    pub fn hex(&self) -> Option<Hex> {
        self.position.map(|[q, r]| Hex::new(q, r))
    }

    /// Apply a step loss to this unit
    pub fn take_loss(&mut self) {
        self.strength = match self.strength {
            UnitStrength::Full => UnitStrength::Half,
            UnitStrength::Half => UnitStrength::Eliminated,
            UnitStrength::Eliminated => UnitStrength::Eliminated,
        };

        if self.strength == UnitStrength::Eliminated {
            self.position = None;
        }
    }

    /// Restore unit by one step (for replacements)
    pub fn restore(&mut self) {
        self.strength = match self.strength {
            UnitStrength::Eliminated => UnitStrength::Half,
            UnitStrength::Half => UnitStrength::Full,
            UnitStrength::Full => UnitStrength::Full,
        };
    }

    /// Move unit to a new position
    pub fn move_to(&mut self, hex: Hex) {
        if self.strength != UnitStrength::Eliminated {
            self.position = Some([hex.q, hex.r]);
        }
    }
}

/// Collection of all unit definitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Units {
    pub units: Vec<UnitDefinition>,
}

impl Units {
    /// Load units from JSON
    pub fn load_from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Get a unit definition by ID
    pub fn get(&self, id: &str) -> Option<&UnitDefinition> {
        self.units.iter().find(|u| u.id == id)
    }

    /// Get all unit IDs for a given side
    pub fn get_side_unit_ids(&self, side: Side) -> Vec<String> {
        self.units
            .iter()
            .filter(|u| u.side == side)
            .map(|u| u.id.clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_definition() {
        let unit = UnitDefinition {
            id: "XLVII".to_string(),
            side: Side::German,
            unit_type: UnitType::Panzer,
            full_strength: 9,
            half_strength: 4,
            movement: 6,
            available_turn: None,
        };

        assert!(unit.can_move_in_panzer_phase());
        assert_eq!(unit.get_combat_strength(&UnitStrength::Full), 9);
        assert_eq!(unit.get_combat_strength(&UnitStrength::Half), 4);
        assert_eq!(unit.loss_full_to_half(), 5);
        assert_eq!(unit.loss_half_to_eliminated(), 4);
    }

    #[test]
    fn test_unit_state_loss() {
        let mut state = UnitState::new(
            "5".to_string(),
            Some(Hex::new(5, 3)),
            UnitStrength::Full,
        );

        state.take_loss();
        assert_eq!(state.strength, UnitStrength::Half);
        assert_eq!(state.hex(), Some(Hex::new(5, 3)));

        state.take_loss();
        assert_eq!(state.strength, UnitStrength::Eliminated);
        assert_eq!(state.hex(), None);
    }

    #[test]
    fn test_unit_state_restore() {
        let mut state = UnitState::new(
            "5".to_string(),
            None,
            UnitStrength::Eliminated,
        );

        state.restore();
        assert_eq!(state.strength, UnitStrength::Half);

        state.restore();
        assert_eq!(state.strength, UnitStrength::Full);
    }
}
