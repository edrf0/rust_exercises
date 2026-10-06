use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::io::{self, BufRead, BufReader, Write};
use std::num::ParseIntError;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum CommandError {
    IOError(io::Error),
    ParseIntError(ParseIntError),
    FromUtf8Error(FromUtf8Error),
    MissingNumberError,
}

impl Display for CommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandError::IOError(e) => write!(f, "{}", e),
            CommandError::ParseIntError(e) => write!(f, "{}", e),
            CommandError::FromUtf8Error(e) => write!(f, "{}", e),
            CommandError::MissingNumberError => write!(f, "Missing a number"),
        }
    }
}

impl Error for CommandError {}

impl From<io::Error> for CommandError {
    fn from(e: io::Error) -> Self {
        CommandError::IOError(e)
    }
}

impl From<ParseIntError> for CommandError {
    fn from(e: ParseIntError) -> Self {
        CommandError::ParseIntError(e)
    }
}

impl From<FromUtf8Error> for CommandError {
    fn from(e: FromUtf8Error) -> Self {
        CommandError::FromUtf8Error(e)
    }
}

pub fn process_commands<R: BufRead, W: Write>(
    reader: &mut R,
    writer: &mut W
) -> Result<usize, CommandError> {
    let mut counter: usize = 0;
    let mut line_buffer = String::new();

    while reader.read_line(&mut line_buffer)? > 0 {
        let trimmed = line_buffer.trim();
        if trimmed.is_empty() {
            line_buffer.clear();
            continue;
        }

        let mut token_iterator = trimmed.split_whitespace();
        let verb = token_iterator.next().unwrap_or("").to_uppercase();

        match verb.as_str() {
            "ADD" | "SUB" => {
                let val: usize = match token_iterator.next() {
                    Some(val) => val.parse()?,
                    None => {
                        write!(writer, "Error: Invalid command\n")?;
                        return Err(CommandError::MissingNumberError);
                    }
                };

                match verb.as_str() {
                    "ADD" => counter = counter.saturating_add(val),
                    "SUB" => counter = counter.saturating_sub(val),
                    _ => unreachable!(),
                }

                write!(writer, "Current value: {}\n", counter)?;
            }
            "RESET" => {
                counter = 0;
                write!(writer, "Reset to 0\n")?;
            }
            _ => write!(writer, "Error: Invalid command\n")?,
        }

        line_buffer.clear();
    }

    Ok(counter)
}

fn main() -> Result<(),CommandError> {
    // Test input simulating user typing into stdin
    let input_data = "ADD 10\nSUB 3\nINVALID\nADD 5\nRESET\nADD 42\n";

    // We wrap a string slice in BufReader to implement `BufRead` for testing
    let mut reader = BufReader::new(input_data.as_bytes());

    // We use a Vec<u8> as our output buffer implementing `Write`
    let mut output_buffer = Vec::new();

    let final_value = process_commands(&mut reader, &mut output_buffer)?;

    assert_eq!(final_value, 42);

    let output_str = String::from_utf8(output_buffer)?;
    let expected_output = "\
Current value: 10
Current value: 7
Error: Invalid command
Current value: 12
Reset to 0
Current value: 42
";

    assert_eq!(output_str, expected_output);

    println!("Success! Testable std::io trait pipeline passed cleanly.");
    Ok(())
}