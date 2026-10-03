fn calculate_annual_interest(principal_str: &str, rate_str: &str) -> Option<f64> {
    let principal = principal_str.parse::<f64>().ok()?;
    let rate = rate_str.parse::<f64>().ok()?;

    // is_finite() checks for BOTH NaN and Infinity in one call!
    if !principal.is_finite() || !rate.is_finite() || principal < 0.0 || rate < 0.0 {
        return None;
    }

    Some(principal * (rate / 100.0))
}

fn main() {
    // Valid inputs
    let valid_interest = calculate_annual_interest("10000.0", "5.5");
    assert!(valid_interest.is_some());
    assert!((valid_interest.unwrap() - 550.0).abs() < f64::EPSILON);

    // Invalid string input
    assert_eq!(calculate_annual_interest("invalid_num", "5.5"), None);

    // Invalid float state (NaN / Infinity)
    assert_eq!(calculate_annual_interest("NaN", "5.5"), None);
    assert_eq!(calculate_annual_interest("10000.0", "inf"), None);

    // Logical domain error (negative values)
    assert_eq!(calculate_annual_interest("-500.0", "5.5"), None);

    println!("Float Parsing & Validation Challenge Passed!");
}