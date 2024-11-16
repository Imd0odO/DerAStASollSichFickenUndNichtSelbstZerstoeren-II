use std::collections::HashMap;
use std::sync::{Arc, Weak};
use crate::external_models::progress::Progress;
use uuid::Uuid;
use crate::external_models::base_level::BaseLevel;
use crate::internal_models::base::Base;
use crate::external_models::board_action::BoardAction as ExternalBoardAction;
use crate::external_models::path_config::PathConfig;

#[derive(Debug)]
pub struct BoardAction {
    pub src: Weak<Base>,
    pub dest: Weak<Base>,
    pub amount: u32,
    pub uuid: Uuid,
    pub player: u32,
    pub progress: Progress,

    pub config: Weak<PathConfig>,
}

impl BoardAction {
    pub fn from_external(attack: &ExternalBoardAction, bases: &HashMap<u32, Arc<Base>>, config: &Arc<PathConfig>) -> Arc<Self> {
        // get src and dest base
        let src: &Arc<Base> = bases.get(&attack.src).unwrap();
        let dest: &Arc<Base> = bases.get(&attack.dest).unwrap();

        // create new attack as arc, with weak references to the bases
        let attack: Arc<BoardAction> = Arc::new(Self {
            src: Arc::downgrade(src),
            dest: Arc::downgrade(dest),
            amount: attack.amount,
            uuid: attack.uuid,
            player: attack.player,
            progress: attack.progress,

            config: Arc::downgrade(config)
        });

        // add the attack to the target base
        let mut attacks;
        {
            attacks = dest.incoming_attacks.lock().unwrap().clone();
        }
        attacks.push(Arc::downgrade(&attack));

        // return the attack
        attack
    }

    pub fn value_at_destination_path_only(&self) -> u32 {
        self.value_in_n_ticks_path_only(&self.progress.distance_remaining())
    }

    pub fn value_in_n_ticks_path_only(&self, n: &u32) -> u32 {
        let config: Arc<PathConfig> = self.config.upgrade().unwrap();
        let distance_remaining: u32 = if n > &self.progress.distance_remaining() {self.progress.distance_remaining()} else {*n};
        let distance_with_protection: u32 = if self.progress.traveled > config.grace_period {0} else {config.grace_period - self.progress.traveled};
        let distance_without_protection: u32 = distance_remaining - if self.progress.traveled > config.grace_period {0} else {distance_with_protection};
        self.amount - (distance_without_protection * config.death_rate)
    }
}

impl Clone for BoardAction {
    fn clone(&self) -> Self {
        Self {
            src: self.src.clone(),
            dest: self.dest.clone(),
            amount: self.amount,
            uuid: self.uuid,
            player: self.player,
            progress: self.progress,
            config: self.config.clone(),
        }
    }
}

