#![allow(non_snake_case)]
mod logic;
mod external_models;
mod internal_models;

use axum::{
    routing::{get, post},
    Json, Router,
};
use external_models::{game_state::GameState, player_action::PlayerAction};
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    info!("Der AStA hat begonnen zu brennen");

    let app = Router::new()
        .route("/", get(identify))
        .route("/", post(index));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn identify() -> &'static str {
    "DerAStASollSichFickenUndNichtSelbstZerstören-II"
}

async fn index(Json(payload): Json<GameState>) -> Json<Vec<PlayerAction>> {
    Json(logic::strategy::decide(payload))
}
