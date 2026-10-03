fn parse_hex_digit(ch: char) -> Option<u8> {
    ch.to_digit(16).and_then(|d| u8::try_from(d).ok())
    // match ch.to_digit(16) {
    //     Some(d) => u8::try_from(d).ok(),
    //     None => None,
    // }
}

fn main() {
    assert_eq!(parse_hex_digit('0'), Some(0));
    assert_eq!(parse_hex_digit('a'), Some(10));
    assert_eq!(parse_hex_digit('F'), Some(15));
    assert_eq!(parse_hex_digit('g'), None); // Invalid hex
    println!("Char Exercise Passed!");
}