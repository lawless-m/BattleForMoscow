use crate::hex::{Direction, Hex};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Terrain {
    Clear,
    Forest,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct City {
    pub name: String,
    pub is_moscow: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SetupMarker {
    German,
    Soviet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapHex {
    pub q: i32,
    pub r: i32,
    pub terrain: Terrain,
    pub city: Option<City>,
    pub fortification: bool,
    pub rail: bool,
    pub river_edges: Vec<String>, // Direction names: "NE", "E", etc.
    pub setup: Option<SetupMarker>,
}

impl MapHex {
    pub fn hex(&self) -> Hex {
        Hex::new(self.q, self.r)
    }

    /// Check if this hex has a river on the edge in the given direction
    pub fn has_river_edge(&self, direction: Direction) -> bool {
        self.river_edges.contains(&direction.to_string().to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapBounds {
    pub min_q: i32,
    pub max_q: i32,
    pub min_r: i32,
    pub max_r: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapEdges {
    pub west: String,  // "german_communication"
    pub east: String,  // "soviet_communication"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Map {
    pub hexes: Vec<MapHex>,
    pub map_bounds: MapBounds,
    pub edges: MapEdges,
}

impl Map {
    /// Load map from JSON file
    pub fn load_from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Build a lookup table for quick hex access
    pub fn build_lookup(&self) -> HashMap<Hex, MapHex> {
        self.hexes
            .iter()
            .map(|mh| (mh.hex(), mh.clone()))
            .collect()
    }

    /// Get a hex by coordinates
    pub fn get_hex(&self, hex: &Hex) -> Option<&MapHex> {
        self.hexes.iter().find(|mh| mh.hex() == *hex)
    }

    /// Check if a hex is on the west edge (German communication)
    pub fn is_west_edge(&self, hex: &Hex) -> bool {
        hex.q == self.map_bounds.min_q
    }

    /// Check if a hex is on the east edge (Soviet communication)
    pub fn is_east_edge(&self, hex: &Hex) -> bool {
        hex.q == self.map_bounds.max_q
    }

    /// Check if a hex is within map bounds
    pub fn is_in_bounds(&self, hex: &Hex) -> bool {
        hex.q >= self.map_bounds.min_q
            && hex.q <= self.map_bounds.max_q
            && hex.r >= self.map_bounds.min_r
            && hex.r <= self.map_bounds.max_r
    }

    /// Get movement cost for entering a hex
    /// Clear = 1, Forest = 2 (unless rail movement)
    pub fn movement_cost(&self, hex: &Hex, rail_movement: bool) -> Option<i32> {
        self.get_hex(hex).map(|mh| match mh.terrain {
            Terrain::Clear => 1,
            Terrain::Forest => {
                if rail_movement && mh.rail {
                    1
                } else {
                    2
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_hex_creation() {
        let hex = MapHex {
            q: 5,
            r: 3,
            terrain: Terrain::Clear,
            city: Some(City {
                name: "Moscow".to_string(),
                is_moscow: true,
            }),
            fortification: false,
            rail: true,
            river_edges: vec!["NE".to_string(), "E".to_string()],
            setup: Some(SetupMarker::Soviet),
        };

        assert_eq!(hex.hex(), Hex::new(5, 3));
        assert!(hex.has_river_edge(Direction::NE));
        assert!(hex.has_river_edge(Direction::E));
        assert!(!hex.has_river_edge(Direction::W));
    }

    #[test]
    fn test_movement_cost() {
        let map = Map {
            hexes: vec![
                MapHex {
                    q: 0,
                    r: 0,
                    terrain: Terrain::Clear,
                    city: None,
                    fortification: false,
                    rail: false,
                    river_edges: vec![],
                    setup: None,
                },
                MapHex {
                    q: 1,
                    r: 0,
                    terrain: Terrain::Forest,
                    city: None,
                    fortification: false,
                    rail: false,
                    river_edges: vec![],
                    setup: None,
                },
                MapHex {
                    q: 2,
                    r: 0,
                    terrain: Terrain::Forest,
                    city: None,
                    fortification: false,
                    rail: true,
                    river_edges: vec![],
                    setup: None,
                },
            ],
            map_bounds: MapBounds {
                min_q: 0,
                max_q: 2,
                min_r: 0,
                max_r: 0,
            },
            edges: MapEdges {
                west: "german_communication".to_string(),
                east: "soviet_communication".to_string(),
            },
        };

        assert_eq!(map.movement_cost(&Hex::new(0, 0), false), Some(1));
        assert_eq!(map.movement_cost(&Hex::new(1, 0), false), Some(2));
        assert_eq!(map.movement_cost(&Hex::new(2, 0), false), Some(2));
        assert_eq!(map.movement_cost(&Hex::new(2, 0), true), Some(1));
    }
}
