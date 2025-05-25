use dice6000::config::Config;
use dice6000::game::setup_players;

#[test]
fn test_setup_players_only_human_when_no_ai_enabled() {
    let config = Config::default();
    let players = setup_players(2, false, false, &config);
    assert_eq!(players.len(), 1);
    assert_eq!(players[0].name, "Human");
}

#[test]
fn test_setup_players_two_players_with_openai() {
    let config = Config::default();
    let players = setup_players(2, true, false, &config);
    assert_eq!(players.len(), 2);
    assert_eq!(players[0].name, "Human");
    assert!(players[0].is_human);
    assert_eq!(players[1].name, "AI (OpenAI)");
}

#[test]
fn test_setup_players_two_players_with_anthropic() {
    let config = Config::default();
    let players = setup_players(2, false, true, &config);
    assert_eq!(players.len(), 2);
    assert_eq!(players[1].name, "AI (Claude)");
}

#[test]
fn test_setup_players_three_players_with_both_ais() {
    let config = Config::default();
    let players = setup_players(3, true, true, &config);
    assert_eq!(players.len(), 3);
    assert_eq!(players[1].name, "AI (OpenAI)");
    assert_eq!(players[2].name, "AI (Claude)");
}

#[test]
fn test_setup_players_unsupported_player_count() {
    let config = Config::default();
    let players = setup_players(4, true, true, &config);
    // Should only have the human player, no AI added
    assert_eq!(players.len(), 1);
    assert_eq!(players[0].name, "Human");
    assert!(players[0].is_human);
}
