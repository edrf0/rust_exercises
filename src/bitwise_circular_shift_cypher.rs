fn scramble_byte(byte: u8, key: u8, shift: u32) -> u8 {
    let xor_byte = byte ^ key;
    xor_byte.rotate_left(shift)
}

fn main() {
    let original: u8 = 0b1100_0011; // 195
    let key: u8      = 0b0000_1111; // 15
    let scrambled = scramble_byte(original, key, 2);

    // XOR result: 0b1100_1100 (204)
    // Rotated left by 2: 0b0011_0011 (51)
    assert_eq!(scrambled, 0b0011_0011);
    println!("Exercise 3 Passed!");
}