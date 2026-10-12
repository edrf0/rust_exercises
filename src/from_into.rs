#[derive(Debug, PartialEq)]
pub enum Status {
    Active,
    Pending,
    Inactive,
    Unknown,
}

impl From<u8> for Status {
    fn from(val: u8) -> Self {
        match val {
            0 => Status::Active,
            1 => Status::Pending,
            2 => Status::Inactive,
            _ => Status::Unknown,
        }
    }
}

impl From<String> for Status {
    fn from(s: String) -> Self {
        let s = s.to_lowercase();
        match s.as_str() {
            "active" => Status::Active,
            "pending" => Status::Pending,
            "inactive" => Status::Inactive,
            _ => Status::Unknown,
        }
    }
}

impl From<&str> for Status {
    fn from(s: &str) -> Self {
        Self::from(s.to_string())
    }
}

pub fn log_message(msg: impl Into<String>) -> String {
    let text: String = msg.into();
    format!("Message: {}", text)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_u8() {
        assert_eq!(Status::from(0), Status::Active);
        assert_eq!(Status::from(1), Status::Pending);
        assert_eq!(Status::from(2), Status::Inactive);
        assert_eq!(Status::from(99), Status::Unknown);
    }

    #[test]
    fn test_from_string() {
        assert_eq!(Status::from("active".to_string()), Status::Active);
        assert_eq!(Status::from("pending"), Status::Pending);
        assert_eq!(Status::from("unknown_val"), Status::Unknown);
    }

    #[test]
    fn test_into_api() {
        let res1 = log_message("Hello static str");
        let res2 = log_message(String::from("Hello owned string"));
        assert!(!res1.is_empty());
        assert!(!res2.is_empty());
    }
}