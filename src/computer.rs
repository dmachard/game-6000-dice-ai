use crate::config::Config;
use crate::score::{calculate_score, roll_dice};
use colored::*;

const NUM_DICE: usize = 6;
const WINNING_SCORE: u32 = 6000;

/// Represents different AI personality types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AIPersonality {
    Conservative,
    Balanced,
    Aggressive,
    Adaptive,
}

impl AIPersonality {
    fn base_risk_tolerance(&self) -> f64 {
        match self {
            AIPersonality::Conservative => 0.3,
            AIPersonality::Balanced => 0.5,
            AIPersonality::Aggressive => 0.7,
            AIPersonality::Adaptive => 0.5, // Will be adjusted dynamically
        }
    }
}

pub fn computer_turn(current_score: u32, other_scores: &[u32], config: &Config) -> u32 {
    let mut dice = NUM_DICE;
    let mut turn_score = 0;
    let mut roll_count = 1;

    // Enhanced AI with configurable personality
    let personality = determine_ai_personality(config);
    let mut ai = ComputerAI::new(current_score, other_scores, personality);

    // Track performance for adaptive learning
    let mut performance_tracker = PerformanceTracker::new();

    loop {
        println!("{} {}", "\n\tRoll number:".bold().green(), roll_count);
        let roll = roll_dice(dice);
        println!("{} {:?}", "\tComputer rolled:".bold().green(), roll);

        let (score, remaining_dice, remaining_dice_values) = calculate_score(&roll);
        println!("{} +{}", "\tScore:".bold().cyan(), score);

        if score == 0 {
            println!("{}", "\tNo points! Computer loses the turn.\n".bold().red());
            performance_tracker.record_bust(turn_score, remaining_dice);
            return 0;
        }

        turn_score += score;
        roll_count += 1;

        // All dice scored - mandatory reroll
        if remaining_dice == 0 {
            println!(
                "{}",
                "\tAll dice scored! Computer gets to roll again!"
                    .bold()
                    .magenta()
            );
            dice = NUM_DICE;
            continue;
        }

        println!("{} {} - {:?}", "\tRemaining dice:".bold().blue(), remaining_dice, remaining_dice_values);

        // Enhanced AI decision making with context
        let roll_u32: Vec<u32> = roll.iter().map(|&x| x as u32).collect();
        let context = DecisionContext {
            remaining_dice,
            turn_score,
            total_score: current_score,
            roll_count,
            last_roll: &roll_u32,
            performance: &performance_tracker,
        };

        let decision = ai.make_decision(&context);

        if decision.should_continue {
            println!("{}", "\tComputer decides to ROLL AGAIN!".bold().yellow());
            if let Some(reasoning) = &decision.reasoning {
                println!("\t{} {}", "Reasoning:".italic().dimmed(), reasoning);
            }
            dice = remaining_dice as usize;
        } else {
            println!(
                "{}",
                "\tComputer decides to TAKE the points!".bold().green()
            );
            if let Some(reasoning) = &decision.reasoning {
                println!("\t{} {}", "Reasoning:".italic().dimmed(), reasoning);
            }
            break;
        }
    }

    performance_tracker.record_success(turn_score);
    turn_score
}

pub fn determine_ai_personality(config: &Config) -> AIPersonality {
    match config.game.computer_strategy.to_lowercase().as_str() {
        "conservative" => AIPersonality::Conservative,
        "balanced" => AIPersonality::Balanced,
        "aggressive" => AIPersonality::Aggressive,
        "adaptive" => AIPersonality::Adaptive,
        _ => {
            // Stratégie par défaut si la configuration n'est pas reconnue
            eprintln!(
                "invalid '{}' strategy, use 'balanced'by default",
                config.game.computer_strategy
            );
            AIPersonality::Balanced
        }
    }
}

#[derive(Debug)]
pub struct DecisionContext<'a> {
    pub remaining_dice: u32,
    pub turn_score: u32,
    pub total_score: u32,
    pub roll_count: u32,
    pub last_roll: &'a [u32],
    pub performance: &'a PerformanceTracker,
}

#[derive(Debug)]
struct AIDecision {
    should_continue: bool,
    reasoning: Option<String>,
}

pub struct ComputerAI {
    personality: AIPersonality,
    risk_tolerance: f64,
    min_turn_score: u32,
    max_turn_score: u32,
    opponent_best: u32,
    decision_history: Vec<(bool, u32, u32)>, // (decision, turn_score, dice_left)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GamePhase {
    Early,    // < 2000 points
    Mid,      // 2000-4000 points
    Late,     // 4000+ points
    Critical, // Someone near winning
}

impl ComputerAI {
    pub fn new(current_score: u32, other_scores: &[u32], personality: AIPersonality) -> Self {
        let opponent_best = other_scores.iter().max().copied().unwrap_or(0);
        let game_phase = Self::determine_game_phase(current_score, opponent_best);

        let (risk_tolerance, min_turn_score, max_turn_score) = Self::calculate_initial_parameters(
            current_score,
            opponent_best,
            personality,
            game_phase,
        );

        Self {
            personality,
            risk_tolerance,
            min_turn_score,
            max_turn_score,
            opponent_best,
            decision_history: Vec::new(),
        }
    }

    pub fn risk_tolerance(&self) -> f64 {
        self.risk_tolerance
    }

    pub fn min_turn_score(&self) -> u32 {
        self.min_turn_score
    }

    pub fn max_turn_score(&self) -> u32 {
        self.max_turn_score
    }

    pub fn decision_history_len(&self) -> usize {
        self.decision_history.len()
    }

    pub fn decision_history(&self) -> &[(bool, u32, u32)] {
        &self.decision_history
    }

    pub fn add_test_decision(&mut self, continued: bool, turn_score: u32, dice_left: u32) {
        self.decision_history
            .push((continued, turn_score, dice_left));
    }

    pub fn personality(&self) -> AIPersonality {
        self.personality
    }

    pub fn determine_game_phase(current_score: u32, opponent_best: u32) -> GamePhase {
        let max_score = current_score.max(opponent_best);

        if max_score >= WINNING_SCORE - 1000 {
            GamePhase::Critical
        } else if max_score >= 4000 {
            GamePhase::Late
        } else if max_score >= 2000 {
            GamePhase::Mid
        } else {
            GamePhase::Early
        }
    }

    fn calculate_initial_parameters(
        current_score: u32,
        opponent_best: u32,
        personality: AIPersonality,
        game_phase: GamePhase,
    ) -> (f64, u32, u32) {
        let base_risk = personality.base_risk_tolerance();

        // Adjust risk based on position
        let position_modifier = if opponent_best > current_score + 1500 {
            0.3 // More aggressive when behind
        } else if current_score > opponent_best + 1500 {
            -0.2 // More conservative when ahead
        } else {
            0.0
        };

        // Adjust for game phase
        let phase_modifier = match game_phase {
            GamePhase::Early => 0.1, // Slightly more aggressive early
            GamePhase::Mid => 0.0,   // Balanced
            GamePhase::Late => -0.1, // Slightly more conservative
            GamePhase::Critical => {
                if opponent_best > WINNING_SCORE - 800 {
                    0.4 // Very aggressive if opponent about to win
                } else if current_score > WINNING_SCORE - 800 {
                    -0.3 // Very conservative if we're about to win
                } else {
                    0.0
                }
            }
        };

        let risk_tolerance = (base_risk + position_modifier + phase_modifier).clamp(0.1, 0.9);

        // Calculate turn score targets
        let min_turn_score = match game_phase {
            GamePhase::Early => {
                if opponent_best > current_score + 1000 {
                    400
                } else {
                    250
                }
            }
            GamePhase::Mid => {
                if opponent_best > current_score + 1000 {
                    500
                } else {
                    300
                }
            }
            GamePhase::Late => {
                if opponent_best > current_score + 1000 {
                    600
                } else {
                    350
                }
            }
            GamePhase::Critical => {
                if opponent_best > WINNING_SCORE - 800 {
                    800 // Need big scores when opponent is close
                } else {
                    200 // Conservative when we're close
                }
            }
        };

        let max_turn_score = min_turn_score * 3;

        (risk_tolerance, min_turn_score, max_turn_score)
    }

    fn make_decision(&mut self, context: &DecisionContext) -> AIDecision {
        // Multiple decision factors
        let factors = self.analyze_decision_factors(context);

        // Adaptive learning from past decisions
        if self.personality == AIPersonality::Adaptive {
            self.adapt_from_history(context);
        }

        // Make the decision
        let should_continue = self.calculate_final_decision(&factors, context);
        let reasoning = self.generate_reasoning(&factors, should_continue);

        // Record decision for learning
        self.decision_history
            .push((should_continue, context.turn_score, context.remaining_dice));

        AIDecision {
            should_continue,
            reasoning: Some(reasoning),
        }
    }

    fn analyze_decision_factors(&self, context: &DecisionContext) -> DecisionFactors {
        DecisionFactors {
            dice_risk: self.calculate_dice_risk(context.remaining_dice),
            score_pressure: self.calculate_score_pressure(context.turn_score),
            position_pressure: self.calculate_position_pressure(context),
            time_pressure: self.calculate_time_pressure(context),
            momentum: self.calculate_momentum(context),
            opportunity_cost: self.calculate_opportunity_cost(context),
        }
    }

    pub fn calculate_dice_risk(&self, remaining_dice: u32) -> f64 {
        // More reasonable risk assessment
        let base_risk = match remaining_dice {
            1 => 0.8,  // 80% risk - dangerous but sometimes necessary
            2 => 0.5,  // 50% risk - risky but manageable
            3 => 0.3,  // 30% risk - reasonable
            4 => 0.15, // 15% risk - quite safe
            5 => 0.1,  // 10% risk - very safe
            6 => 0.05, // 5% risk - extremely safe
            _ => 0.0,
        };

        // Adjust based on actual scoring probability
        let scoring_prob = self.calculate_scoring_probability(remaining_dice as usize);
        base_risk * (1.0 - scoring_prob * 0.5) // Less penalty for good scoring chances
    }

    fn calculate_score_pressure(&self, turn_score: u32) -> f64 {
        // Much less pressure to stop with low scores
        if turn_score < 200 {
            -0.5 // Strong pressure to continue
        } else if turn_score < self.min_turn_score {
            -0.2 // Pressure to continue
        } else if turn_score > self.max_turn_score {
            0.6 // Pressure to stop
        } else {
            // Gradual increase in pressure to stop
            (turn_score as f64 - self.min_turn_score as f64)
                / (self.max_turn_score as f64 - self.min_turn_score as f64)
                * 0.3
        }
    }

    fn calculate_position_pressure(&self, context: &DecisionContext) -> f64 {
        let score_diff = self.opponent_best as i32 - context.total_score as i32;

        if score_diff > 2000 {
            -0.4 // Pressure to take risks when far behind
        } else if score_diff < -1000 {
            0.3 // Pressure to be conservative when ahead
        } else {
            score_diff as f64 / 5000.0 // Gradual pressure
        }
    }

    fn calculate_time_pressure(&self, context: &DecisionContext) -> f64 {
        // Pressure based on how close anyone is to winning
        let max_score = context.total_score.max(self.opponent_best);
        let distance_to_win = WINNING_SCORE - max_score;

        if distance_to_win <= 500 {
            if context.total_score > self.opponent_best {
                0.4
            } else {
                -0.6
            }
        } else if distance_to_win <= 1500 {
            if context.total_score > self.opponent_best {
                0.2
            } else {
                -0.3
            }
        } else {
            0.0
        }
    }

    fn calculate_momentum(&self, context: &DecisionContext) -> f64 {
        // Consider recent roll quality and streak
        let roll_quality = self.assess_roll_quality(context.last_roll);
        let streak_bonus = if context.roll_count > 3 { 0.2 } else { 0.0 };

        (roll_quality - 0.5) * 0.3 + streak_bonus
    }

    fn calculate_opportunity_cost(&self, context: &DecisionContext) -> f64 {
        // What we might gain vs what we might lose
        let _potential_gain = self.estimate_potential_gain(context.remaining_dice);
        let potential_loss = context.turn_score as f64;

        if potential_loss > 800.0 {
            (potential_loss / 1000.0) * 0.4 // High opportunity cost
        } else {
            0.0
        }
    }

    fn calculate_final_decision(
        &self,
        factors: &DecisionFactors,
        context: &DecisionContext,
    ) -> bool {
        // Always continue if we have very few points and plenty of dice (basic common sense)
        if context.turn_score < 200 && context.remaining_dice >= 3 {
            return true;
        }

        // Always continue if we haven't reached minimum threshold and have safe dice count
        if context.turn_score < self.min_turn_score && context.remaining_dice >= 3 {
            return true;
        }

        // Never continue with 1 die and substantial points unless desperate
        if context.remaining_dice == 1 && context.turn_score >= 300 {
            if self.calculate_time_pressure(context) < -0.5 {
                // Only if we're desperate
                return context.turn_score < 500;
            }
            return false;
        }

        // Be very careful with 2 dice unless we really need points
        if context.remaining_dice == 2 && context.turn_score >= 400 {
            return context.turn_score < self.min_turn_score;
        }

        // With 3+ dice, be more willing to continue
        if context.remaining_dice >= 3 {
            // Continue if we haven't reached a reasonable threshold
            if context.turn_score < 300 {
                return true;
            }
            // Continue if we haven't reached our minimum and dice count is good
            if context.turn_score < self.min_turn_score {
                return true;
            }
        }

        // Weighted decision based on all factors (only for borderline cases)
        let continue_score = -factors.dice_risk * 0.2
            + -factors.score_pressure * 0.3
            + -factors.position_pressure * 0.2
            + -factors.time_pressure * 0.15
            + factors.momentum * 0.1
            + -factors.opportunity_cost * 0.15
            + (self.risk_tolerance - 0.5) * 0.2;

        continue_score > -0.1 // Lower threshold for continuing
    }

    fn generate_reasoning(&self, factors: &DecisionFactors, decision: bool) -> String {
        let mut reasons = Vec::new();

        if factors.dice_risk > 0.6 {
            reasons.push("High risk with few dice".to_string());
        }

        if factors.score_pressure > 0.5 {
            reasons.push("Good points accumulated".to_string());
        } else if factors.score_pressure < -0.2 {
            reasons.push("Need more points".to_string());
        }

        if factors.position_pressure < -0.3 {
            reasons.push("Behind in game".to_string());
        } else if factors.position_pressure > 0.2 {
            reasons.push("Leading in game".to_string());
        }

        if factors.time_pressure.abs() > 0.3 {
            if factors.time_pressure > 0.0 {
                reasons.push("Close to winning".to_string());
            } else {
                reasons.push("Opponent close to winning".to_string());
            }
        }

        if reasons.is_empty() {
            if decision {
                "Calculated risk worth taking".to_string()
            } else {
                "Better to secure these points".to_string()
            }
        } else {
            reasons.join(", ")
        }
    }

    pub fn adapt_from_history(&mut self, context: &DecisionContext) {
        // Simple adaptive learning - adjust risk tolerance based on recent performance
        if self.decision_history.len() >= 10 {
            let recent_decisions: Vec<_> = self.decision_history.iter().rev().take(10).collect();

            let risky_decisions = recent_decisions
                .iter()
                .filter(|(continued, _score, dice)| *continued && *dice <= 2)
                .count();

            // If we've been taking too many risky decisions, become more conservative
            if risky_decisions > 5 {
                self.risk_tolerance *= 0.9;
            } else if risky_decisions < 2 {
                self.risk_tolerance *= 1.05;
            }

            self.risk_tolerance = self.risk_tolerance.clamp(0.2, 0.8);
        }

        // Use performance tracker data to adjust behavior
        let bust_rate = if context.performance.busted_turns.len()
            + context.performance.successful_turns.len()
            > 0
        {
            context.performance.busted_turns.len() as f64
                / (context.performance.busted_turns.len()
                    + context.performance.successful_turns.len()) as f64
        } else {
            0.3 // Default assumption
        };

        // If we're busting too often, become more conservative
        if bust_rate > 0.4 {
            self.risk_tolerance *= 0.95;
        } else if bust_rate < 0.2 {
            self.risk_tolerance *= 1.02;
        }

        self.risk_tolerance = self.risk_tolerance.clamp(0.2, 0.8);
    }

    fn calculate_scoring_probability(&self, dice_count: usize) -> f64 {
        // Enhanced probability calculation including three-of-a-kind possibilities
        match dice_count {
            1 => 2.0 / 6.0,                    // Only 1 and 5 score
            2 => 1.0 - (4.0f64 / 6.0).powi(2), // At least one 1 or 5
            3 => {
                // At least one 1 or 5, or three of a kind
                let no_scoring = (4.0f64 / 6.0).powi(3); // No 1s or 5s
                let three_of_kind_prob = 6.0 * (1.0f64 / 6.0).powi(3); // Any three of a kind
                1.0 - no_scoring + three_of_kind_prob
            }
            4 => 0.83,
            5 => 0.90,
            6 => 0.95,
            _ => 0.0,
        }
    }

    fn assess_roll_quality(&self, roll: &[u32]) -> f64 {
        // Simple quality assessment based on scoring dice
        let ones = roll.iter().filter(|&&x| x == 1).count();
        let fives = roll.iter().filter(|&&x| x == 5).count();
        let total_scoring = ones + fives;

        (total_scoring as f64 / roll.len() as f64).min(1.0)
    }

    fn estimate_potential_gain(&self, remaining_dice: u32) -> f64 {
        // Rough estimate of potential points from remaining dice
        let prob = self.calculate_scoring_probability(remaining_dice as usize);
        let avg_score = match remaining_dice {
            1 => 75.0, // Average of 1 (100) and 5 (50)
            2 => 120.0,
            3 => 180.0,
            4 => 250.0,
            5 => 320.0,
            6 => 400.0,
            _ => 0.0,
        };

        prob * avg_score
    }
}

#[derive(Debug)]
struct DecisionFactors {
    dice_risk: f64,
    score_pressure: f64,
    position_pressure: f64,
    time_pressure: f64,
    momentum: f64,
    opportunity_cost: f64,
}

#[derive(Debug, Default)]
pub struct PerformanceTracker {
    successful_turns: Vec<u32>,
    busted_turns: Vec<(u32, u32)>, // (points_lost, dice_when_busted)
}

impl PerformanceTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_success(&mut self, points: u32) {
        self.successful_turns.push(points);
    }

    pub fn record_bust(&mut self, points_lost: u32, dice_remaining: u32) {
        self.busted_turns.push((points_lost, dice_remaining));
    }
}
