use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;
use crate::external_models::game::Game;
use crate::external_models::game_config::GameConfig;
use crate::internal_models::base::Base;
use crate::internal_models::board_action::BoardAction;
use crate::external_models::game_state::GameState as ExternalGameState;

pub struct GameState {
    pub bases: HashMap<u32,Arc<Base>>,
    pub attacks: HashMap<Uuid,Arc<BoardAction>>,
    pub config: GameConfig,
    pub game: Game,
}

impl GameState {
    pub fn from_external(game_state: ExternalGameState) -> Self {
        // create Hashmap for bases
        let mut bases: HashMap<u32, Arc<Base>> = HashMap::with_capacity(1_000);
        game_state.bases.iter().for_each(|base| {
            bases.insert(base.uid, Base::from_external(base));
        });

        // create Hashmap for attacks
        let mut attacks: HashMap<Uuid, Arc<BoardAction>> = HashMap::with_capacity(50_000);
        game_state.actions.iter().for_each(|attack| {
            attacks.insert(attack.uuid, BoardAction::from_external(attack, &bases));
        });

        // return gameState
        GameState {
            bases,
            attacks,
            config: game_state.config,
            game: game_state.game,
        }
    }
}