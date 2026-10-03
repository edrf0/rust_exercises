use std::collections::HashMap;

fn word_frequencies(text: &str) -> HashMap<String, usize> {
    text.split_whitespace()
        .map(|s| s.trim_matches(|c:char| c.is_ascii_punctuation()).to_lowercase())
        .filter(|w| !w.is_empty())
        .fold(HashMap::new(), |mut acc, word| {
            *acc.entry(word).or_insert(0) += 1;
            acc
        })
}

fn main() {
    let text = "Rust is fast. Rust is safe! Fast, safe Rust.";
    let freqs = word_frequencies(text);

    assert_eq!(freqs.get("rust"), Some(&3));
    assert_eq!(freqs.get("is"), Some(&2));
    assert_eq!(freqs.get("fast"), Some(&2));
    assert_eq!(freqs.get("safe"), Some(&2));
    assert_eq!(freqs.get("missing"), None);

    println!("HashMap Exercise Passed!");
}