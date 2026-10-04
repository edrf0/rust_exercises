use std::collections::HashMap;

pub fn count_words(text: &str) -> HashMap<String, usize> {
    let mut word_count_map = HashMap::new();
    text.split_whitespace()
        .fold(&mut word_count_map, |wcm, word| {
            *wcm.entry(word.to_lowercase()).or_insert(0) += 1;
            wcm
        });
    word_count_map
}

pub fn most_frequent(word_count_map: &HashMap<String, usize>) -> Option<(String, usize)> {
    word_count_map.iter()
        .max_by_key(|&(_, count)| count)
        .map(|(key, count)| (key.clone(), *count))
}

fn main() {
    let text = "Rust is fast and Rust is safe and Rust is fun";

    let word_count_map = count_words(text);

    assert_eq!(word_count_map.get("rust"), Some(&3));
    assert_eq!(word_count_map.get("is"), Some(&3));
    assert_eq!(word_count_map.get("and"), Some(&2));

    let word_maxcount_tuple = most_frequent(&word_count_map).unwrap();
    // "rust" or "is" both have count 3
    assert_eq!(word_maxcount_tuple.1, 3);
    assert!(word_maxcount_tuple.0 == "rust" || word_maxcount_tuple.0 == "is");

    println!("Success! HashMap counter works cleanly.");
}