use crate::ai::ai_turn;
use crate::human::human_turn;
use colored::Colorize;
use std::process::Command;

const WINNING_SCORE: u32 = 6000;

pub fn start_game() {
    let mut human_score = 0;
    let mut ai_score = 0;
    let mut turn_number = 1;

    let mut scores = vec![("Human", 0), ("AI", 0)];

    loop {
        clear_screen();
        print_summary(turn_number, human_score, ai_score);

        // Human
        human_score += human_turn();
        scores[0].1 = human_score;
        if human_score >= WINNING_SCORE {
            println!("{}", "You win!".bold().red().on_white());
            break;
        }

        clear_screen();
        print_summary(turn_number, human_score, ai_score);

        // AI
        ai_score += ai_turn(ai_score, human_score);
        scores[1].1 = ai_score;
        if ai_score >= WINNING_SCORE {
            println!("{}", "AI wins!".bold().red().on_white());
            break;
        }

        println!(
            "{}",
            "AI has finished its turn. Press Enter to continue..."
                .bold()
                .magenta()
        );
        std::io::stdin().read_line(&mut String::new()).unwrap();

        turn_number += 1;
    }
}

fn clear_screen() {
    Command::new("clear").status().unwrap();
}

fn print_summary(turn_number: u32, human_score: u32, ai_score: u32) {
    println!("{}", "6000 Dice Game".bold().blue());
    println!("{}", "==============================".blue());
    println!(
        "{}",
        format!(
            "Turn: {} | Human Score: {} | AI Score: {}",
            turn_number, human_score, ai_score
        )
        .bold()
        .yellow()
    );
    println!("{}", "==============================\n".blue());
}
