use std::fmt;
use std::error::Error;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub enum RecordError {
    EmptyUsername,
    MissingComma,
    InvalidAge(ParseIntError),
}

impl fmt::Display for RecordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RecordError::EmptyUsername => write!(f, "Username cannot be empty"),
            RecordError::MissingComma => write!(f, "Invalid line format: missing ','"),
            RecordError::InvalidAge(err) => write!(f, "Invalid age integer: {err}"),
        }
    }
}

impl Error for RecordError {}

impl From<ParseIntError> for RecordError {
    fn from(err: ParseIntError) -> Self {
        RecordError::InvalidAge(err)
    }
}

fn parse_user_record(line: &str) -> Result<(String, u8), RecordError> {
    let (username, age) = line.split_once(',').ok_or(RecordError::MissingComma)?;
    if username.is_empty() {
        return Err(RecordError::EmptyUsername);
    }
    let age: u8 = age.parse()?;
    Ok((username.to_string(), age))
}

fn main() {
    assert_eq!(parse_user_record("Alice,30"), Ok(("Alice".to_string(), 30)));
    assert_eq!(parse_user_record("Bob30"), Err(RecordError::MissingComma));
    assert_eq!(parse_user_record(",30"), Err(RecordError::EmptyUsername));
    assert!(matches!(parse_user_record("Alice,not_a_num"), Err(RecordError::InvalidAge(_))));

    println!("Custom std Error Exercise Passed!");
}