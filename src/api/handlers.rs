use axum::Json;
use axum::{
    extract::{State, Path},
    http::StatusCode,
    Extension,
};
use std::collections::HashMap;
use uuid::Uuid;
use std::sync::Arc;
use std::sync::Mutex;

use crate::config::Config;
use crate::score::{roll_dice, calculate_score};

use crate::api::models::{
    Status,
    CreateGameRequest, GameResponse, GameState,
    RollDiceResponse, RollDiceRequest
};


pub async fn status_handler() -> Json<Status> {
    Json(Status {
        status: "ok".to_string(),
    })
}

pub type GameStore = Arc<Mutex<HashMap<String, GameState>>>;

pub async fn create_game(
    State(store): State<GameStore>,
    Extension(config): Extension<Arc<Config>>,
    Json(_request): Json<CreateGameRequest>,
) -> Result<Json<GameResponse>, StatusCode> {
    let game_id = Uuid::new_v4().to_string();
    let game_state = GameState::new(game_id.clone(), false, false, &config);
    
    // insert game into the store
    let mut games = store.lock().unwrap();
    games.insert(game_id.clone(), game_state.clone());
    
    println!("Game created with ID: {}", game_id);
    Ok(Json(GameResponse {
        success: true,
        message: "Game created with succes".to_string(),
        game_state: Some(game_state),
    }))
}

pub async fn roll_dice_handler(
    Path(game_id): Path<String>,
    State(store): State<GameStore>,
    Extension(config): Extension<Arc<Config>>,
    Json(_request): Json<RollDiceRequest>,
) -> Result<Json<RollDiceResponse>, StatusCode> {
    let mut games = store.lock().unwrap();
    let game_state = games.get_mut(&game_id)
        .ok_or(StatusCode::NOT_FOUND)?;

    if game_state.game_over {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Roll dice
    let dice_to_roll = if game_state.dice_count == 0 { 6 } else { game_state.dice_count };
    let roll = roll_dice(dice_to_roll);
    game_state.dice = roll.clone();

    // Calculate score and indices of rerollable dice (non-scoring dice)
    // rerollable_dice : indices des dés non scorants (peuvent être relancés)
    // remaining_dice_values : valeurs des dés non scorants (pour affichage ou logique avancée)
    let (score, remaining_dice, remaining_dice_values) = calculate_score(&roll);
    // Calcule les indices des dés relançables (non scorants)
    let mut rerollable_dice = Vec::new();
    let mut remaining = remaining_dice_values.clone();
    for (i, &die) in roll.iter().enumerate() {
        if let Some(pos) = remaining.iter().position(|&v| v == die) {
            rerollable_dice.push(i);
            remaining.remove(pos);
        }
    }
    let busted = score == 0;

    if busted {
        game_state.current_player_mut().turn_score = 0;
        game_state.current_player_mut().roll_score = 0;
        game_state.next_player();
        return Ok(Json(RollDiceResponse {
            dice: roll,
            roll_score: 0,
            turn_score: 0,
            can_continue: false,
            busted: true,
            rerollable_dice: rerollable_dice.clone(),
            remaining_dice_values: remaining_dice_values.clone(),
        }));
    }

    // Update player's score
    {
        let current_player = game_state.current_player_mut();
        current_player.roll_score = score;
        current_player.turn_score += score;
    }

    // If all dice scored, allow another roll
    let next_dice_count = if remaining_dice == 0 { 6 } else { remaining_dice as usize };
    game_state.dice_count = next_dice_count;

    // can_continue doit être false si plus de dés à relancer (rerollable_dice est vide)
    let can_continue = !rerollable_dice.is_empty();

    Ok(Json(RollDiceResponse {
        dice: game_state.dice.clone(),
        roll_score: score,
        turn_score: game_state.current_player().turn_score,
        can_continue,
        busted: false,
        rerollable_dice,
        remaining_dice_values,
    }))
}
