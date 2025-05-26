use crate::config::Config;
use crate::score::{calculate_score, roll_dice};

use colored::*;
use reqwest::blocking::Client;
use serde_json::json;
use std::env;
use textwrap::wrap;

const NUM_DICE: usize = 6;

#[derive(Debug)]
struct AIDecisionLog {
    remaining_dice: u32,
    turn_score: u32,
    decision: String,
    explanation: String,
}

pub fn ai_turn(
    ai_score: u32,
    other_scores: &[u32],
    ai_type: &Option<String>,
    config: &Config,
) -> u32 {
    let mut dice = NUM_DICE;
    let mut turn_score = 0;
    let mut roll_count = 1;
    let mut history = Vec::new();

    loop {
        println!("{} {}", "\n\tRoll number:".bold().green(), roll_count);

        let roll = roll_dice(dice);
        println!("{} {:?}", "\tAI rolled:".bold().green(), roll);

        let (score, remaining_dice) = calculate_score(&roll);
        println!("{} {}", "\tScore:".bold().cyan(), score);
        println!("{} {}", "\tRemaining dice:".bold().blue(), remaining_dice);

        if score == 0 {
            println!("{}", "\tAI scored nothing. End of turn.\n".bold().red());
            return 0;
        }

        turn_score += score;
        roll_count += 1;

        if remaining_dice == 0 {
            println!("{}", "\tAI gets another full roll!".bold().magenta());
            continue;
        }

        let prompt = build_prompt(
            ai_score,
            other_scores,
            turn_score,
            remaining_dice,
            score,
            &history,
            config.game.prompt_language.as_str(),
        );
        let (decision, explanation) = match ai_type {
            Some(ai_type_str) => match ai_type_str.as_str() {
                "openai" => ai_decision_with_chatgpt(&prompt, config),
                "anthropic" => ai_decision_with_claude(&prompt, config),
                _ => ai_decision_with_claude(&prompt, config), // Fallback to Claude
            },
            None => ai_decision_with_claude(&prompt, config), // Default fallback
        };

        history.push(AIDecisionLog {
            remaining_dice,
            turn_score,
            decision: decision.clone(),
            explanation: explanation.clone(),
        });

        let wrapped_explanation = wrap(&explanation, 80);
        let max_lines = 10;

        println!(
            "{} {}",
            "\tAI decision:".bold().blue(),
            decision.bold().white()
        );
        println!("{}", "\tReason:".bold().green());
        for line in wrapped_explanation.iter().take(max_lines) {
            println!("\t  {}", line);
        }
        if wrapped_explanation.len() > max_lines {
            println!("\t  [...]");
        }

        if decision.trim().eq_ignore_ascii_case("T") {
            println!("{}", "\tAI banks its points.\n".bold().green());
            break;
        } else {
            dice = remaining_dice as usize;
        }
    }

    turn_score
}

fn format_history(history: &[AIDecisionLog]) -> String {
    if history.is_empty() {
        return String::from("No prior AI decisions.");
    }

    let mut formatted = String::from("History of AI decisions:\n");

    for (i, entry) in history.iter().enumerate() {
        formatted.push_str(&format!(
            "Turn={} - remaining_dice={} turn_score={}, decision={}, explanation={}\n",
            i + 1,
            entry.remaining_dice,
            entry.turn_score,
            entry.decision,
            entry.explanation
        ));
    }

    formatted
}

fn build_prompt(
    ai_score: u32,
    other_scores: &[u32],
    turn_score: u32,
    remaining_dice: u32,
    score: u32,
    history: &[AIDecisionLog],
    lang: &str,
) -> String {
    let history_str = format_history(history);

    let language_instruction = match lang {
        "fr" => "french",
        "en" => "english",
        _ => "",
    };

    let other_scores_str = other_scores
        .iter()
        .enumerate()
        .map(|(i, score)| format!("Player {}: {}", i + 1, score))
        .collect::<Vec<_>>()
        .join(", ");

    format!(
        "You are reflecting on the decisions made during this turn. Use the outcomes of each step to adjust your next move.\n\
        You are the AI playing the 6000 dice game. Follow these rules exactly:\n\
        - Straight (1-2-3-4-5-6): 2000 points, all dice used.\n\
        - Three pairs: 1500 points, all dice used.\n\
        - Six of a kind: face value × 1000.\n\
        - Three 1s: 1000 points.\n\
        - Three of 2–6: face value × 100.\n\
        - Each 1: 100 points.\n\
        - Each 5: 50 points.\n\
        - A roll scoring 0 loses all turn points.\n\
        - Only reroll non-scoring dice.\n\
        \n\
        You have the basic rules and scoring system of the 6000 dice game.\n\
        Use the rules and scoring system as a foundation for reasoning.\n\
        Review the outcomes of your rerolls earlier this turn and use them to refine your risk-taking strategy.\n\
        Continuously adapt your strategy based on the evolving game state and recent outcomes.\n\
        Let your own experience from this turn guide your choices, not rigid rules.\n\
        Think critically and learn from your actions to optimize future decisions.\n\
        Do not follow rigid thresholds but learn from your history.\n\
        Each reroll is an independent event; avoid gambler’s fallacy.\n\
        You lose the entire turn score if a reroll produces no scoring dice.\n\
        \n\
        {history_str}\n\
        \n\
        Current situation:\n\
        - AI score: {ai_score}\n\
        - Other players' scores: {other_scores_str}\n\
        - Turn score: {turn_score}\n\
        - Dice remaining: {remaining_dice}\n\
        - Roll score: {score}\n\
        \n\
        Reminder: Player 1 is always a human.\n\
        \n\
        Respond with exact JSON format only:\n\
        {{\n\
          \"decision\": \"R\",  // \"R\" = roll again, \"T\" = take points\n\
          \"explanation\": \"Explain your decision\"\n\
        }}\n\
        \n\
        Don't mention combinations not present in the roll. Be rigorous.\n\
        Now give your answer in {language_instruction}:"
    )
}

fn ai_decision_with_chatgpt(prompt: &str, config: &Config) -> (String, String) {
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let client = Client::new();

    let request_body = json!({
        "model": config.openai.model,
        "messages": [
            { "role": "system", "content": "You are an expert and strategic AI agent playing the 6000 dice game. Your role is to analyze each turn in detail, learn from past outcomes, and make optimal decisions by reasoning through uncertainty and risk. Answer in JSON format with keys: 'decision' and 'explanation'." },
            { "role": "user", "content": prompt }
        ]
    });

    let resp = client
        .post(&config.openai.url)
        .bearer_auth(api_key)
        .json(&request_body)
        .send()
        .expect("Request failed");

    let json_resp: serde_json::Value = resp.json().expect("Invalid JSON");

    let content = &json_resp["choices"][0]["message"]["content"];
    let parsed: serde_json::Value = serde_json::from_str(content.as_str().unwrap_or("")).unwrap();

    (
        parsed["decision"].as_str().unwrap_or("T").to_string(),
        parsed["explanation"].as_str().unwrap_or("").to_string(),
    )
}

fn ai_decision_with_claude(prompt: &str, config: &Config) -> (String, String) {
    let api_key = env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY not set");
    let client = Client::new();

    let request_body = json!({
        "model": config.anthropic.model,
        "max_tokens": 1024,
        "temperature": 0.7,
        "system": "You are an expert and strategic AI agent playing the 6000 dice game. Your role is to analyze each turn in detail, learn from past outcomes, and make optimal decisions by reasoning through uncertainty and risk. Answer in JSON format with keys: 'decision' and 'explanation'.",
        "messages": [
            {
                "role": "user",
                "content": prompt
            }
        ]
    });

    let resp = client
        .post(&config.anthropic.url)
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .json(&request_body)
        .send()
        .expect("Request failed");

    let json_resp: serde_json::Value = resp.json().expect("Invalid JSON");

    let content = &json_resp["content"][0]["text"];
    let parsed: serde_json::Value = serde_json::from_str(content.as_str().unwrap_or(""))
        .unwrap_or_else(|_| {
            eprintln!("Failed to parse AI response as JSON: {:?}", content);
            json!({
                "decision": "T",
                "explanation": "Could not parse response"
            })
        });

    (
        parsed["decision"].as_str().unwrap_or("T").to_string(),
        parsed["explanation"].as_str().unwrap_or("").to_string(),
    )
}
