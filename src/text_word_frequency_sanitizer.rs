fn sanitize_and_group(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(|word| word.trim_matches(|c: char| c.is_ascii_punctuation()).to_lowercase())
        .filter(|word| !word.is_empty())
        .fold(Vec::new(), |mut acc, word| {
            if !acc.contains(&word) {
                acc.push(word);
            }
            acc
        })
}

fn main() {
    let text = "Hello, world! Welcome to Rust. Hello again, Rust world!";
    let result = sanitize_and_group(text);

    let expected = vec![
        "hello".to_string(),
        "world".to_string(),
        "welcome".to_string(),
        "to".to_string(),
        "rust".to_string(),
        "again".to_string(),
    ];

    assert_eq!(result, expected);
    println!("Exercise 2 Passed!");
}