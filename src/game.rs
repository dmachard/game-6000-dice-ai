use crate::ai::ai_turn;
use crate::human::human_turn;
use colored::Colorize;
use std::process::Command;

const WINNING_SCORE: u32 = 6000;

pub struct Player {
    pub name: String,
    pub score: u32,
    pub is_human: bool,
    pub ai_type: Option<String>, // "openai" ou "anthropic"
}

pub fn start_game(ai_player_count: u8, has_openai: bool, has_anthropic: bool) {
    let mut players = setup_players(ai_player_count, has_openai, has_anthropic);

    // let mut human_score = 0;
    // let mut ai_score = 0;
    let mut turn_number = 1;

    // let mut scores = [("Human", 0), ("AI", 0)];

    loop {
        clear_screen();
        print_summary(turn_number, &players);

        for i in 0..players.len() {
            println!(
                "\n{}",
                format!("--- {}'s Turn ---", players[i].name).bold().cyan()
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

                ai_turn(players[i].score, &other_scores, &players[i].ai_type)
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

        // // Human
        // human_score += human_turn();
        // scores[0].1 = human_score;
        // if human_score >= WINNING_SCORE {
        //     println!("{}", "You win!".bold().red().on_white());
        //     break;
        // }

        // clear_screen();
        // print_summary(turn_number, human_score, ai_score);

        // // AI
        // ai_score += ai_turn(ai_score, human_score);
        // scores[1].1 = ai_score;
        // if ai_score >= WINNING_SCORE {
        //     println!("{}", "AI wins!".bold().red().on_white());
        //     break;
        // }

        // println!(
        //     "{}",
        //     "AI has finished its turn. Press Enter to continue..."
        //         .bold()
        //         .magenta()
        // );
        // std::io::stdin().read_line(&mut String::new()).unwrap();

        // turn_number += 1;
    }
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

fn setup_players(player_count: u8, has_openai: bool, has_anthropic: bool) -> Vec<Player> {
    let mut players = Vec::new();

    // add human player
    players.push(Player {
        name: "Human".to_string(),
        score: 0,
        is_human: true,
        ai_type: None,
    });

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
            // default
            players.push(Player {
                name: "AI".to_string(),
                score: 0,
                is_human: false,
                ai_type: if has_openai {
                    Some("openai".to_string())
                } else {
                    Some("anthropic".to_string())
                },
            });
        }
    }

    players
}
