use std::collections::HashMap;

/// 1. Count the occurrences of each word in a slice using the Entry API.
pub fn count_words<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
    words.iter().fold(HashMap::new(), |mut acc, &word| {
        *acc.entry(word).or_insert(0) += 1;
        acc
    })
}

/// 2. Safely retrieve the count of a word, returning 0 if not found.
pub fn get_word_count(counts: &HashMap<&str, usize>, word: &str) -> usize {
    counts.get(word).copied().unwrap_or(0)
}

/// 3. Remove all entries with a count strictly less than `min_count` in-place.
pub fn filter_frequent_words(counts: &mut HashMap<&str, usize>, min_count: usize) {
    counts.retain(|_, v| *v >= min_count);
}

/// 4. Merge `incoming` counts into `primary` counts. If a word exists in both, sum their counts.
pub fn merge_word_counts<'a>(primary: &mut HashMap<&'a str, usize>, incoming: HashMap<&'a str, usize>) {
    incoming.iter().for_each(|(k, v)| {
        primary.entry(k).and_modify(|cnt| *cnt += v).or_insert(*v);
    })
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_counter() {
        let words = vec!["rust", "code", "rust", "fast", "code", "rust"];
        let mut counts = count_words(&words);

        assert_eq!(get_word_count(&counts, "rust"), 3);
        assert_eq!(get_word_count(&counts, "fast"), 1);
        assert_eq!(get_word_count(&counts, "missing"), 0);

        filter_frequent_words(&mut counts, 2);
        assert_eq!(get_word_count(&counts, "rust"), 3);
        assert_eq!(get_word_count(&counts, "code"), 2);
        assert_eq!(get_word_count(&counts, "fast"), 0); // Removed

        let extra = HashMap::from([("rust", 1), ("python", 2)]);
        merge_word_counts(&mut counts, extra);
        assert_eq!(get_word_count(&counts, "rust"), 4);
        assert_eq!(get_word_count(&counts, "python"), 2);
    }
}