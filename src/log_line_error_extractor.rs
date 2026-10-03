fn extract_errors(logs: &[&str]) -> Vec<String> {
    logs.iter()
        .filter(|log| log.contains("ERROR"))
        .filter_map(|log| log.split_once(": ")) // Yields Some((_, msg)) or None
        .map(|(_, msg)| msg.to_uppercase())     // Destructures tuple directly!
        .collect()
}

fn main() {
    let logs = vec![
        "INFO: Server started on port 8080",
        "ERROR: Database connection timeout",
        "WARN: Low disk space warning",
        "ERROR: Invalid authorization token",
    ];

    let errors = extract_errors(&logs);

    let expected = vec![
        "DATABASE CONNECTION TIMEOUT".to_string(),
        "INVALID AUTHORIZATION TOKEN".to_string(),
    ];

    assert_eq!(errors, expected);
    println!("Pipeline Exercise 1 Passed!");
}