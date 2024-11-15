use serde::Deserialize;
use num::integer::{sqrt, Roots};

#[derive(Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Position {
    pub x: i32, // x coordinate
    pub y: i32, // y coordinate
    pub z: i32, // y coordinate
}

impl Default for Position {
    fn default() -> Self {
        Position { x: 0, y: 0, z: 0 }
    }
}

impl Position {
    pub fn distance_to(&self, target_position: &Self) -> u32 {
        (
              (self.x - target_position.x).pow(2)
            + (self.y - target_position.y).pow(2)
            + (self.z - target_position.z).pow(2)
        ).sqrt() as u32
    }
}
