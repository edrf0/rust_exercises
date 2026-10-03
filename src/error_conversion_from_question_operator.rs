use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub enum PipelineError {
    EmptyInput,
    ValueOutOfRange(u32),
    ParseFailure(String),
}

impl fmt::Display for PipelineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PipelineError::EmptyInput => write!(f, "Pipeline input cannot be empty"),
            PipelineError::ValueOutOfRange(val) => write!(f, "Value {val} exceeds allowed range"),
            PipelineError::ParseFailure(msg) => write!(f, "Parse failure: {}", msg),
        }
    }
}

impl Error for PipelineError {}

impl From<ParseIntError> for PipelineError {
    fn from(err: ParseIntError) -> Self {
        PipelineError::ParseFailure(format!("{:?}", err))
    }
}

pub fn validate_value(val: u32) -> Result<u32, PipelineError> {
    if val == 0 {
        return Err(PipelineError::EmptyInput);
    }
    if val > 100 {
        return Err(PipelineError::ValueOutOfRange(val));
    }
    Ok(val)
}

pub fn process_raw_input(raw: &str) -> Result<u32, PipelineError> {
    let val = raw.parse::<u32>()?;
    validate_value(val)
}

fn main() {
    assert_eq!(process_raw_input("42"), Ok(42));
    assert_eq!(process_raw_input("0"), Err(PipelineError::EmptyInput));
    assert_eq!(process_raw_input("200"), Err(PipelineError::ValueOutOfRange(200)));

    // Invalid number string triggers ParseIntError -> From conversion -> ParseFailure
    match process_raw_input("abc") {
        Err(PipelineError::ParseFailure(_)) => println!("Parse error handled cleanly!"),
        _ => panic!("Expected ParseFailure!"),
    }

    println!("From Trait Error Conversion Exercise Passed!");
}