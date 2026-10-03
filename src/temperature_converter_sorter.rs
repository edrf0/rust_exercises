fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 1.8 + 32.0
}

fn sort_temperatures(temps: &mut [f64]) {
    temps.sort_by(|a, b| a.total_cmp(b));
}

fn main() {
    let temp_f = celsius_to_fahrenheit(20.0);
    assert!((temp_f - 68.0).abs() < f64::EPSILON);

    let mut temps = vec![100.4, 32.0, 98.6, 72.5];
    sort_temperatures(&mut temps);
    assert_eq!(temps, vec![32.0, 72.5, 98.6, 100.4]);

    println!("Exercise 2 Passed!");
}