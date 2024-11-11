use std::collections::HashMap;
use std::sync::{Arc, Weak};
use crate::external_models::progress::Progress;
use uuid::Uuid;
use crate::internal_models::base::Base;
use crate::external_models::board_action::BoardAction as ExternalBoardAction;

#[derive(Debug)]
pub struct BoardAction {
    src: Weak<Base>,
    dest: Weak<Base>,
    amount: u32,
    uuid: Uuid,
    player: u32,
    progress: Progress,
}

impl BoardAction {
    pub fn from_external(attack: &ExternalBoardAction, bases: &HashMap<u32, Arc<Base>>) -> Arc<Self> {
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
        });

        // add the attack to the target base
        dest.incoming_attacks.lock().unwrap().push(Arc::downgrade(&attack));

        // return the attack
        return attack;
    }
}

