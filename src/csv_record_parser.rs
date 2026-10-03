fn parse_csv(csv_data: &str) -> Vec<Vec<String>> {
    csv_data
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| {
            line.split(',')
                .map(|field| field.trim().to_string())
                .collect() // Collects fields into Vec<String>
        })
        .collect() // Collects lines into Vec<Vec<String>>
}

fn main() {
    let data = "
# User Data
alice , 30 , admin
bob,25,user

# System Account

guest , 18 , guest
";

    let parsed = parse_csv(data);

    let expected = vec![
        vec!["alice".to_string(), "30".to_string(), "admin".to_string()],
        vec!["bob".to_string(), "25".to_string(), "user".to_string()],
        vec!["guest".to_string(), "18".to_string(), "guest".to_string()],
    ];

    assert_eq!(parsed, expected);
    println!("Exercise 1 Passed!");
}