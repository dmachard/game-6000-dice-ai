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

use axum::extract::Query;

use crate::config::Config;
use crate::score::{roll_dice, calculate_score};
use crate::computer::computer_turn_stateful;

use crate::api::models::{
    CreateGameRequest, GameResponse, GameState, StatusQuery,
    RollDiceRequest, StatusFullResponse
};

const NUM_DICE: usize = 6;

pub type GameStore = Arc<Mutex<HashMap<String, GameState>>>;

pub async fn create_game(
    State(store): State<GameStore>,
    Extension(config): Extension<Arc<Config>>,
    Json(_request): Json<CreateGameRequest>,
) -> Result<Json<GameResponse>, StatusCode> {
    let game_id = Uuid::new_v4().to_string();
    let mut game_state = GameState::new(game_id.clone(), false, false, &config);
    // Ensure the first player can roll at the start
    game_state.dice_count = NUM_DICE;
    game_state.dice.clear(); // No dice rolled yet
    
    // insert game into the store
    let mut games = store.lock().unwrap();
    games.insert(game_id.clone(), game_state.clone());
    
    println!("Game created with ID: {}", game_id);
    Ok(Json(GameResponse {
        success: true,
        game_state: Some(game_state),
    }))
}

pub async fn roll_dice_handler(
    Path(game_id): Path<String>,
    State(store): State<GameStore>,
    Extension(_config): Extension<Arc<Config>>,
    Json(_request): Json<RollDiceRequest>,
) -> Result<Json<StatusFullResponse>, StatusCode> {
    let mut games = store.lock().unwrap();
    let game_state = games.get_mut(&game_id)
        .ok_or(StatusCode::NOT_FOUND)?;

    if game_state.game_over {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Roll dice
    let dice_to_roll = if game_state.dice_count == 0 { NUM_DICE } else { game_state.dice_count };
    let roll = roll_dice(dice_to_roll);
    game_state.dice = roll.clone();

    let (score, remaining_dice, _remaining_dice_values, rerollable_dice) = calculate_score(&roll);
    game_state.rerollable_dice = rerollable_dice;
    let busted = score == 0;

    if busted {
        game_state.current_player_mut().turn_score = 0;
        game_state.current_player_mut().roll_score = 0;
        game_state.turn_terminated = true;

        return Ok(Json(StatusFullResponse {
            success: true,
            game_state: Some(game_state.clone()),
            ai_decision: None,
            ai_explanation: None,
            turn_end_reason: Some("busted".to_string()),
        }));
    }

    {
        let current_player = game_state.current_player_mut();
        current_player.roll_score = score;
        current_player.turn_score += score;
    }

    let next_dice_count = if remaining_dice == 0 { 6 } else { remaining_dice as usize };
    game_state.dice_count = next_dice_count;

    Ok(Json(StatusFullResponse {
        success: true,
        game_state: Some(game_state.clone()),
        ai_decision: None,
        ai_explanation: None,
       // turn_end_reason: None,
    }))
}

pub async fn bank_points_handler(
    Path(game_id): Path<String>,
    State(store): State<GameStore>,
    Extension(_config): Extension<Arc<Config>>,
) -> Result<Json<StatusFullResponse>, StatusCode> {
    let mut games = store.lock().unwrap();
    let game_state = games.get_mut(&game_id)
        .ok_or(StatusCode::NOT_FOUND)?;

    if game_state.game_over {
        return Err(StatusCode::BAD_REQUEST);
    }

    let current_player = game_state.current_player_mut();
    current_player.score += current_player.turn_score;
    current_player.turn_score = 0;
    current_player.roll_score = 0;
    game_state.turn_terminated = true;
    game_state.turn_end_reason = Some("banked".to_string());

    Ok(Json(StatusFullResponse {
        success: true,
        game_state: Some(game_state.clone()),
        ai_decision: None,
        ai_explanation: None,
     //   turn_end_reason: Some("banked".to_string()),
    }))
}

pub async fn game_status_handler(
    Path(game_id): Path<String>,
    State(store): State<GameStore>,
    Extension(config): Extension<Arc<Config>>,
    _query: Query<StatusQuery>,
) -> Result<Json<StatusFullResponse>, StatusCode> {
    let mut games = store.lock().unwrap();
    let game_state = games.get_mut(&game_id)
        .ok_or(StatusCode::NOT_FOUND)?;

    let mut ai_decision = None;
    let mut ai_explanation = None;
    if !game_state.current_player().is_human && !game_state.game_over && !game_state.turn_terminated {
        let summary = computer_turn_stateful(game_state, &config);
        ai_decision = summary.ai_decision;
        ai_explanation = summary.ai_explanation;

        if ai_decision.is_some("T") {
            game_state.turn_end_reason = Some("banked".to_string());
        }
    }

    let (score, _remaining_dice, _remaining_dice_values, rerollable_dice) = calculate_score(&game_state.dice.clone());
    game_state.rerollable_dice = rerollable_dice;
    
   // let busted = score == 0;
    if game_state.turn_score == 0 {
        game_state.turn_end_reason = Some("busted".to_string());
    }
    
    Ok(Json(StatusFullResponse {
        success: true,
        game_state: Some(game_state.clone()),
        ai_decision,
        ai_explanation,
       // turn_end_reason,
    }))
}

pub async fn next_player_handler(
    Path(game_id): Path<String>,
    State(store): State<GameStore>,
    Extension(_config): Extension<Arc<Config>>,
) -> Result<Json<StatusFullResponse>, StatusCode> {
    let mut games = store.lock().unwrap();
    let game_state = games.get_mut(&game_id)
        .ok_or(StatusCode::NOT_FOUND)?;
    if game_state.game_over {
        return Err(StatusCode::BAD_REQUEST);
    }
   // game_state.turn_terminated = false;
    game_state.turn_end_reason = Some("".to_string());
    game_state.next_player();
    game_state.check_winner();
    game_state.dice.clear();
    let ai_decision = None;
    let ai_explanation = None;
    Ok(Json(StatusFullResponse {
        success: true,
        game_state: Some(game_state.clone()),
        ai_decision,
        ai_explanation,
      //  turn_end_reason: None,
    }))
}
