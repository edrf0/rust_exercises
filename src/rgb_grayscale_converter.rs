fn rgb_to_grayscale(rgb: (u8, u8, u8)) -> u8 {
    let (r, g, b) = rgb;
    let luminosity = 0.299 * r as f64 + 0.587 * g as f64 + 0.114 * b as f64;
    luminosity as u8
}

fn main() {
    let pure_red = (255, 0, 0);
    assert_eq!(rgb_to_grayscale(pure_red), 76); // 0.299 * 255 ≈ 76

    let white = (255, 255, 255);
    assert_eq!(rgb_to_grayscale(white), 255);
    println!("Tuple Exercise Passed!");
}