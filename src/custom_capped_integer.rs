pub struct CustomCappedU32<const CAP: u32> {
    value: u32,
}

impl<const CAP: u32> CustomCappedU32<CAP> {
    pub fn new(value: u32) -> Self {
        Self { value: value.min(CAP) }
    }
    pub fn saturating_add(self, other: Self) -> Self {
        match self.value.checked_add(other.value) {
            Some(new_value) if new_value <= CAP => Self { value: new_value },
            _ => Self { value: CAP }
        }
    }
}

pub fn safe_boundary(a: u32, b: u32) -> CustomCappedU32<1000> {
    let custom_a = CustomCappedU32::<1000>::new(a);
    let custom_b = CustomCappedU32::<1000>::new(b);
    custom_a.saturating_add(custom_b)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_boundary() {
        let c = safe_boundary(10, 5);
        assert_eq!(c.value, 15);
        let c = safe_boundary(1000, 1000);
        assert_eq!(c.value, 1000);
        let c = safe_boundary(20000, 100000);
        assert_eq!(c.value, 1000);
    }
}