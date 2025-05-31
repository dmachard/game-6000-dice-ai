use dice6000::config::Config;
use dice6000::game::setup_players;

#[test]
fn test_setup_players_when_no_ai_enabled() {
    let config = Config::init();
    let players = setup_players(false, false, &config);
    assert_eq!(players.len(), 2);

    assert_eq!(players[0].name, "Human");
    assert!(players[0].is_human);

    assert_eq!(players[1].name, "Computer");
    assert!(!players[1].is_human);
}

#[test]
fn test_setup_players_with_openai() {
    let config = Config::init();
    let players = setup_players(true, false, &config);

    assert_eq!(players.len(), 3);

    assert_eq!(players[0].name, "Human");
    assert_eq!(players[1].name, "Computer");
    assert_eq!(players[2].name, "AI (OpenAI)");
}

#[test]
fn test_setup_players_with_anthropic() {
    let config = Config::init();
    let players = setup_players(false, true, &config);
    assert_eq!(players.len(), 3);

    assert_eq!(players[0].name, "Human");
    assert_eq!(players[1].name, "Computer");
    assert_eq!(players[2].name, "AI (Claude)");
}

#[test]
fn test_setup_players_with_both_ais() {
    let config = Config::init();
    let players = setup_players(true, true, &config);
    println!("Players: {:?}", players);
    assert_eq!(players.len(), 4);

    assert_eq!(players[0].name, "Human");
    assert_eq!(players[1].name, "Computer");
    assert_eq!(players[2].name, "AI (OpenAI)");
    assert_eq!(players[3].name, "AI (Claude)");
}
