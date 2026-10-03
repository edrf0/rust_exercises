fn parse_query_string(query: &str) -> Vec<(String, String)> {
    query.split('&')
        .filter(|&s| !s.is_empty())
        .filter_map(|s| s.split_once('='))
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

fn main() {
    let query = "name=Alice&age=30&&role=admin&badpair";

    let parsed = parse_query_string(query);

    let expected = vec![
        ("name".to_string(), "Alice".to_string()),
        ("age".to_string(), "30".to_string()),
        ("role".to_string(), "admin".to_string()),
    ];

    assert_eq!(parsed, expected);
    println!("Query String Challenge Passed!");
}