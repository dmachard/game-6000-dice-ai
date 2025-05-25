use colored::*;
use std::env;

use dice6000::config::Config;
use dice6000::game::start_game;

fn main() {
    // Load configuration
    let config = Config::load("config.yaml").unwrap_or_else(|e| {
        println!(
            "Warning: Could not load config.yaml ({}), using defaults",
            e
        );
        Config::default()
    });

    let openai_key = env::var("OPENAI_API_KEY").is_ok();
    let anthropic_key = env::var("ANTHROPIC_API_KEY").is_ok();

    if !openai_key && !anthropic_key {
        println!(
            "{}",
            "Error: Neither OPENAI_API_KEY nor ANTHROPIC_API_KEY environment variables are set."
                .bold()
                .red()
        );
        println!(
            "{}",
            "Please set at least one API key before running the game:"
                .bold()
                .yellow()
        );
        println!("  export OPENAI_API_KEY=your_openai_key");
        println!("  export ANTHROPIC_API_KEY=your_anthropic_key");
        return;
    }

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <command>", args[0]);
        println!("Commands:");
        println!("  rules - Display the game rules");
        println!("  play - Play the game");
        return;
    }

    match args[1].as_str() {
        "rules" => {
            display_rules();
        }
        "play" => {
            println!("Welcome to the 6000 Dice Game!");

            // number of playser
            let ai_player_count = if openai_key && anthropic_key {
                println!(
                    "{}",
                    "Both API keys detected - Starting game with 1 human + 2 AI players!"
                        .bold()
                        .green()
                );
                3
            } else {
                println!(
                    "{}",
                    "One API key detected - Starting game with 1 human + 1 AI player!"
                        .bold()
                        .blue()
                );
                2
            };

            start_game(ai_player_count, openai_key, anthropic_key, &config);
        }
        _ => {
            println!("Unknown command: {}", args[1]);
            println!("Use 'rules' to display the game rules or 'start' to start the game.");
        }
    }
}

fn display_rules() {
    println!("{}", "SIX THOUSAND DICE GAME RULES".bold().green());
    println!("{}", "============================".green());
    println!();
    println!("OBJECTIVE:");
    println!("  Be the first player to reach or exceed 6,000 points.");
    println!();
    println!("GAMEPLAY:");
    println!("  1. Players take turns rolling six dice");
    println!("  2. After each roll, you must set aside at least one scoring die");
    println!("  3. You may continue rolling with remaining dice or bank your points");
    println!("  4. If you cannot score with a roll, you lose all points from that turn");
    println!();
    println!("SCORING COMBINATIONS:");
    println!("  - Straight (1-2-3-4-5-6): 2000 points, all dice used");
    println!("  - Three pairs: 1500 points, all dice used");
    println!("  - Six of a kind: face value × 1000 points");
    println!("  - Three 1s: 1000 points");
    println!("  - Three of 2-6: face value × 100 points");
    println!("  - Single 1: 100 points each");
    println!("  - Single 5: 50 points each");
}
