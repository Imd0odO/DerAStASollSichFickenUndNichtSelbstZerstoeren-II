use crate::external_models::position::Position;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Base {
    pub position: Position,       // position of the base
    pub uid: u32,                 // uid of the base
    pub player: u32,              // owner of the base
    pub population: u32,          // current population of the base
    pub level: u32,               // level of the base
    pub units_until_upgrade: u32, // number of units required to upgrade
}

impl Default for Base {
    fn default() -> Self {
        Base {
            position: Position::default(),
            uid: 0,
            player: 0,
            population: 0,
            level: 0,
            units_until_upgrade: 0,
        }
    }
}