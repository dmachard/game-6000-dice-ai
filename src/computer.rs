use crate::config::Config;
use crate::score::{calculate_score, roll_dice};
use colored::*;
use crate::api::models::GameState;

const NUM_DICE: usize = 6;
// const WINNING_SCORE: u32 = 6000;

#[derive(Debug, Clone, Copy)]
pub enum AIPersonality {
    Conservative,
    Balanced,
    Aggressive,
}

impl AIPersonality {
    fn min_score(&self) -> u32 {
        match self {
            AIPersonality::Conservative => 200,
            AIPersonality::Balanced => 300,
            AIPersonality::Aggressive => 400,
        }
    }

    fn max_score(&self) -> u32 {
        match self {
            AIPersonality::Conservative => 600,
            AIPersonality::Balanced => 800,
            AIPersonality::Aggressive => 1200,
        }
    }
}

pub fn computer_turn(current_score: u32, other_scores: &[u32], config: &Config) -> u32 {
    let mut dice = NUM_DICE;
    let mut turn_score = 0;
    let mut roll_count = 1;
    let personality = get_ai_personality(config);

    loop {
        println!("{} {}", "\n\tRoll number:".bold().green(), roll_count);
        let roll = roll_dice(dice);
        println!("{} {:?}", "\tComputer rolled:".bold().green(), roll);

        let (score, remaining_dice, remaining_dice_values, _rerollable_dice) = calculate_score(&roll);
        println!("{} +{}", "\tScore:".bold().cyan(), score);

        if score == 0 {
            println!("{}", "\tNo points! Computer loses the turn.\n".bold().red());
            return 0;
        }

        turn_score += score;
        roll_count += 1;

        // Relance obligatoire si tous les dés ont scoré
        if remaining_dice == 0 {
            println!("{}", "\tAll dice scored! Computer gets to roll again!".bold().magenta());
            dice = NUM_DICE;
            continue;
        }

        println!("{} {} - {:?}", "\tRemaining dice:".bold().blue(), remaining_dice, remaining_dice_values);

        // Décision simple mais efficace
        if should_continue(turn_score, remaining_dice, current_score, other_scores, &personality) {
            println!("{}", "\tComputer decides to ROLL AGAIN!".bold().yellow());
            dice = remaining_dice as usize;
        } else {
            println!("{}", "\tComputer decides to TAKE the points!".bold().green());
            break;
        }
    }

    turn_score
}

fn should_continue(
    turn_score: u32,
    remaining_dice: u32,
    current_score: u32,
    other_scores: &[u32],
    personality: &AIPersonality,
) -> bool {
    let opponent_best = other_scores.iter().max().copied().unwrap_or(0);
    let min_score = personality.min_score();
    let max_score = personality.max_score();

    // Jamais continuer avec 1 dé et plus de 300 points
    if remaining_dice == 1 && turn_score >= 300 {
        return false;
    }

    // Toujours continuer si on a peu de points et assez de dés
    if turn_score < 200 && remaining_dice >= 3 {
        return true;
    }

    // Continuer si on n'a pas atteint le minimum et qu'on a des dés sûrs
    if turn_score < min_score && remaining_dice >= 3 {
        return true;
    }

    // Arrêter si on a dépassé le maximum
    if turn_score >= max_score {
        return false;
    }

    // Logique de rattrapage : plus agressif si on est en retard
    if opponent_best > current_score + 1500 {
        return turn_score < max_score && remaining_dice >= 2;
    }

    // Logique de protection : plus conservateur si on mène
    if current_score > opponent_best + 1000 {
        return turn_score < min_score && remaining_dice >= 4;
    }

    // Décision par défaut basée sur les dés restants
    match remaining_dice {
        1 => false,
        2 => turn_score < min_score,
        _ => turn_score < min_score + 100,
    }
}

fn get_ai_personality(config: &Config) -> AIPersonality {
    match config.game.computer_strategy.to_lowercase().as_str() {
        "conservative" => AIPersonality::Conservative,
        "aggressive" => AIPersonality::Aggressive,
        _ => AIPersonality::Balanced,
    }
}

// Version pour l'API web
pub struct ComputerTurnResult {
    pub turn_score: u32,
    pub busted: bool,
    pub rolls: Vec<Vec<u8>>,
    pub ai_decision: Option<String>,
    pub ai_explanation: Option<String>,
}

pub fn computer_turn_stateful(game_state: &mut GameState, config: &Config) -> ComputerTurnResult {
    let dice = if game_state.dice_count == 0 { NUM_DICE } else { game_state.dice_count };
    let current_score = game_state.current_player().score;
    let other_scores: Vec<u32> = game_state.players.iter().enumerate()
        .filter(|(i, _)| *i != game_state.current_player_index)
        .map(|(_, p)| p.score)
        .collect();
    let personality = get_ai_personality(config);
    let mut rolls = Vec::new();

    // Si aucun dé n'a été lancé ce tour-ci, on lance
    let roll = roll_dice(dice);
    rolls.push(roll.clone());
    let (score, remaining_dice, _remaining_dice_values, _rerollable_dice) = calculate_score(&roll);

    if score == 0 {
        // Busted
        game_state.current_player_mut().turn_score = 0;
        game_state.current_player_mut().roll_score = 0;
        game_state.dice = roll;
        game_state.turn_terminated = true;
        return ComputerTurnResult {
            turn_score: 0,
            busted: true,
            rolls,
            ai_decision: Some("BUSTED".to_string()),
            ai_explanation: Some("L'IA a fait 0 point, tour terminé.".to_string()),
        };
    }

    // Mise à jour du score
    let turn_score = game_state.current_player().turn_score + score;
    game_state.dice = roll.clone();
    game_state.current_player_mut().roll_score = score;
    game_state.current_player_mut().turn_score = turn_score;

    // Relance obligatoire si tous les dés sont scorants
    if remaining_dice == 0 {
        game_state.dice_count = NUM_DICE;
        // On ne termine pas le tour, l'IA doit relancer au prochain appel
        return ComputerTurnResult {
            turn_score,
            busted: false,
            rolls,
            ai_decision: Some("RELANCE_OBLIGATOIRE".to_string()),
            ai_explanation: Some("Tous les dés sont scorants, relance obligatoire.".to_string()),
        };
    }

    // Décision IA : continuer ou sécuriser
    if should_continue(turn_score, remaining_dice, current_score, &other_scores, &personality) {
        // L'IA décide de relancer
        game_state.dice_count = remaining_dice as usize;
        // On ne termine pas le tour, l'IA doit relancer au prochain appel
        return ComputerTurnResult {
            turn_score,
            busted: false,
            rolls,
            ai_decision: Some("R".to_string()),
            ai_explanation: Some("L'IA décide de relancer.".to_string()),
        };
    } else {
        // L'IA décide de sécuriser
        game_state.current_player_mut().score += turn_score;
        game_state.current_player_mut().turn_score = 0;
        game_state.current_player_mut().roll_score = 0;
        game_state.turn_terminated = true;
        return ComputerTurnResult {
            turn_score,
            busted: false,
            rolls,
            ai_decision: Some("T".to_string()),
            ai_explanation: Some(format!("L'IA sécurise {} points.", turn_score)),
        };
    }
}