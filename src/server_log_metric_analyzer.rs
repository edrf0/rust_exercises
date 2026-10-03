use std::fmt;
use std::error::Error;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub enum LogError {
    EmptyLine,
    MalformedLine,
    InvalidStatusCode(ParseIntError),
}

impl Display for LogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogError::EmptyLine => write!(f, "Log line is empty"),
            LogError::MalformedLine => write!(f, "Log line is malformed. Format: \
            [YYYY-MM-DD] IP: ip_address - Status: status_code - status_message_body"),
            LogError::InvalidStatusCode(err) => write!(f, "Invalid status code: {err}"),
        }
    }
}

impl Error for LogError {}

impl From<ParseIntError> for LogError {
    fn from(err: ParseIntError) -> Self {
        LogError::InvalidStatusCode(err)
    }
}

#[derive(Debug, PartialEq)]
pub struct LogReport {
    pub total_logs: usize,
    pub error_count: usize,
    pub unique_ips: HashSet<String>,
    pub status_codes: HashMap<u16, usize>,
}

impl Default for LogReport {
    fn default() -> Self {
        Self {
            total_logs: 0,
            error_count: 0,
            unique_ips: HashSet::<String>::new(),
            status_codes: HashMap::<u16, usize>::new(),
        }
    }
}

impl Display for LogReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"Total logs: {}\nError count: {}\nUnique IPs: {:?}\nStatus codes: {:?}\n",
               self.total_logs,self.error_count, self.unique_ips, self.status_codes)
    }
}

fn analyze_log(line: &str, log_report: &mut LogReport) -> Result<(), LogError> {
    let line = line.trim();
    if line.is_empty() {
        return Err(LogError::EmptyLine);
    }
    // "[YYYY-MM-DD] IP: ip_address - Status: status_code - status_message_body"
    let token_vector = line
        .split(' ')
        .filter(|s| !s.contains(":") && *s != "-")
        .map(|s| s.trim_matches(['[',']',' ']))
        .collect::<Vec<&str>>();
    if token_vector.len() < 4 {
        return Err(LogError::MalformedLine);
    }
    let code = token_vector[2].parse::<u16>()?;
    log_report.total_logs += 1;
    log_report.error_count += if code != 200 { 1 } else { 0 };
    log_report.unique_ips.insert(token_vector[1].to_string());
    *log_report.status_codes.entry(code).or_insert(0) += 1;

    Ok(())
}

fn main() {
    let logs = vec![
        "[2026-03-30] IP: 192.168.1.10 - Status: 200 - OK",
        "[2026-03-30] IP: 192.168.1.15 - Status: 404 - Not Found",
        "[2026-03-30] IP: 192.168.1.10 - Status: 500 - Server Crash",
        "   ",
        "[2026-03-30] IP: 192.168.1.20 - Status: bad_code - Malformed",
    ];

    let mut log_report = LogReport::default();

    for log in logs {
        match analyze_log(log, &mut log_report) {
            Ok(()) => println!("{log}"),
            Err(err) => println!("{err}"),
        }
    }

    println!("{log_report}");

    println!("Mini-project skeleton ready!");
}