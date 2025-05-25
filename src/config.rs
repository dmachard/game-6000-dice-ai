use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub game: GameConfig,
    pub openai: OpenAIConfig,
    pub anthropic: AnthropicConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GameConfig {
    pub human_player_name: String,
    pub prompt_language: String,
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

    pub fn default() -> Self {
        Config {
            game: GameConfig {
                human_player_name: "Human Player".to_string(),
                prompt_language: "en".to_string(),
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
