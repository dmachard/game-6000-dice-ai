use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub server: ServerConfig,
    pub game: GameConfig,
    pub openai: OpenAIConfig,
    pub anthropic: AnthropicConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GameConfig {
    pub human_player_name: String,
    pub computer_player_name: String,
    pub computer_strategy: String,
    pub ai_decision_language: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenAIConfig {
    pub url: String,
    pub model: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnthropicConfig {
    pub url: String,
    pub model: String,
}

impl Config {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&contents)?;
        Ok(config)
    }

    pub fn init() -> Self {
        Config {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 3000,
            },
            game: GameConfig {
                human_player_name: "Human".to_string(),
                computer_player_name: "Computer".to_string(),
                computer_strategy: "adaptative".to_string(),
                ai_decision_language: "en".to_string(),
            },
            openai: OpenAIConfig {
                url: "https://api.openai.com/v1/chat/completions".to_string(),
                model: "gpt-4".to_string(),
            },
            anthropic: AnthropicConfig {
                url: "https://api.anthropic.com/v1/messages".to_string(),
                model: "claude-sonnet-4-20250514".to_string(),
            },
        }
    }
}
