use std::error::Error;
use std::fmt;
use std::num::ParseIntError;
use std::sync::mpsc;
use std::thread;
use std::thread::JoinHandle;

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
        PipelineError::ParseFailure(err.to_string())
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

pub fn run_parallel_pipeline(inputs: Vec<&'static str>) -> Vec<Result<u32, PipelineError>> {
    let (tx, rx) = mpsc::channel::<Result<u32, PipelineError>>();
    let mut handles: Vec<JoinHandle<()>> = Vec::with_capacity(inputs.len());
    for input in inputs {
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            let result = process_raw_input(input);
            let _ = tx_clone.send(result);
        });
        handles.push(handle);
    }
    for handle in handles {
        if let Err(e) = handle.join() {
            eprintln!("Error in thread: {:?}", e);
        }
    }
    drop(tx);
    rx.into_iter().collect()
}

fn main() {
    let inputs = vec!["42", "0", "150", "abc"];
    let results = run_parallel_pipeline(inputs);

    assert_eq!(results.len(), 4);
    assert!(results.contains(&Ok(42)));
    assert!(results.contains(&Err(PipelineError::EmptyInput)));
    assert!(results.contains(&Err(PipelineError::ValueOutOfRange(150))));

    println!("Parallel Channel Pipeline Exercise Passed!");
}