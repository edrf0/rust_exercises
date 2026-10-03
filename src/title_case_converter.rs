fn to_title_case(sentence: &str) -> String {
    sentence
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    // Capitalize first character, lowercase the rest
                    let mut capitalized = first.to_uppercase().to_string();
                    capitalized.push_str(&chars.as_str().to_lowercase());
                    capitalized
                }
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn main() {
    assert_eq!(to_title_case("hello rust world"), "Hello Rust World");
    assert_eq!(to_title_case("fUllStAcK"), "Fullstack");
    println!("String Exercise 1 Passed!");
}