use std::cmp::max;
use std::collections::HashMap;
use std::sync::Arc;
use crate::external_models::progress::Progress;
use uuid::Uuid;
use crate::internal_models::base::Base;
use crate::external_models::board_action::BoardAction as EAction;
use crate::external_models::game_config::GameConfig;

#[derive(Debug)]
pub struct BoardAction {
    pub src: Arc<Base>,             // uid of source base
    pub dest: Arc<Base>,            // uid of destination base
    pub amount: u32,                // number of bits moved
    pub uuid: Uuid,                 // uuid of the action
    pub player: u32,                // id of the player who took the action
    pub progress: Progress,         // progress off the action
}

impl EAction {
    pub fn convert(&self, base_lookup: &HashMap<u32, Arc<Base>>) -> () {

        if !base_lookup.contains_key(&self.src) || !base_lookup.contains_key(&self.dest) {return;}

        let action: Arc<BoardAction> = Arc::new(BoardAction {
            src: base_lookup.get(&self.src).unwrap().clone(),
            dest: base_lookup.get(&self.dest).unwrap().clone(),
            amount: self.amount,
            uuid: self.uuid,
            player: self.player,
            progress: self.progress,
        });

        action.src.incoming_attacks.lock().unwrap().push(action.clone());
    }

    pub fn value_at_arrival(&self, game_config: &GameConfig) -> u32 {
        let distance_with_damage: u32 = self.progress.distance_remaining() - max(game_config.paths.grace_period - self.progress.traveled, 0);

        let defence: u32 =  todo!();

        let damage: u32 = self.amount - distance_with_damage * game_config.paths.death_rate;
        max(damage, 0)
    }
}

