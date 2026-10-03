fn describe_float(val: f64) -> &'static str {
    if val.is_nan() {
        return "NaN";
    }
    if val.is_infinite() {
        return "Infinity";
    }
    if val == 0.0 || val == -0.0 {
        return "Zero";
    }
    "Normal"
}

fn main() {
    assert_eq!(describe_float(0.0 / 0.0), "NaN");
    assert_eq!(describe_float(1.0 / 0.0), "Infinity");
    assert_eq!(describe_float(-0.0), "Zero");
    assert_eq!(describe_float(42.5), "Normal");
    println!("Exercise 1 Passed!");
}