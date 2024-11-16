use std::cmp::max;
use std::sync::Arc;
use itertools::Itertools;
use tracing::info;
use rand::random;
use crate::external_models::{game_state::GameState as ExternalGameState, player_action::PlayerAction};
use crate::internal_models::base::Base;
use crate::internal_models::game_state::GameState;
use crate::logic::config::MIN_REQUIREMENTS;

pub fn decide(game_state: ExternalGameState) -> Vec<PlayerAction> {
    info!("Request: Game {}, Tick {}", game_state.game.uid, game_state.game.tick);

    let mut actions: Vec<PlayerAction> = Vec::new();

    let game_state: GameState = GameState::from_external(game_state);

    // sort bases
    let mut own_bases: Vec<&Arc<Base>> = Vec::new();
    let mut enemy_bases: Vec<&Arc<Base>> = Vec::new();

    info!("Sorting bases");

    game_state.bases.values().for_each(|base| {
        if base.player == game_state.game.player {
            own_bases.push(base)
        }
        else {
            enemy_bases.push(base)
        }
    });

    info!("Evaluating bases");

    own_bases.iter().for_each(|own_base| {
        info!("Evaluating base {}", own_base.uid);
        let mut target: u32 = 999_999_999;
        let mut cost_to_conquer: u32 = 1_000_000_000;
        let mut distance: u32 = 1_000_000_000;

        // check if base is dying within the next 5 ticks
        if own_base.will_die() && own_base.units_in_n_ticks(5).is_none() {
            // save the troops to a nearby base
            let bases = own_base.search_n_nearest_bases(if game_state.bases.len() < 50 {game_state.bases.len() as u32} else {15}, &game_state.bases);
            let nearest_bases: Vec<&Arc<Base>> = bases.iter().filter(|base| base.player == game_state.game.player).collect();

            // guarantied death
            if nearest_bases.len() == 0 {
                return
            }

            // send troops to nearest
            let dest_base = nearest_bases.iter().min_by(|a, b| own_base.distance_to_base(a).cmp(&own_base.distance_to_base(b))).unwrap();

            info!("Loosing Base {}", own_base.uid);
            actions.push(PlayerAction {
                src: own_base.uid,
                dest: dest_base.uid,
                amount: if own_base.population <= 1 {0} else {own_base.population - 1}
            })
        }

        // check if we want to attack
        if own_base.population > MIN_REQUIREMENTS[own_base.level as usize].MIN_UNITS_FOR_ATTACK {
            enemy_bases.iter().for_each(|enemy_base| {
                let attack_cost: u32 = own_base.required_to_kill_other_base(enemy_base, &game_state.config.paths);
                let d = own_base.distance_to_base(enemy_base);
                if attack_cost < cost_to_conquer && own_base.population > attack_cost {
                    if own_base.population - attack_cost > MIN_REQUIREMENTS[own_base.level as usize].MIN_UNITS_AFTER_ATTACK {
                        cost_to_conquer = attack_cost;
                        target = enemy_base.uid;
                        distance = d;
                    }
                    else if own_base.population - attack_cost == cost_to_conquer && d < distance {
                        cost_to_conquer = attack_cost;
                        target = enemy_base.uid;
                        distance = d;
                    }
                }
            });

            // attack if match found
            if target != 999_999_999 {
                info!("Attacking Base {}", target);
                actions.push(PlayerAction {
                    src: own_base.uid,
                    dest: target,
                    amount: cost_to_conquer + 1,
                });
                return
            }
        }

        // check if we want to upgrade
        if own_base.population > MIN_REQUIREMENTS[own_base.level as usize].MIN_UNITS_FOR_UPGRADE && own_base.level < 14 {
            info!("Upgrading Base {}", own_base.uid);
            actions.push(PlayerAction {
                src: own_base.uid,
                dest: own_base.uid,
                amount: own_base.config.upgrade().unwrap()[own_base.level as usize].spawn_rate,
            });

            return
        }

        // check if we want to distribute
        if own_base.population > MIN_REQUIREMENTS[own_base.level as usize].MIN_UNITS_FOR_DISTRIBUTION && random() && random() && random() && random() {
            let bases = own_base.search_n_nearest_bases(if game_state.bases.len() < 50 { game_state.bases.len() as u32 } else { 15 }, &game_state.bases);
            let nearest_bases: Vec<&Arc<Base>> = bases.iter().filter(|base| base.player == game_state.game.player).collect();

            if nearest_bases.len() == 0 {
                return
            }

            // send troops to nearest
            let nearest = nearest_bases.iter().filter(|base| { base.population < MIN_REQUIREMENTS[own_base.level as usize].MAX_UNITS_FOR_SUPPLY }).min_by(|a, b| own_base.distance_to_base(a).cmp(&own_base.distance_to_base(b)));
            if let Some(dest_base) = nearest {
                info!("Supplying Base {}", dest_base.uid);
                actions.push(PlayerAction {
                    src: own_base.uid,
                    dest: dest_base.uid,
                    amount: if own_base.population <= 1 { 0 } else { own_base.population - 1 }
                });
                return
            };
        }
        info!("Not doing anything!");
    });

    info!("Success!");

    actions
}
