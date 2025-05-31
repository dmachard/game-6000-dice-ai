# 🎲 6000 Dice Game

A fun experiment to see how "super intelligent" AIs perform at a simple dice game.

## 🎯 The Concept

A dice game where humans face off against expensive AIs in an epic battle to reach 6000 points. Watch them **over-analyze every single roll...**,

## 🧠 Rules Summary

- **Straight (1–6):** 2000 points
- **Three pairs:** 1500 points
- **Six of a kind:** value × 1000
- **Three 1s:** 1000 points
- **Three of a kind (2–6):** value × 100
- **Each 1:** 100 points
- **Each 5:** 50 points
- **No points:** Lose your turn's accumulated score

## 📸 Screenshots

Below are some screenshots of the game in action:

![AI Turn](screenshots/cli_gameplay_human_v1.png)
![AI Turn](screenshots/cli_gameplay_ai_v0.png)
![AI Turn](screenshots/cli_human_win.png)

## 🔧 Setup (optional)

You may provide at least one valid API key to enable AI/LLM gameplay.

To use OpenAI's GPT-4, set your API key:

```bash
export OPENAI_API_KEY=your_key_here
```

To use Anthropic Claude 4, set your API key:

```bash
export ANTHROPIC_API_KEY=your_key_here
```

You can configure one or both. The game will use the corresponding AI(s) based on the keys provided.

## 🚀 Start game

```bash
cargo run play
```

## 🧠 AI Personalities

To make things even more entertaining, each AI can be assigned a unique personality:

```yaml
ai_personality: "vicious" # Options: "default", "paranoid", "academic", "vicious"