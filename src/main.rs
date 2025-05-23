use colored::*;
use dice6000::game::start_game;
use std::env;

fn main() {
    if env::var("OPENAI_API_KEY").is_err() {
        println!(
            "{}",
            "Error: The environment variable OPENAI_API_KEY is not set."
                .bold()
                .red()
        );
        println!(
            "{}",
            "Please set it before running the game (e.g., export OPENAI_API_KEY=your_key)."
                .bold()
                .yellow()
        );
        return;
    }

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <command>", args[0]);
        println!("Commands:");
        println!("  rules   - Display the game rules");
        println!("  start   - Start the game");
        return;
    }

    match args[1].as_str() {
        "rules" => {
            display_rules();
        }
        "start" => {
            println!("Welcome to the 6000 Dice Game!");
            start_game();
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
