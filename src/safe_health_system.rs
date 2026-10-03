use std::cmp::min;

fn apply_damage(current_health: u8, damage: u8) -> u8 {
    current_health.saturating_sub(damage)
}

fn apply_healing(current_health: u8, heal_amount: u8, max_health: u8) -> u8 {
    min(current_health.saturating_add(heal_amount),max_health)
}

fn main() {
    assert_eq!(apply_damage(30, 50), 0); // No underflow panic!
    assert_eq!(apply_healing(80, 50, 100), 100); // Capped at max health!
    println!("Exercise 1 Passed!");
}