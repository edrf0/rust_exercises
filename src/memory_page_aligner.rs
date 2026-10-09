pub fn align_to_page(address: usize, page_size: usize) -> usize {
    if page_size == 0 { return 0; } // page_size of 0 means 0 memory occupied
    let mod_address_page_size = address % page_size;
    if mod_address_page_size == 0 { return address; } // Already aligned
    address + page_size - mod_address_page_size
}

pub fn is_valid_header(magic_be: [u8; 4]) -> bool {
    u32::from_be_bytes(magic_be) == 0xDEADBEEF
}

pub fn extract_bit_stats(val: u64) -> (u32, u32, bool) {
    (val.count_ones(),val.leading_zeros(),val.is_power_of_two())
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_primitives() {
        // Test 1: Page alignment
        assert_eq!(align_to_page(1024, 1), 1024);
        assert_eq!(align_to_page(100, 4096), 4096);
        assert_eq!(align_to_page(4096, 4096), 4096);
        assert_eq!(align_to_page(4097, 4096), 8192);
        assert_eq!(align_to_page(8192, 4096), 8192);

        // Test 2: Big-endian byte decoding
        let magic_bytes = [0xDE, 0xAD, 0xBE, 0xEF];
        assert!(is_valid_header(magic_bytes));

        let invalid_bytes = [0xEF, 0xBE, 0xAD, 0xDE];
        assert!(!is_valid_header(invalid_bytes));

        // Test 3: Bit stats
        let (ones, leading_zeros, is_pow2) = extract_bit_stats(16);
        assert_eq!(ones, 1);
        assert_eq!(leading_zeros, 59); // 64 bits total - 1 set bit at position 4 - 4 trailing zeros
        assert!(is_pow2);
    }
}