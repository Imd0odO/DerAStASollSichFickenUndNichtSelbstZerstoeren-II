use crate::external_models::{game_state::GameState as ExternalGameState, player_action::PlayerAction};
use crate::internal_models::game_state::GameState;

pub fn decide(game_state: ExternalGameState) -> Vec<PlayerAction> {

    let game_state: GameState = GameState::from_external(game_state);

    game_state.bases.values().for_each(|base| {


    });

    vec![PlayerAction {
        src: 0,
        dest: 0,
        amount: 0,
    }]
}
