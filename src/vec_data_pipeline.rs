/// 1. Initialize a vector with a pre-allocated capacity and populate it.
pub fn initialize_scores(initial: &[u32]) -> Vec<u32> {
    // let mut vec = Vec::with_capacity(initial.len());
    // for &i in initial {
    //     vec.push(i);
    // }
    // vec
    //
    // initial.iter()
    //     .copied()
    //     .collect()
    //
    // initial.to_vec()
    //
    Vec::from(initial)
}

/// 2. Remove all scores strictly less than `threshold` in-place.
pub fn filter_failing_scores(scores: &mut Vec<u32>, threshold: u32) {
    scores.retain(|&x| x >= threshold);
}

/// 3. Sort the vector and remove duplicate values.
pub fn deduplicate_and_sort(scores: &mut Vec<u32>) {
    scores.sort_unstable();
    scores.dedup();
}

/// 4. Perform a binary search for a target score.
pub fn find_score_position(scores: &[u32], target: u32) -> Result<usize, usize> {
    scores.binary_search(&target)
}

/// 5. Drain the last `n` elements out of the vector and return them as a new Vec.
pub fn extract_top_scores(scores: &mut Vec<u32>, n: usize) -> Vec<u32> {
    scores[scores.len() - n..].iter().copied().collect()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline() {
        let mut scores = initialize_scores(&[45, 90, 85, 45, 60, 100, 75, 90, 30]);
        assert_eq!(scores.len(), 9);

        filter_failing_scores(&mut scores, 50);
        assert!(!scores.contains(&45));
        assert!(!scores.contains(&30));

        deduplicate_and_sort(&mut scores);
        for i in 1..scores.len() {
            assert!(scores[i - 1] < scores[i]);
        }

        let pos = find_score_position(&scores, 90);
        assert!(pos.is_ok());

        let top = extract_top_scores(&mut scores, 2);
        assert_eq!(top.len(), 2);
    }
}