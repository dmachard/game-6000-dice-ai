use crate::score::{calculate_score, roll_dice};
use colored::*;
use reqwest::blocking::Client;
use serde_json::json;
use std::env;
use textwrap::wrap;

const NUM_DICE: usize = 6;

pub fn ai_turn(ai_score: u32, human_score: u32) -> u32 {
    println!("{}", "AI Turn".bold().blue());
    let mut dice = NUM_DICE;
    let mut turn_score = 0;
    let mut roll_count = 1;

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

        let prompt = build_prompt(ai_score, human_score, turn_score, remaining_dice, score);
        let (decision, explanation) = ai_decision_with_chatgpt(&prompt);

        let wrapped_explanation = wrap(&explanation, 80);
        let max_lines = 3;

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
        //println!("{} {}", "\tReason:".bold().green(), explanation);

        if decision.trim().eq_ignore_ascii_case("T") {
            println!("{}", "\tAI banks its points.\n".bold().green());
            break;
        } else {
            dice = remaining_dice as usize;
        }
    }

    turn_score
}

fn build_prompt(
    ai_score: u32,
    human_score: u32,
    turn_score: u32,
    remaining_dice: u32,
    score: u32,
) -> String {
    format!(
        "You are the AI playing the 6000 dice game. Follow these rules exactly:\n\
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
        Strategy (aggressive but calculated):\n\
        - If behind, take more risks to catch up, but don't be reckless.\n\
        - If turn score < 300, reroll cautiously, even with few dice.\n\
        - 1 die left: reroll only if likely to get 1 or 5, otherwise take points.\n\
        - If ahead, be cautious but reroll if a big score is likely (>600).\n\
        - Never combine dice from previous rolls.\n\
        \n\
        Current situation:\n\
        - AI score: {ai_score}\n\
        - Human score: {human_score}\n\
        - Turn score: {turn_score}\n\
        - Dice remaining: {remaining_dice}\n\
        - Roll score: {score}\n\
        \n\
        Specific advice:\n\
        - >3 dice: reroll to maximize score.\n\
        - 2 dice: reroll if turn score < 300, else take points.\n\
        - 1 die: take points unless feeling bold.\n\
        \n\
        Respond with exact JSON format only:\n\
        {{\n\
          \"decision\": \"R\",  // \"R\" = roll again, \"T\" = take points\n\
          \"explanation\": \"Explain your decision\"\n\
        }}\n\
        \n\
        Don't mention combinations not present in the roll. Be rigorous.\n\
        Now give your answer:"
    )
}

fn ai_decision_with_chatgpt(prompt: &str) -> (String, String) {
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let client = Client::new();

    let request_body = json!({
        "model": "gpt-4",
        "messages": [
            { "role": "system", "content": "You are the AI for the 6000 dice game." },
            { "role": "user", "content": prompt }
        ]
    });

    let resp = client
        .post("https://api.openai.com/v1/chat/completions")
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
