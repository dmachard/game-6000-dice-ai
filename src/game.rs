use crate::ai::ai_turn;
use crate::human::human_turn;

const WINNING_SCORE: u32 = 6000;

pub fn start_game() {
    let mut human_score = 0;
    let mut ai_score = 0;

    let mut scores = vec![("Human", 0), ("AI", 0)];

    loop {
        // Human
        human_score += human_turn();
        scores[0].1 = human_score;
        if human_score >= WINNING_SCORE {
            println!("You win!");
            break;
        }
        print_scores(&scores);

        // AI
        ai_score += ai_turn(ai_score, human_score);
        scores[1].1 = ai_score;
        if ai_score >= WINNING_SCORE {
            println!("AI wins!");
            break;
        }
        print_scores(&scores);
    }
}

fn print_scores(scores: &[(&str, u32)]) {
    println!("SCORES TABLE:");
    for (player, score) in scores {
        println!("- {}: {}", player, score);
    }
    println!();
}
