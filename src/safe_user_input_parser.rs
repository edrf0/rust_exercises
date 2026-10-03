use std::convert::TryFrom;

fn process_port(input_port: u32, multiplier: u16) -> Option<u16> {
    let u16_input_port = u16::try_from(input_port);
    match u16_input_port {
        Ok(port) => port.checked_mul(multiplier),
        Err(_) => None,
    }
}

fn main() {
    assert_eq!(process_port(8080, 2), Some(16160));
    assert_eq!(process_port(70000, 1), None); // Invalid port (> u16::MAX)
    assert_eq!(process_port(40000, 2), None); // Multiplication overflows u16
    println!("Exercise 2 Passed!");
}