use std::cell::{Cell, RefCell, UnsafeCell};
use std::ptr;

// ==========================================
// Part 1: `Cell<T>` for Copy Types
// ==========================================
pub struct Metrics {
    hit_count: Cell<usize>,
}

impl Metrics {
    pub fn new(initial_hits: usize) -> Self {
        Self { hit_count: Cell::new(initial_hits) }
    }

    pub fn record_hit(&self) {
        self.hit_count.set(self.hit_count.get() + 1);
    }

    pub fn get_hits(&self) -> usize {
        self.hit_count.get()
    }
}

// ==========================================
// Part 2: `RefCell<T>` for Non-Copy Types
// ==========================================
pub struct AuditSystem {
    logs: RefCell<Vec<String>>,
}

impl AuditSystem {
    pub fn new() -> Self {
        Self { logs: RefCell::new(Vec::new()) }
    }


    pub fn add_log(&self, entry: &str) {
        self.logs.borrow_mut().push(entry.to_string());
    }

    pub fn print_logs(&self) {
        println!("{}", self.logs.borrow().join("\n"));
    }
}

// ==========================================
// Part 3: `UnsafeCell<T>` Building Block
// ==========================================
pub struct RawCell<T> {
    value: UnsafeCell<T>,
}

impl<T> RawCell<T> {
    pub fn new(value: T) -> Self {
        Self { value: UnsafeCell::new(value) }
    }

    pub fn set(&self, value: T) {
        unsafe { ptr::write(self.value.get(), value) };
    }

    pub fn get_ptr(&self) -> *mut T {
        self.value.get()
    }
}

impl<T: Copy> RawCell<T> {
    pub fn get_copy(&self) -> T {
        unsafe { *self.value.get() }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cells() {
        // Test Part 1: Cell
        let m = Metrics::new(0);
        m.record_hit();
        m.record_hit();
        assert_eq!(m.get_hits(), 2);

        // Test Part 2: RefCell
        let audit = AuditSystem::new();
        audit.add_log("User logged in");
        audit.add_log("User performed action");
        assert_eq!(audit.logs.borrow().len(), 2);

        // Test Part 3: UnsafeCell
        let rc = RawCell::new(10);
        assert_eq!(rc.get_copy(), 10);
        rc.set(42);
        assert_eq!(rc.get_copy(), 42);
        let rc = RawCell::new("abc".to_string());
        assert_eq!(unsafe { (*rc.get_ptr()).clone() }, "abc".to_string());
    }
}