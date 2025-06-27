use crate::ai::ai_turn;
use crate::computer::computer_turn;
use crate::config::Config;
use crate::human::human_turn;
use crate::api::models::{Player};

use colored::Colorize;
use std::process::Command;

const WINNING_SCORE: u32 = 6000;


pub fn start_game(has_openai: bool, has_anthropic: bool, config: &Config) {
    let mut players = setup_players(has_openai, has_anthropic, config);
    let mut turn_number = 1;

    loop {
        clear_screen();
        print_summary(turn_number, &players);

        for i in 0..players.len() {
            println!(
                "{}",
                format!("--- {} is playing ---", players[i].name)
                    .bold()
                    .cyan()
            );

            let turn_score = if players[i].is_human {
                human_turn()
            } else {
                let other_scores: Vec<u32> = players
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| *j != i)
                    .map(|(_, p)| p.score)
                    .collect();

                if players[i].ai_type == Some("computer".to_string()) {
                    computer_turn(players[i].score, &other_scores, config)
                } else {
                    ai_turn(players[i].score, &other_scores, &players[i].ai_type, config)
                }
            };

            players[i].score += turn_score;

            if players[i].score >= WINNING_SCORE {
                clear_screen();
                print_summary(turn_number, &players);
                println!(
                    "{}",
                    format!("{} wins with {} points!", players[i].name, players[i].score)
                        .bold()
                        .red()
                        .on_white()
                );
                return;
            }

            if !players[i].is_human {
                println!(
                    "{}",
                    format!(
                        "{} has finished its turn. Press Enter to continue...",
                        players[i].name
                    )
                    .bold()
                    .magenta()
                );
                std::io::stdin().read_line(&mut String::new()).unwrap();
            }

            clear_screen();
            print_summary(turn_number, &players);
        }
        turn_number += 1;
    }
}

pub fn setup_players(has_openai: bool, has_anthropic: bool, config: &Config) -> Vec<Player> {
    let mut players = Vec::new();

    // Always add human player
    players.push(Player {
        name: config.game.human_player_name.clone(),
        score: 0,
        is_human: true,
        ai_type: None,
        turn_score: 0,
        roll_score: 0,
    });

    // Always add computer player
    players.push(Player {
        name: config.game.computer_player_name.clone(),
        score: 0,
        is_human: false,
        ai_type: Some("computer".to_string()),
        turn_score: 0,
        roll_score: 0,
    });

    // add ai players
    if has_openai {
        players.push(Player {
            name: "AI (OpenAI)".to_string(),
            score: 0,
            is_human: false,
            ai_type: Some("openai".to_string()),
            turn_score: 0,
            roll_score: 0,
        });
    }

    if has_anthropic {
        players.push(Player {
            name: "AI (Claude)".to_string(),
            score: 0,
            is_human: false,
            ai_type: Some("anthropic".to_string()),
            turn_score: 0,
            roll_score: 0,
        });
    }

    if config.ollama.enabled {
        players.push(Player {
            name: "AI (Ollama)".to_string(),
            score: 0,
            is_human: false,
            ai_type: Some("ollama".to_string()),
            turn_score: 0,
            roll_score: 0,
        });
    }

    players
}

fn clear_screen() {
    Command::new("clear").status().unwrap();
}

fn print_summary(turn_number: u32, players: &[Player]) {
    println!("{}", "6000 Dice Game".bold().blue());
    println!("{}", "==============================".blue());

    let mut summary = format!("Turn: {}", turn_number);
    for player in players {
        summary.push_str(&format!(" | {}: {}", player.name, player.score));
    }

    println!("{}", summary.bold().yellow());
    println!("{}", "==============================\n".blue());
}
