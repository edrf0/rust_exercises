use std::collections::HashMap;
use std::fmt;
use std::error::Error;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    EmptyInput,
    MissingEquals(usize), // Line index (0-based) where '=' was missing
    InvalidKey(String),   // The offending key string
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::EmptyInput => write!(f, "Empty input"),
            ParseError::MissingEquals(u) => write!(f, "Missing equals {}", u),
            ParseError::InvalidKey(key) => write!(f, "Invalid key {}", key),
        }
    }
}

impl Error for ParseError {}

pub fn parse_config(raw: &str) -> Result<HashMap<String, String>, ParseError> {
    if raw.trim().is_empty() {
        return Err(ParseError::EmptyInput);
    }
    let mut hashmap = HashMap::<String, String>::new();
    for (line_index, line) in raw.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if !line.contains('=') {
            return Err(ParseError::MissingEquals(line_index))
        }
        let (key,value) = line.split_once('=').unwrap_or(("", ""));
        let key = key.trim();
        if key.is_empty() || key.contains(' ') {
            return Err(ParseError::InvalidKey(key.to_string()));
        }
        hashmap.insert(key.to_string(), value.to_string());
    }
    Ok(hashmap)
}

fn main() {
    // Test 1: Empty input
    assert_eq!(parse_config("   \n "), Err(ParseError::EmptyInput));

    // Test 2: Missing equals sign on line index 1
    let bad_line = "PORT=8080\nINVALID_LINE\nHOST=localhost";
    assert_eq!(parse_config(bad_line), Err(ParseError::MissingEquals(1)));

    // Test 3: Invalid key (contains spaces)
    let bad_key = "SERVER PORT=8080";
    assert_eq!(parse_config(bad_key), Err(ParseError::InvalidKey("SERVER PORT".to_string())));

    // Test 4: Valid parsing
    let valid = "PORT=8080\n\nHOST=127.0.0.1\nLOG_LEVEL=debug";
    let config = parse_config(valid).unwrap();
    assert_eq!(config.get("PORT"), Some(&"8080".to_string()));
    assert_eq!(config.get("HOST"), Some(&"127.0.0.1".to_string()));
    assert_eq!(config.get("LOG_LEVEL"), Some(&"debug".to_string()));

    println!("Success! Custom error handling parser passed all assertions.");
}