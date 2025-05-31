use dice6000::computer::{
    AIPersonality, ComputerAI, DecisionContext, GamePhase, PerformanceTracker,
};
use dice6000::config::Config;

#[test]
fn test_ai_personalities() {
    let _config = Config::init();

    // Test conservative AI
    let conservative_ai = ComputerAI::new(3000, &[3000], AIPersonality::Conservative);
    assert!(conservative_ai.risk_tolerance() < 0.5);

    // Test aggressive AI
    let aggressive_ai = ComputerAI::new(3000, &[3000], AIPersonality::Aggressive);
    assert!(aggressive_ai.risk_tolerance() > 0.5);
}

#[test]
fn test_game_phase_detection() {
    assert_eq!(
        ComputerAI::determine_game_phase(1000, 1500),
        GamePhase::Early
    );
    assert_eq!(ComputerAI::determine_game_phase(3000, 2500), GamePhase::Mid);
    assert_eq!(
        ComputerAI::determine_game_phase(4500, 4000),
        GamePhase::Late
    );
    assert_eq!(
        ComputerAI::determine_game_phase(5800, 5000),
        GamePhase::Critical
    );
}

#[test]
fn test_risk_calculation() {
    let ai = ComputerAI::new(3000, &[3000], AIPersonality::Balanced);

    // Should be very risky with 1 die
    assert!(ai.calculate_dice_risk(1) > 0.6);

    // Should be less risky with more dice
    assert!(ai.calculate_dice_risk(4) < 0.5);
}

#[test]
fn test_adaptive_behavior() {
    let mut ai = ComputerAI::new(3000, &[3000], AIPersonality::Adaptive);
    let initial_risk = ai.risk_tolerance();

    // Simulate several risky decisions
    for _ in 0..8 {
        ai.add_test_decision(true, 400, 2); // Add risky decisions using the public method
    }

    // Create a mock context for adaptation
    let mut performance = PerformanceTracker::new();
    // Simulate high bust rate to trigger conservative adaptation
    performance.record_bust(300, 2);
    performance.record_bust(250, 1);
    performance.record_bust(400, 2);
    performance.record_success(200);
    performance.record_success(150);

    let context = DecisionContext {
        remaining_dice: 3,
        turn_score: 300,
        total_score: 3000,
        roll_count: 2,
        last_roll: &[1, 2, 3],
        performance: &performance,
    };

    ai.adapt_from_history(&context);

    // Risk tolerance should have decreased due to:
    // 1. Too many risky decisions (8+ out of 10 with dice <= 2)
    // 2. High bust rate (60% = 3 busts out of 5 total turns)
    assert!(
        ai.risk_tolerance() < initial_risk,
        "Expected risk tolerance {} to be less than initial {}",
        ai.risk_tolerance(),
        initial_risk
    );

    println!(
        "Initial risk: {}, Final risk: {}",
        initial_risk,
        ai.risk_tolerance()
    );
}

#[test]
fn test_adaptive_behavior_detailed() {
    let mut ai = ComputerAI::new(3000, &[3000], AIPersonality::Adaptive);
    let initial_risk = ai.risk_tolerance();

    // Test the specific conditions that should trigger adaptation
    println!("Initial risk tolerance: {}", initial_risk);

    // Add exactly the risky decisions that should trigger adaptation
    for i in 0..8 {
        ai.add_test_decision(true, 400, 2); // 8 risky decisions
        println!(
            "Added risky decision {}, history length: {}",
            i + 1,
            ai.decision_history_len()
        );
    }

    // Add 2 safe decisions to reach the 10 minimum for adaptation
    ai.add_test_decision(false, 300, 4);
    ai.add_test_decision(false, 250, 3);

    println!("Total decisions in history: {}", ai.decision_history_len());

    // Create performance tracker with high bust rate
    let mut performance = PerformanceTracker::new();
    // Create a 50% bust rate (3 busts, 3 successes)
    for _ in 0..3 {
        performance.record_bust(300, 2);
        performance.record_success(200);
    }

    let context = DecisionContext {
        remaining_dice: 3,
        turn_score: 300,
        total_score: 3000,
        roll_count: 2,
        last_roll: &[1, 2, 3],
        performance: &performance,
    };

    ai.adapt_from_history(&context);
    let final_risk = ai.risk_tolerance();

    println!("Final risk tolerance: {}", final_risk);
    println!("Risk reduction: {}", initial_risk - final_risk);

    // Should be reduced due to too many risky decisions (8 out of 10 with dice <= 2)
    // The threshold is 5, so 8 > 5 should trigger reduction
    assert!(
        final_risk < initial_risk,
        "Expected final risk {} to be less than initial {}",
        final_risk,
        initial_risk
    );
}
