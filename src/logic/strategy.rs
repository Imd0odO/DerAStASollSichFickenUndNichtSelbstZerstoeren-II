use std::sync::Arc;
use crate::external_models::{game_state::GameState as ExternalGameState, player_action::PlayerAction};
use crate::internal_models::base::Base;
use crate::internal_models::game_state::GameState;

pub fn decide(game_state: ExternalGameState) -> Vec<PlayerAction> {

    let game_state: GameState = GameState::from_external(game_state);

    // sort bases
    let mut own_bases: Vec<&Arc<Base>> = Vec::new();
    let mut enemy_bases: Vec<&Arc<Base>> = Vec::new();

    game_state.bases.values().for_each(|base| {
        if base.player == game_state.game.player {
            own_bases.push(base)
        }
        else {
            enemy_bases.push(base)
        }
    });

    own_bases.iter().for_each(|own_base| {
        enemy_bases.iter().for_each(|enemy_base| {


       })
    });

    vec![PlayerAction {
        src: 0,
        dest: 0,
        amount: 0,
    }]
}
