use rand::Rng;

pub fn roll_dice(n: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(1..=6)).collect()
}

/// Calculate the score for a given roll of dice.
/// Returns a tuple (score, remaining_dice).
///
/// Scoring rules:
/// - Straight (1-2-3-4-5-6): 2000 points, all dice used.
/// - Three pairs: 1500 points, all dice used.
/// - Six of a kind: face value * 1000 points, all dice used.
/// - Three 1s: 1000 points.
/// - Three of 2–6: face value * 100 points.
/// - Each remaining 1: 100 points.
/// - Each remaining 5: 50 points.
///
/// Remaining dice are those that did not contribute to the score.
/// If no scoring dice, score is 0 and all dice remain.
pub fn calculate_score(dice: &[u8]) -> (u32, u32) {
    let mut counts: [u32; 7] = [0; 7];
    for &d in dice {
        counts[d as usize] += 1;
    }

    let mut score = 0;

    if counts[1..=6] == [1, 1, 1, 1, 1, 1] {
        return (2000, 0); // Straight
    }

    if counts.iter().filter(|&&c| c == 2).count() == 3 {
        return (1500, 0); // Three pairs
    }

    for (i, &count) in counts.iter().enumerate().skip(1) {
        if count == 6 {
            return ((i as u32) * 1000, 0);
        }
    }

    for (i, count) in counts.iter_mut().enumerate().skip(2) {
        if *count >= 3 {
            score += (i as u32) * 100;
            *count -= 3;
        }
    }

    if counts[1] == 3 {
        score += 1000;
        counts[1] = 0;
    }

    score += (counts[1].min(2)) * 100;
    counts[1] = counts[1].saturating_sub(2);
    score += (counts[5].min(2)) * 50;
    counts[5] = counts[5].saturating_sub(2);

    let remaining_dice = counts.iter().skip(1).sum::<u32>();
    (score, remaining_dice)
}
