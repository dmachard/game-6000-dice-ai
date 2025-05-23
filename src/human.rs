use crate::score::{calculate_score, roll_dice};
use std::io;

const NUM_DICE: usize = 6;

pub fn human_turn() -> u32 {
    println!("--- Human Turn ---");
    let mut dice = NUM_DICE;
    let mut turn_score = 0;
    let mut turn_count = 1;

    loop {
        println!("\n\t--- TURN: {} SCORE: {} ---", turn_count, turn_score);

        let roll = roll_dice(dice);
        println!("\t\tYou rolled: {:?}", roll);

        let (score, remaining_dice) = calculate_score(&roll);
        println!("\t\tScore: +{}", score);

        if score == 0 {
            println!("\t\tNo points! You lose the turn.\n");
            return 0;
        }

        turn_score += score;
        turn_count += 1;

        if remaining_dice == 0 {
            println!("\t\tPlay again!");
            continue;
        }

        println!("\t\tRemaining dice: {}", remaining_dice);
        println!("\t\t(T)ake points or (R)oll again?");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        if choice.trim().eq_ignore_ascii_case("T") {
            println!("\t\tYou banked your points.\n");
            break;
        } else {
            dice = remaining_dice as usize;
        }
    }

    turn_score
}
