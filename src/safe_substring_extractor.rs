fn safe_truncate(s: &str, max_chars: usize) -> &str {
    // char_indices yields (byte_position, char)
    // .nth(max_chars) finds the byte position where the max_chars index begins
    match s.char_indices().nth(max_chars) {
        Some((byte_idx, _)) => &s[..byte_idx], // Slice up to that exact byte boundary
        None => s,                             // If string has fewer chars, return whole slice
    }
}

fn main() {
    let text = "🦀Rust Programming";

    // "🦀Rust" is 5 characters (🦀 + R + u + s + t), but takes 8 bytes!
    assert_eq!(safe_truncate(text, 5), "🦀Rust");
    assert_eq!(safe_truncate(text, 100), "🦀Rust Programming"); // Exceeds length
    println!("String Exercise 2 Passed!");
}