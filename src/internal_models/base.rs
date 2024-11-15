use std::collections::HashMap;
use std::sync::{Arc, Mutex, Weak};
use crate::external_models::position::Position;
use crate::internal_models::board_action::BoardAction;
use crate::external_models::base::Base as ExternalBase;
use crate::external_models::base_level::BaseLevel;

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
}