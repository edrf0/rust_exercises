use std::fmt;
use std::error::Error;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub struct ServerConfig {
    pub server_id: u32,
    pub host: String,
    pub port: u16,
}

#[derive(Debug, PartialEq)]
pub enum ConfigParseError {
    EmptyServerString,
    MissingBrackets,
    InvalidServerId(ParseIntError),
    MissingColonSeparators,
    InvalidPort(ParseIntError),
}

impl fmt::Display for ConfigParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigParseError::EmptyServerString => write!(f, "Empty server string"),
            ConfigParseError::MissingBrackets => write!(f, "Missing surrounding '[' and ']' for server ID"),
            ConfigParseError::InvalidServerId(err) => write!(f, "Invalid server ID integer: {err}"),
            ConfigParseError::MissingColonSeparators => write!(f, "Missing ':' separator in either host or port"),
            ConfigParseError::InvalidPort(err) => write!(f, "Invalid port integer: {err}"),
        }
    }
}

impl Error for ConfigParseError {}

// Default conversion for ParseIntError (mapped to InvalidPort)
impl From<ParseIntError> for ConfigParseError {
    fn from(err: ParseIntError) -> Self {
        ConfigParseError::InvalidPort(err)
    }
}

fn parse_server_config(line: &str) -> Result<ServerConfig, ConfigParseError> {
    let line = line.trim();

    if line.is_empty() {
        return Err(ConfigParseError::EmptyServerString);
    }

    let _ = line.find('[').ok_or(ConfigParseError::MissingBrackets)?;
    let _ = line.find(']').ok_or(ConfigParseError::MissingBrackets)?;

    if line.chars().filter(|&c| c == ':').count() != 2 {
        return Err(ConfigParseError::MissingColonSeparators)
    }

    let tokens: Vec<&str> = line.split(':').collect();

    let server_id = tokens[0]
        .trim_matches(['[',']',' '])
        .parse::<u32>()
        .map_err(ConfigParseError::InvalidServerId)?;

    let host = tokens[1].trim().to_string();

    let port = tokens[2].trim().parse::<u16>()?;

    Ok(ServerConfig {
        server_id,
        host,
        port,
    })
}

fn main() {
    // 1. Success case
    let valid = "[101]: 127.0.0.1:8080";
    assert_eq!(
        parse_server_config(valid),
        Ok(ServerConfig {
            server_id: 101,
            host: "127.0.0.1".to_string(),
            port: 8080,
        })
    );

    // 2. Error cases
    assert_eq!(parse_server_config("101: 127.0.0.1:8080"), Err(ConfigParseError::MissingBrackets));
    assert!(matches!(parse_server_config("[bad_id]: 127.0.0.1:8080"), Err(ConfigParseError::InvalidServerId(_))));
    assert_eq!(parse_server_config("[101] 127.0.0.1 8080"), Err(ConfigParseError::MissingColonSeparators));
    assert!(matches!(parse_server_config("[101]: 127.0.0.1:not_a_port"), Err(ConfigParseError::InvalidPort(_))));

    println!("Error Handling Drill Passed!");
}