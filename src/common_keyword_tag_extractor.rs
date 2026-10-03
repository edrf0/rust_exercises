use std::collections::HashSet;

fn tokenize(s: &str) -> HashSet<String> {
    s.split_whitespace()
        .map(|w| w.trim_matches(|c: char| c.is_ascii_punctuation()).to_lowercase())
        .filter(|w| !w.is_empty())
        .collect()
}

fn find_shared_keywords(article_a: &str, article_b: &str, stop_words: &[&str]) -> HashSet<String> {
    let stop_words_set: HashSet<&str> = stop_words.iter().copied().collect();
    let set_a = tokenize(article_a);
    let set_b = tokenize(article_b);

    set_a.intersection(&set_b)
        .filter(|word| !stop_words_set.contains(word.as_str()))
        .cloned()
        .collect()
}

fn main() {
    let article_a = "Rust provides memory safety without a garbage collector!";
    let article_b = "Memory safety and high performance make Rust awesome.";
    let stop_words = vec!["a", "and", "without", "make", "or"];

    let shared = find_shared_keywords(article_a, article_b, &stop_words);

    let mut expected = HashSet::new();
    expected.insert("rust".to_string());
    expected.insert("memory".to_string());
    expected.insert("safety".to_string());

    assert_eq!(shared, expected);
    println!("HashSet Exercise Passed!");
}