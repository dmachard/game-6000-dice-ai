use dice6000::score::calculate_score;

#[test]
fn test_score_single_1() {
    let (score, remaining) = calculate_score(&[1, 2, 3, 4, 6, 6]);
    assert_eq!(score, 100);
    assert_eq!(remaining, 5);
}

#[test]
fn test_score_three_ones() {
    let (score, remaining) = calculate_score(&[1, 1, 1, 2, 3, 4]);
    assert_eq!(score, 1000);
    assert_eq!(remaining, 3);
}

#[test]
fn test_score_straight() {
    let (score, remaining) = calculate_score(&[1, 2, 3, 4, 5, 6]);
    assert_eq!(score, 2000);
    assert_eq!(remaining, 0);
}

#[test]
fn test_score_three_pairs() {
    let (score, remaining) = calculate_score(&[2, 2, 3, 3, 5, 5]);
    assert_eq!(score, 1500);
    assert_eq!(remaining, 0);
}

#[test]
fn test_score_six_of_a_kind() {
    let (score, remaining) = calculate_score(&[4, 4, 4, 4, 4, 4]);
    assert_eq!(score, 4000);
    assert_eq!(remaining, 0);
}

#[test]
fn test_score_three_of_a_kind_and_singles() {
    let (score, remaining) = calculate_score(&[3, 3, 3, 1, 5, 2]);
    assert_eq!(score, 450);
    assert_eq!(remaining, 1);
}

#[test]
fn test_score_no_score() {
    let (score, remaining) = calculate_score(&[2, 3, 4, 6, 6, 2]);
    assert_eq!(score, 0);
    assert_eq!(remaining, 6);
}

#[test]
fn test_score_multiple_ones_and_fives() {
    let (score, remaining) = calculate_score(&[1, 1, 5, 5, 2, 3]);
    assert_eq!(score, 300); // 2 ones * 100 + 2 fives * 50
    assert_eq!(remaining, 2);
}
