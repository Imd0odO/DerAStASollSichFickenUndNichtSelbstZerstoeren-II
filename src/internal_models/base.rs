use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use crate::external_models::position::Position;
use crate::internal_models::board_action::BoardAction;
use crate::external_models::base::Base as EBase;
use crate::external_models::board_action::BoardAction as EBoardAction;

#[derive(Debug)]
pub struct Base {
    pub position: Position,       // position of the base
    pub uid: u32,                 // uid of the base
    pub player: u32,              // owner of the base
    pub population: u32,          // current population of the base
    pub level: u32,               // level of the base
    pub units_until_upgrade: u32, // number of units required to upgrade

    pub incoming_attacks: Mutex<Vec<Arc<BoardAction>>>
}

impl Base {
    pub fn from_external(base: &EBase) -> Self {
        Self {
            position: base.position,
            uid: base.uid,
            player: base.player,
            population: base.population,
            level: base.level,
            units_until_upgrade: base.units_until_upgrade,

            incoming_attacks: Mutex::new(Vec::new()),
        }
    }

    pub fn from_game_state(bases: &mut Vec<EBase>, actions: &mut Vec<EBoardAction>) -> HashMap<u32, Arc<Self>> {
        let mut new_bases: HashMap<u32, Arc<Self>> = HashMap::new();

        while let Some(base) = bases.pop() {
            let base: Base = Base::from_external(&base);
            new_bases.insert(base.uid, Arc::new(base));
        }

        while let Some(action) = actions.pop() {
            action.convert(&new_bases);
        }

        new_bases
    }
}