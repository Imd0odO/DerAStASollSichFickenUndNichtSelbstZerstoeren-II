use crate::external_models::position::Position;
use serde::Deserialize;


#[derive(Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Base {
    pub position: Position,       // position of the base
    pub uid: u32,                 // uid of the base
    pub name: String,                // name of the base
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
            name: String::from("test"),
            player: 0,
            population: 0,
            level: 0,
            units_until_upgrade: 0,
        }
    }
}

impl Clone for Base {
    fn clone(&self) -> Self {
        Self {
            position: self.position,
            uid: self.uid,
            name: self.name.clone(),
            player: self.player,
            population: self.population,
            level: self.level,
            units_until_upgrade: self.units_until_upgrade,
        }
    }
}