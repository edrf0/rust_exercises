use std::borrow::Cow;
use std::cell::{Cell, RefCell};

#[derive(Default)]
pub struct AuditTracker {
    pub modified_count: Cell<usize>,
    pub modifications_log: RefCell<Vec<String>>,
}

impl AuditTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_modification(&self, original: &str) {
        self.modified_count.set(self.modified_count.get() + 1);
        self.modifications_log.borrow_mut().push(original.to_string());
    }

    pub fn sanitize_log<'a>(&self, log: &'a str) -> Cow<'a, str> {
        if log.contains("SECRET_TOKEN") {
            let modified_log = log.replace("SECRET_TOKEN", "[REDACTED]");
            self.record_modification(&log);
            Cow::Owned(modified_log)
        } else {
            Cow::Borrowed(log)
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_copy_and_interior_mutability() {
        let tracker = AuditTracker::new();

        let clean_line = "INFO 2026-10-07 Service started cleanly";
        let dirty_line = "WARN 2026-10-07 Auth failed for SECRET_TOKEN in request";

        // Test 1: Clean line must return Cow::Borrowed matching exact pointer address
        let res1 = tracker.sanitize_log(clean_line);
        assert!(matches!(res1, Cow::Borrowed(_)));
        assert_eq!(res1, clean_line);

        // Test 2: Dirty line must return Cow::Owned
        let res2 = tracker.sanitize_log(dirty_line);
        assert!(matches!(res2, Cow::Owned(_)));
        assert_eq!(res2, "WARN 2026-10-07 Auth failed for [REDACTED] in request");

        // Test 3: Audit stats updated via interior mutability
        assert_eq!(tracker.modified_count.get(), 1);

        let logs = tracker.modifications_log.borrow();
        assert_eq!(logs.len(), 1);
        assert!(logs[0].contains(dirty_line));
    }
}