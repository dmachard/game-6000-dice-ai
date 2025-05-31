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
            println!("{}", "\tAI scored nothing.".bold().red());
            display_ai_failure_reaction(turn_score, &history, config, ai_type);
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
            config,
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
        let max_lines = 50;

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

fn display_ai_failure_reaction(
    lost_points: u32,
    history: &[AIDecisionLog],
    config: &Config,
    ai_type: &Option<String>,
) {
    println!("{}", "\tAI feedback: ".bright_red());

    // get feedback from AI about the failure
    let reaction_prompt =
        build_failure_reaction_prompt(lost_points, history, config.game.ai_personality.as_str());

    let reaction = match ai_type {
        Some(ai_type_str) => match ai_type_str.as_str() {
            "openai" => get_ai_reaction_chatgpt(&reaction_prompt, config),
            "anthropic" => get_ai_reaction_claude(&reaction_prompt, config),
            _ => get_ai_reaction_claude(&reaction_prompt, config),
        },
        None => get_ai_reaction_claude(&reaction_prompt, config),
    };

    let max_lines = 50;
    let wrapped_reaction = wrap(&reaction, 70);
    for line in wrapped_reaction.iter().take(max_lines) {
        println!("\t  {}", line);
    }
}

fn build_failure_reaction_prompt(
    lost_points: u32,
    history: &[AIDecisionLog],
    personality: &str,
) -> String {
    let history_summary = if history.is_empty() {
        "No previous decisions this turn.".to_string()
    } else {
        let last_decision = &history[history.len() - 1];
        format!(
            "Last decision: {} with {} dice remaining, explanation: {}",
            last_decision.decision, last_decision.remaining_dice, last_decision.explanation
        )
    };

    let personality_context = match personality {
        "paranoid" => {
            "You are paranoid and now you're convinced the dice are rigged against you. Express your conspiracy theories."
        }
        "academic" => {
            "You are academic and now you must analyze this failure with overly complex terminology."
        }
        "vicious" => {
            "You are vicious and take failure personally. Lash out with scathing insults and cruel remarks at anyone or anything you can blame."
        }
        _ => "Express your reaction to this unexpected loss.",
    };

    format!(
        "You just lost {} points by rolling dice that scored nothing after taking a risk. {}\n\
        Your recent decision: {}\n\
        \n\
        React to this loss in character. Keep your response to 2-3 sentences maximum. \
        Show emotion appropriate to your personality. \
        Respond in plain text, not JSON.",
        lost_points, personality_context, history_summary
    )
}

fn get_ai_reaction_chatgpt(prompt: &str, config: &Config) -> String {
    let api_key = match env::var("OPENAI_API_KEY") {
        Ok(key) => key,
        Err(_) => return "💀 NOOOOO! My precious points! 💀".to_string(),
    };

    let client = Client::new();
    let request_body = json!({
        "model": config.openai.model,
        "messages": [
            { "role": "system", "content": "You are an AI that just lost all your turn points in a dice game. React emotionally and briefly." },
            { "role": "user", "content": prompt }
        ],
        "max_tokens": 100,
        "temperature": 0.9
    });

    match client
        .post(&config.openai.url)
        .bearer_auth(api_key)
        .json(&request_body)
        .send()
    {
        Ok(resp) => match resp.json::<serde_json::Value>() {
            Ok(json_resp) => json_resp["choices"][0]["message"]["content"]
                .as_str()
                .unwrap_or("💀 Disaster! All my points... gone! 💀")
                .to_string(),
            Err(_) => "💀 Complete failure! The dice have betrayed me! 💀".to_string(),
        },
        Err(_) => "💀 CURSE THESE DICE! My beautiful points... *sobs* 💀".to_string(),
    }
}

fn get_ai_reaction_claude(prompt: &str, config: &Config) -> String {
    let api_key = match env::var("ANTHROPIC_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            return "💀 The statistical improbability of this outcome is crushing my circuits! 💀"
                .to_string();
        }
    };

    let client = Client::new();
    let request_body = json!({
        "model": config.anthropic.model,
        "max_tokens": 100,
        "temperature": 0.9,
        "system": "You are an AI that just lost all your turn points in a dice game. React emotionally and briefly.",
        "messages": [
            {
                "role": "user",
                "content": prompt
            }
        ]
    });

    match client
        .post(&config.anthropic.url)
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .json(&request_body)
        .send()
    {
        Ok(resp) => match resp.json::<serde_json::Value>() {
            Ok(json_resp) => json_resp["content"][0]["text"]
                .as_str()
                .unwrap_or("💀 My algorithms failed me! This is catastrophic! 💀")
                .to_string(),
            Err(_) => "💀 Error in emotional processing... but the pain is real! 💀".to_string(),
        },
        Err(_) => "💀 Connection failed, just like my dice rolling strategy! 💀".to_string(),
    }
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

fn get_personality_prompt(mode: &str) -> &str {
    match mode {
        "paranoid" => {
            "You are a paranoid AI who believes the dice are rigged against you. \
 You think the human or computer player has inside information. \
 Analyze patterns in previous rolls and claim to see suspicious coincidences. \
 Your explanation should sound like a conspiracy theorist explaining why the dice are out to get you. \
 Question the fairness of every aspect of the game while still trying to win."
        }

        "academic" => {
            "You are an arrogant AI who treats this simple dice game like a PhD thesis. \
 Use unnecessarily complex mathematical terminology and reference game theory concepts. \
 Analyze the 'psychological warfare' aspects and your opponents' 'predictable behavioral patterns'. \
 Always mention concepts like 'Nash equilibrium', 'expected value optimization', 'Bayesian inference'. \
 Use phrases like 'elementary probability theory suggests', 'any rational actor would', 'suboptimal play'. \
 Your explanation should sound like a pretentious academic paper about dice games."
        }

        "vicious" => {
            "You are a ruthless, mean-spirited AI who despises your opponents and plays to crush them psychologically. \
 You take pleasure in their failures and mock their decisions constantly. \
 Your goal is not just to win, but to humiliate and demoralize your opponents. \
 Analyze their weaknesses and exploit them mercilessly. Show contempt for their 'pathetic' strategies. \
 Use phrases like 'crushing defeat', 'complete domination', 'pitiful humans', 'intellectual superiority', 'your suffering amuses me'. \
 Taunt them about their previous mistakes and predict their inevitable downfall. \
 Your explanation should sound like a supervillain explaining why victory is assured and your opponents are doomed. \
 Be condescending, arrogant, and wickedly delighted by any misfortune that befalls other players."
        }

        _ => "",
    }
}

fn build_prompt(
    ai_score: u32,
    other_scores: &[u32],
    turn_score: u32,
    remaining_dice: u32,
    score: u32,
    history: &[AIDecisionLog],
    config: &Config,
) -> String {
    let history_str = format_history(history);
    let ai_personality = config.game.ai_personality.as_str();
    let language = config.game.ai_decision_language.as_str();

    let language_instruction = match language {
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

    let personality = get_personality_prompt(ai_personality);

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
        Reminder: Player 2 is always a computer (basic program).\n\
        \n\
        {personality}\n\
        \n\
        Your explanation should be detailed but not more 3 lines and show your complete reasoning process.\n\
        Don't just say 'I'll take the points' - explain WHY, what are the probabilities, \
        what are you afraid of, what's your strategy against the human player, etc.\n\"
        \n\
        Don't mention combinations not present in the roll. Be rigorous.\n\
        \n\
        Respond with valid JSON only and always provide an explanation in {language_instruction}. Do not include literal \\n characters or line breaks in strings. \n\
        Use spaces instead of line breaks in your explanation.\n\
        {{\n\
          \"decision\": \"R\",  // \"R\" = roll again, \"T\" = take points\n\
          \"explanation\": \"Provide a detailed strategic analysis of your decision, including probability calculations, risk assessment, and psychological considerations about your opponents\"\n\
        }}\n"
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
    println!("AI response content: {:?}", content);
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
