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
pub fn calculate_score(dice: &[u8]) -> (u32, u32, Vec<u8>, Vec<usize>) {
    let mut counts: [u32; 7] = [0; 7];
    for &d in dice {
        counts[d as usize] += 1;
    }

    let mut score = 0;
    let mut remaining_counts = counts.clone();

    if counts[1..=6] == [1, 1, 1, 1, 1, 1] {
        return (2000, 0, Vec::new(), Vec::new()); // Straight, all dice used
    }

    if counts.iter().filter(|&&c| c == 2).count() == 3 {
        return (1500, 0, Vec::new(), Vec::new()); // Three pairs, all dice used
    }

    for (i, &count) in counts.iter().enumerate().skip(1) {
        if count == 6 {
            return ((i as u32) * 1000, 0, Vec::new(), Vec::new()); // Six of a kind, all dice used
        }
    }

    // Handle three of a kind for 2-6
    for (i, count) in remaining_counts.iter_mut().enumerate().skip(2) {
        if *count >= 3 {
            score += (i as u32) * 100;
            *count -= 3;
        }
    }
    
    // Handle three 1s (special case: 1000 points)
    if remaining_counts[1] >= 3 {
        score += 1000;
        remaining_counts[1] -= 3;
    }
    
    // Handle remaining 1s (100 points each)
    let remaining_ones = remaining_counts[1].min(2);
    score += remaining_ones * 100;
    remaining_counts[1] -= remaining_ones;
    
    // Handle remaining 5s (50 points each)
    let remaining_fives = remaining_counts[5].min(2);
    score += remaining_fives * 50;
    remaining_counts[5] -= remaining_fives;
    
    // Calculate total remaining dice count
    let remaining_dice_count = remaining_counts.iter().skip(1).sum::<u32>();
    
    // Build the vector of remaining dice values and their indices in the original roll
    let mut remaining_dice_values = Vec::new();
    let mut remaining_indices = Vec::new();
    let mut restants = remaining_counts.clone();
    for (i, &die) in dice.iter().enumerate() {
        if die > 0 && restants[die as usize] > 0 {
            remaining_dice_values.push(die);
            remaining_indices.push(i);
            restants[die as usize] -= 1;
        }
    }
    (score, remaining_dice_count, remaining_dice_values, remaining_indices)
}
