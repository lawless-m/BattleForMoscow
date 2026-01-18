use serde::{Deserialize, Serialize};
use std::cmp::max;

/// Axial coordinates for hex grid
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Hex {
    pub q: i32,
    pub r: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    NE,
    E,
    SE,
    SW,
    W,
    NW,
}

impl Direction {
    pub fn all() -> [Direction; 6] {
        [
            Direction::NE,
            Direction::E,
            Direction::SE,
            Direction::SW,
            Direction::W,
            Direction::NW,
        ]
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            Direction::NE => "NE",
            Direction::E => "E",
            Direction::SE => "SE",
            Direction::SW => "SW",
            Direction::W => "W",
            Direction::NW => "NW",
        }
    }

    pub fn from_string(s: &str) -> Option<Direction> {
        match s {
            "NE" => Some(Direction::NE),
            "E" => Some(Direction::E),
            "SE" => Some(Direction::SE),
            "SW" => Some(Direction::SW),
            "W" => Some(Direction::W),
            "NW" => Some(Direction::NW),
            _ => None,
        }
    }
}

impl Hex {
    pub fn new(q: i32, r: i32) -> Self {
        Hex { q, r }
    }

    /// Calculate distance between two hexes using axial coordinates
    /// distance = max(abs(a.q - b.q), abs(a.r - b.r), abs((a.q + a.r) - (b.q + b.r)))
    pub fn distance(&self, other: &Hex) -> i32 {
        let dq = (self.q - other.q).abs();
        let dr = (self.r - other.r).abs();
        let ds = ((self.q + self.r) - (other.q + other.r)).abs();
        max(max(dq, dr), ds)
    }

    /// Get the neighbor in a specific direction
    pub fn neighbor(&self, direction: Direction) -> Hex {
        match direction {
            Direction::NE => Hex::new(self.q + 1, self.r - 1),
            Direction::E => Hex::new(self.q + 1, self.r),
            Direction::SE => Hex::new(self.q, self.r + 1),
            Direction::SW => Hex::new(self.q - 1, self.r + 1),
            Direction::W => Hex::new(self.q - 1, self.r),
            Direction::NW => Hex::new(self.q, self.r - 1),
        }
    }

    /// Get all six neighbors of this hex
    pub fn neighbors(&self) -> Vec<Hex> {
        Direction::all()
            .iter()
            .map(|&dir| self.neighbor(dir))
            .collect()
    }

    /// Check if two hexes are adjacent (distance of 1)
    pub fn is_adjacent(&self, other: &Hex) -> bool {
        self.distance(other) == 1
    }

    /// Get the direction from this hex to an adjacent hex
    /// Returns None if the hexes are not adjacent
    pub fn direction_to(&self, other: &Hex) -> Option<Direction> {
        if !self.is_adjacent(other) {
            return None;
        }

        for dir in Direction::all() {
            if self.neighbor(dir) == *other {
                return Some(dir);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let a = Hex::new(0, 0);
        let b = Hex::new(1, 0);
        assert_eq!(a.distance(&b), 1);

        let c = Hex::new(3, 3);
        assert_eq!(a.distance(&c), 6);
    }

    #[test]
    fn test_neighbors() {
        let hex = Hex::new(0, 0);
        let neighbors = hex.neighbors();
        assert_eq!(neighbors.len(), 6);

        assert!(neighbors.contains(&Hex::new(1, -1))); // NE
        assert!(neighbors.contains(&Hex::new(1, 0)));  // E
        assert!(neighbors.contains(&Hex::new(0, 1)));  // SE
        assert!(neighbors.contains(&Hex::new(-1, 1))); // SW
        assert!(neighbors.contains(&Hex::new(-1, 0))); // W
        assert!(neighbors.contains(&Hex::new(0, -1))); // NW
    }

    #[test]
    fn test_adjacency() {
        let a = Hex::new(5, 3);
        let b = Hex::new(6, 3);
        assert!(a.is_adjacent(&b));

        let c = Hex::new(10, 10);
        assert!(!a.is_adjacent(&c));
    }

    #[test]
    fn test_direction_to() {
        let a = Hex::new(5, 3);
        let b = Hex::new(6, 3);
        assert_eq!(a.direction_to(&b), Some(Direction::E));

        let c = Hex::new(5, 4);
        assert_eq!(a.direction_to(&c), Some(Direction::SE));

        let d = Hex::new(10, 10);
        assert_eq!(a.direction_to(&d), None);
    }
}
