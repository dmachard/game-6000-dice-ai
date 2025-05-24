use crate::score::{calculate_score, roll_dice};
use colored::*;
use std::io::{self, Write};

const NUM_DICE: usize = 6;

pub fn human_turn() -> u32 {
    println!("{}", "Human Turn".bold().blue());
    let mut dice = NUM_DICE;
    let mut turn_score = 0;
    let mut roll_count = 1;

    loop {
        println!("{} {}", "\n\tRoll number:".bold().green(), roll_count);

        let roll = roll_dice(dice);
        println!("{} {:?}", "\tYou rolled:".bold().green(), roll);

        let (score, remaining_dice) = calculate_score(&roll);
        println!("{} +{}", "\tScore:".bold().cyan(), score);

        if score == 0 {
            println!(
                "{}",
                "\tNo points! You lose the turn. Press Enter to continue...\n"
                    .bold()
                    .red()
            );
            std::io::stdin().read_line(&mut String::new()).unwrap();
            return 0;
        }

        turn_score += score;
        roll_count += 1;

        if remaining_dice == 0 {
            println!(
                "{}",
                "\tAll dice scored! You get to roll again!".bold().magenta()
            );
            continue;
        }

        println!("{} {}", "\tRemaining dice:".bold().blue(), remaining_dice);
        print!("{}", "\t(T)ake points or (R)oll again? ".bold().white());
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        if choice.trim().eq_ignore_ascii_case("T") {
            break;
        } else {
            dice = remaining_dice as usize;
        }
    }

    turn_score
}
