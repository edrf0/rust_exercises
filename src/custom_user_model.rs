use std::fmt;

#[derive(Debug, PartialEq)]
pub struct User {
    pub username: String,
    pub email: String,
    pub active: bool,
}

impl Default for User {
    fn default() -> Self {
        Self {
            username: "guest".to_string(),
            email: "guest@example.com".to_string(),
            active: true,
        }
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let status = if self.active { "Active" } else { "Inactive" };
        write!(f, "{} <{}> ({})", self.username, self.email, status)
    }
}

pub fn new_user<S: AsRef<str>>(username: S, email: S) -> User {
    User {
        username: username.as_ref().to_string(),
        email: email.as_ref().to_string(),
        active: true,
    }
}

fn main() {
    // 1. Test Default
    let default_user = User::default();
    assert_eq!(default_user.username, "guest");
    assert_eq!(default_user.email, "guest@example.com");
    assert!(default_user.active);

    // 2. Test Display
    let user = User {
        username: "alice".to_string(),
        email: "alice@rust.org".to_string(),
        active: true,
    };
    assert_eq!(format!("{user}"), "alice <alice@rust.org> (Active)");

    // 3. Test AsRef generic constructor with &str and String inputs
    let u1 = new_user("bob", "bob@rust.org"); // &str inputs
    let u2 = new_user("bob".to_string(), "bob@rust.org".to_string()); // String inputs

    assert_eq!(u1, u2);
    assert_eq!(format!("{u1}"), "bob <bob@rust.org> (Active)");

    println!("Traits Exercise Passed!");
}