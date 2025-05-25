use crate::ai::ai_turn;
use crate::config::Config;
use crate::human::human_turn;

use colored::Colorize;
use std::process::Command;

const WINNING_SCORE: u32 = 6000;

pub struct Player {
    pub name: String,
    pub score: u32,
    pub is_human: bool,
    pub ai_type: Option<String>, // "openai" or "anthropic"
}

pub fn start_game(ai_player_count: u8, has_openai: bool, has_anthropic: bool, config: &Config) {
    let mut players = setup_players(ai_player_count, has_openai, has_anthropic, config);
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

                ai_turn(players[i].score, &other_scores, &players[i].ai_type, config)
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

pub fn setup_players(
    player_count: u8,
    has_openai: bool,
    has_anthropic: bool,
    config: &Config,
) -> Vec<Player> {
    let mut players = Vec::new();

    // Always add human player
    players.push(Player {
        name: config.game.human_player_name.clone(),
        score: 0,
        is_human: true,
        ai_type: None,
    });

    // If no AI options are enabled, return only the human player
    if !has_openai && !has_anthropic {
        return players;
    }

    // add ai players
    match player_count {
        2 => {
            // 1 human + 1 IA
            if has_openai {
                players.push(Player {
                    name: "AI (OpenAI)".to_string(),
                    score: 0,
                    is_human: false,
                    ai_type: Some("openai".to_string()),
                });
            } else if has_anthropic {
                players.push(Player {
                    name: "AI (Claude)".to_string(),
                    score: 0,
                    is_human: false,
                    ai_type: Some("anthropic".to_string()),
                });
            }
        }
        3 => {
            // 1 human + 2 IA
            players.push(Player {
                name: "AI (OpenAI)".to_string(),
                score: 0,
                is_human: false,
                ai_type: Some("openai".to_string()),
            });
            players.push(Player {
                name: "AI (Claude)".to_string(),
                score: 0,
                is_human: false,
                ai_type: Some("anthropic".to_string()),
            });
        }
        _ => {
            println!(
                "Info: Unsupported player count ({}). Only human player will be added.",
                player_count
            );
            // No AI added
        }
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
