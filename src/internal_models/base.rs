use std::collections::HashMap;
use std::sync::{Arc, Mutex, Weak};
use crate::external_models::position::Position;
use crate::internal_models::board_action::BoardAction;
use crate::external_models::base::Base as ExternalBase;
use crate::external_models::base_level::BaseLevel;
use itertools;
use itertools::Itertools;
use serde::__private::de::Content::F32;

#[derive(Debug)]
pub struct Base {
    pub position: Position,       // position of the base
    pub uid: u32,                 // uid of the base
    pub player: u32,              // owner of the base
    pub population: u32,          // current population of the base
    pub level: u32,               // level of the base
    pub units_until_upgrade: u32, // number of units required to upgrade

    pub incoming_attacks: Mutex<Vec<Weak<BoardAction>>>,
    pub config: Weak<Vec<BaseLevel>>,
}

impl Base {
    pub fn from_external(base: &ExternalBase, config: &Arc<Vec<BaseLevel>>) -> Arc<Self> {
        // return new base with no incoming attacks
        Arc::new(Self {
            position: base.position,
            uid: base.uid,
            player: base.player,
            population: base.population,
            level: base.level,
            units_until_upgrade: base.units_until_upgrade,

            incoming_attacks: Mutex::new(Vec::new()),
            config: Arc::downgrade(config)
        })
    }

    pub fn distance_to_base(&self, base: &Arc<Base>) -> u32 {
        self.position.distance_to(&base.position)
    }

    pub fn search_n_nearest_bases(&self, n: u32, other_bases: &HashMap<u32, Arc<Base>>) -> Vec<Arc<Base>> {
        let mut hits: Vec<Arc<Base>> = vec![];
        let mut distances: HashMap<u32, u32> = HashMap::new();

        other_bases.iter().for_each(|(_, base)| {
            // fill the array until n
            if hits.len() < n as usize {
                hits.push(base.clone())
            }

            distances.insert(base.uid, self.distance_to_base(&base));

            // todo: make performant later
            hits.sort_by(|a, b| {distances.get(&a.uid).unwrap().cmp(distances.get(&b.uid).unwrap())});

            if hits.len() > n as usize {
                hits.pop();
            }
        });

        hits
    }

    pub fn damage_from_base(&self, base: &Arc<Base>) -> u32 {
        let mut incoming_attacks: Vec<BoardAction> = self.incoming_attacks.lock().unwrap().iter().filter(|attack| {
            let attack: Arc<BoardAction> = attack.upgrade().unwrap();
            let origin: Arc<Base> = attack.src.upgrade().unwrap();
            origin.uid == base.uid && attack.player != self.player
        }).map(|attack| {(*attack.upgrade().unwrap()).clone()}).collect();

        /*

        let mut outgoing_attacks: Vec<BoardAction> = base.incoming_attacks.lock().unwrap().iter().filter(|attack| {
            let attack: Arc<BoardAction> = attack.upgrade().unwrap();
            let origin: Arc<Base> = attack.src.upgrade().unwrap();
            origin.uid == self.uid && attack.player != self.player
        }).map(|attack| {(*attack.upgrade().unwrap()).clone()}).collect();

        incoming_attacks.iter_mut().for_each(|outgoing_attack| {
            outgoing_attacks.iter_mut().for_each(|incoming_attack| {
                let ticks_until_collision: u32 = if outgoing_attack.progress.traveled > incoming_attack.progress.traveled {outgoing_attack.progress.traveled - incoming_attack.progress.traveled} else {incoming_attack.progress.traveled - outgoing_attack.progress.traveled} / 2;

                let own_value_at_collision: u32 = outgoing_attack.value_in_n_ticks_path_only(&ticks_until_collision);
                let opponent_value_at_collision: u32 = incoming_attack.value_in_n_ticks_path_only(&ticks_until_collision);

                if opponent_value_at_collision == own_value_at_collision {
                    outgoing_attack.amount = 0;
                    incoming_attack.amount = 0;
                }
                else if opponent_value_at_collision > own_value_at_collision {
                    outgoing_attack.amount = 0;
                    incoming_attack.amount -= opponent_value_at_collision - own_value_at_collision;
                }
                else if opponent_value_at_collision < own_value_at_collision {
                    incoming_attack.amount = 0;
                    outgoing_attack.amount -= own_value_at_collision - opponent_value_at_collision
                }
            })
        });
        */

        incoming_attacks.iter().map(|attack| attack.value_at_destination_path_only()).sum()
    }

    pub fn will_die(&self) -> bool {
        self.units_in_n_ticks(1_000_000).is_none()
    }

    pub fn units_in_n_ticks(&self, n: u32) -> Option<u32> {
        let mut hitpoints: i64 = self.population as i64;
        let attacks = self.incoming_attacks.lock().unwrap();
        let last_attack: Option<&Weak<BoardAction>> = attacks.iter().filter(|attack| attack.upgrade().unwrap().progress.distance_remaining() < n).max_by(|attack_a, attack_b| attack_a.upgrade().unwrap().progress.distance_remaining().cmp(&attack_b.upgrade().unwrap().progress.distance_remaining()));
        let last_attack_hit_time: u32 = if last_attack.is_none() {0} else {last_attack.unwrap().upgrade().unwrap().progress.distance_remaining()};

        hitpoints += (last_attack_hit_time * self.config.upgrade().unwrap()[self.level as usize].spawn_rate) as i64;

        self.incoming_attacks.lock().unwrap().iter().map(|attack| attack.upgrade().unwrap().src.upgrade().unwrap()).unique_by(|base| base.uid).for_each(|base| {
            if hitpoints >= 0 {
                hitpoints -= self.damage_from_base(&base) as i64;
            }
        });

        if hitpoints < 0 {
            return None;
        }
        Some(hitpoints as u32)
    }

    pub fn required_to_kill_in_n_ticks(&self, base: &Arc<Base>, n: u32) -> u32 {
        let units_in_n_ticks: Option<u32> = base.units_in_n_ticks(n);
        let attack_resistance: u32 = self.damage_from_base(&base);
        if units_in_n_ticks.is_some() {units_in_n_ticks.unwrap() + attack_resistance} else {0}
    }

    pub fn required_to_kill_other_base(&self, base: &Arc<Base>) -> u32 {
        self.required_to_kill_in_n_ticks(base, self.distance_to_base(&base))
    }
}