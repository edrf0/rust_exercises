fn parse_config_setting(line: &str) -> Result<(String, u16), String> {
    let (key, value) = line
        .trim()
        .split_once('=')
        .ok_or_else(|| "Invalid line format: missing '='".to_string())?;

    let key = key.trim();
    if key.is_empty() {
        return Err("Key cannot be empty".to_string());
    }

    // map_err converts ParseIntError into String, then ? early-returns if it fails
    let parsed_val = value
        .trim()
        .parse::<u16>()
        .map_err(|_| "Invalid integer value".to_string())?;

    Ok((key.to_string(), parsed_val))
}

fn main() {
    // Valid cases
    assert_eq!(
        parse_config_setting("  PORT=8080 "),
        Ok(("PORT".to_string(), 8080))
    );

    // Error cases
    assert!(parse_config_setting("NO_EQUALS_SIGN").is_err());
    assert!(parse_config_setting(" =8080").is_err());
    assert!(parse_config_setting("PORT=not_a_number").is_err());
    assert!(parse_config_setting("PORT=70000").is_err()); // Exceeds u16::MAX

    println!("Error Handling Warmup Passed!");
}