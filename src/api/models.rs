use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::game;

#[derive(Deserialize, Serialize)]
pub struct StatusQuery {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub id: String,
    pub players: Vec<Player>,
    pub current_player_index: usize,
    pub dice: Vec<u8>,
    pub dice_count: usize,
    pub game_over: bool,
    pub winner: Option<String>,
    pub turn_number: u32,
   // pub turn_terminated: bool,
    pub rerollable_dice: Vec<usize>,
    pub turn_end_reason: Option<String>, // None =  in progress, Some("busted"), Some("banked"), Some("win"), etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub score: u32,
    pub is_human: bool,
    pub ai_type: Option<String>,
    pub turn_score: u32,
    pub roll_score: u32
}

#[derive(Debug, Deserialize)]
pub struct CreateGameRequest {
    pub language: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RollDiceRequest {
}

#[derive(Debug, Serialize)]
pub struct RollDiceResponse {
    pub dice: Vec<u8>,
    pub roll_score: u32,
    pub turn_score: u32,
    pub busted: bool,
    pub rerollable_dice: Vec<usize>,
    pub remaining_dice_values: Vec<u8>,
    pub ai_decision: Option<String>,
    pub ai_explanation: Option<String>,
}


#[derive(Debug, Serialize)]
pub struct StatusFullResponse {
    pub success: bool,
    pub game_state: Option<GameState>,
    pub ai_decision: Option<String>,
    pub ai_explanation: Option<String>,
    
}

#[derive(Debug, Serialize)]
pub struct GameResponse {
    pub success: bool,
    pub game_state: Option<GameState>,
}

#[derive(Debug, Serialize)]
pub struct ScoresResponse {
    pub players: Vec<PlayerScore>,
    pub current_player: String,
    pub turn_number: u32,
}

#[derive(Debug, Serialize)]
pub struct PlayerScore {
    pub name: String,
    pub total_score: u32,
    pub turn_score: u32,
    pub roll_score: u32,
    pub is_current: bool,
}

impl GameState {
    pub fn new(id: String, has_openai: bool, has_anthropic: bool, config: &Config) -> Self {
        let game_players = game::setup_players(has_openai, has_anthropic, config);

        Self {
            id,
            players: game_players
                .into_iter()
                .map(
                    |p| Player {
                        name: p.name,
                        score: p.score,
                        is_human: p.is_human,
                        ai_type: p.ai_type,
                        turn_score: p.turn_score,
                        roll_score: p.roll_score,
                    },
                )
                .collect(),
            current_player_index: 0,
            dice: Vec::new(),
            game_over: false,
            turn_terminated: false,
            winner: None,
            turn_number: 1,
            dice_count: 6,
            rerollable_dice: Vec::new(),
        }
    }

    pub fn current_player(&self) -> &Player {
        &self.players[self.current_player_index]
    }

    pub fn current_player_mut(&mut self) -> &mut Player {
        &mut self.players[self.current_player_index]
    }

    pub fn next_player(&mut self) {
        self.current_player_index = (self.current_player_index + 1) % self.players.len();
        self.turn_number += 1;
        self.dice_count = 6;
    }

    pub fn check_winner(&mut self) {
        if let Some(player) = self.players.iter().find(|p| p.score >= 6000) {
            self.game_over = true;
            self.winner = Some(player.name.clone());
        }
    }
}