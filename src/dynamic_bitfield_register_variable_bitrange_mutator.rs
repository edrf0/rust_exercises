#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct Register64(pub u64);

impl Register64 {
    pub fn new(val: u64) -> Self {
        Self(val)
    }

    pub fn get_bit(&self, index: u8) -> bool {
        if index >= 64 { return false; }
        self.0 & (1 << index) != 0
    }

    pub fn set_bit(&mut self, index: u8) {
        if index >= 64 { return; }
        self.0 |= 1 << index
    }

    pub fn clear_bit(&mut self, index: u8) {
        if index >= 64 { return; }
        self.0 &= !(1 << index)
    }

    pub fn toggle_bit(&mut self, index: u8) {
        if index >= 64 { return; }
        self.0 ^= 1 << index
    }

    pub fn get_range(&self, start_bit: u8, length: u8) -> u64 {
        if start_bit >= 64 || start_bit + length > 64 { return 0; }
        (self.0 >> start_bit) & ((1 << length) - 1)
    }

    pub fn set_range(&mut self, start_bit: u8, length: u8, value: u64) {
        if start_bit >= 64 || start_bit + length > 64 { return; }
        // Turn off range bits in self.0
        let initial_ones_mask = (1 << length) - 1;
        let shifted_ones_mask = initial_ones_mask << start_bit;
        let zeros_mask = !shifted_ones_mask;
        self.0 &= zeros_mask;
        // Extract range bits from value
        let value_range_bits = value & initial_ones_mask;
        // Shift extracted value range bits
        let shifted_value_bits = value_range_bits << start_bit;
        // Set range in self.0
        self.0 |= shifted_value_bits;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_individual_bit_operations() {
        let mut reg = Register64::new(0);

        reg.set_bit(3);
        reg.set_bit(15);
        assert!(reg.get_bit(3));
        assert!(reg.get_bit(15));
        assert!(!reg.get_bit(4));

        reg.toggle_bit(3); // 1 -> 0
        reg.toggle_bit(4); // 0 -> 1
        assert!(!reg.get_bit(3));
        assert!(reg.get_bit(4));

        reg.clear_bit(15);
        assert!(!reg.get_bit(15));
    }

    #[test]
    fn test_bit_range_operations() {
        let mut reg = Register64::new(0);

        // Set a 10-bit field (bits 12..22) to value 0x2AB (683)
        reg.set_range(12, 10, 0x2AB);

        assert_eq!(reg.get_range(12, 10), 0x2AB);

        // Ensure surrounding bits remain untouched (bits 0..12 and 22..64 are zero)
        assert_eq!(reg.get_range(0, 12), 0);
        assert_eq!(reg.get_range(22, 42), 0);

        // Overwrite overlapping region (bits 16..8 = 8 bits) with 0xFF
        reg.set_range(16, 8, 0xFF);
        assert_eq!(reg.get_range(16, 8), 0xFF);
    }
}