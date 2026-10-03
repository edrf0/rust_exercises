fn get_api_endpoint(user_id: u32, is_premium: bool) -> String {
    // true -> Some("premium") -> unwrap Some("premium") -> "premium"
    // false -> None -> _or -> "free"
    let tier = is_premium.then_some("premium").unwrap_or("free");
    format!("/api/v1/{}/user/{}", tier, user_id)
}

fn main() {
    assert_eq!(get_api_endpoint(42, true), "/api/v1/premium/user/42");
    assert_eq!(get_api_endpoint(42, false), "/api/v1/free/user/42");
    println!("Bool Exercise Passed!");
}