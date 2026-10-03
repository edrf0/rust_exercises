fn total_long_word_chars(text: &str) -> usize {
    text.split_whitespace()
        .map(|s| s.trim_matches(|c: char| c.is_ascii_punctuation()).len()) // Transforms &str -> usize
        .filter(|&len| len > 3)                                             // Filters usize directly
        .sum()
}

fn main() {
    let text = "Rust is fast, safe, and extremely efficient for systems development!";

    // Words > 3 chars: "Rust" (4), "fast" (4), "safe" (4), "extremely" (9), "efficient" (9), "systems" (7), "development" (11)
    // Total chars = 4 + 4 + 4 + 9 + 9 + 7 + 11 = 48
    assert_eq!(total_long_word_chars(text), 48);
    println!("Pipeline Exercise 2 Passed!");
}