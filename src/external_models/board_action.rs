use crate::external_models::progress::Progress;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub struct BoardAction {
    pub src: u32,           // uid of source base
    pub dest: u32,          // uid of destination base
    pub amount: u32,        // number of bits moved
    pub uuid: Uuid,         // uuid of the action
    pub player: u32,        // id of the player who took the action
    pub progress: Progress, // progress off the action
}

impl Default for BoardAction {
    fn default() -> Self {
        BoardAction {
            src: 0,
            dest: 0,
            amount: 0,
            uuid: Uuid::default(),
            player: 0,
            progress: Progress::default(),
        }
    }
}