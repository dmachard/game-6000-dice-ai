use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub game: GameConfig,
    pub openai: OpenAIConfig,
    pub anthropic: AnthropicConfig,
    pub ollama: OllamaConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GameConfig {
    pub human_player_name: String,
    pub computer_player_name: String,
    pub computer_strategy: String,
    pub ai_output_language: String,
    pub ai_personality: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OpenAIConfig {
    pub url: String,
    pub model: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AnthropicConfig {
    pub url: String,
    pub model: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OllamaConfig {
    pub enabled: bool,
    pub url: String,
    pub model: String,
    pub timeout: Option<u64>, // Optional timeout in seconds
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
                ai_output_language: "en".to_string(),
                ai_personality: "default".to_string(),
            },
            openai: OpenAIConfig {
                url: "https://api.openai.com/v1/chat/completions".to_string(),
                model: "gpt-4".to_string(),
            },
            anthropic: AnthropicConfig {
                url: "https://api.anthropic.com/v1/messages".to_string(),
                model: "claude-sonnet-4-20250514".to_string(),
            },
            ollama: OllamaConfig {
                enabled: false,
                url: "http://localhost:11434/api/chat".to_string(),
                model: "llama3.1:8b".to_string(),
                timeout: Some(120), // Optional timeout in seconds
            },
        }
    }
}
