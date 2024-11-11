use crate::external_models::{game_state::GameState, player_action::PlayerAction};

pub fn decide(game_state: GameState) -> Vec<PlayerAction> {
    // TODO: place your player logic here.
    vec![PlayerAction {
        src: 0,
        dest: 0,
        amount: 0,
    }]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decide_test() {
        let want = vec![PlayerAction::default()];

        let result = decide(GameState::default());

        assert!(want == result)
    }
}
