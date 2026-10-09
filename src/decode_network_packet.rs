#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PacketSummary {
    pub payload_size: u16,
    pub channel_id_native: u16,
    pub required_slots: usize,
    pub next_pow2_buffer_capacity: usize,
    pub has_odd_parity_flags: bool,
}

pub fn parse_and_align_packet(
    raw_header: u64,
    ring_buffer_slot_size: usize,
) -> Option<PacketSummary> {
    let payload_size = (raw_header & 0xFF_FF) as u16;
    if payload_size == 0 {
        return None;
    }
    let channel_id_native = (((raw_header >> 16) & 0xFF_FF) as u16).swap_bytes();
    let flags = ((raw_header >> 32) & 0xFF_FF) as u16;
    let required_slots = payload_size.div_ceil(ring_buffer_slot_size as u16) as usize;
    let next_pow2_buffer_capacity = payload_size.next_power_of_two() as usize;
    let has_odd_parity_flags = flags.count_ones() % 2 != 0;

    Some(PacketSummary {
        payload_size,
        channel_id_native,
        required_slots,
        next_pow2_buffer_capacity,
        has_odd_parity_flags,
    })
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_practical_packet_framing() {
        // Construct a raw 64-bit header:
        // payload_size = 300 (0x012C) -> bits 0..16
        // channel_id = 0x1234 in Big Endian -> bits 16..32
        // flags = 0x0007 (3 bits set: 0b111 -> odd parity) -> bits 32..48
        let payload: u64 = 300;
        let channel_be: u64 = 0x1234;
        let flags: u64 = 0x0007;

        let raw_header: u64 = payload | (channel_be << 16) | (flags << 32);

        let summary = parse_and_align_packet(raw_header, 64).expect("Valid packet");

        assert_eq!(summary.payload_size, 300);
        assert_eq!(summary.channel_id_native, 0x3412); // Swapped bytes
        assert_eq!(summary.required_slots, 5); // 300 / 64 = 4.68 -> 5 slots
        assert_eq!(summary.next_pow2_buffer_capacity, 512); // Next power of 2 after 300
        assert!(summary.has_odd_parity_flags); // 3 bits set -> odd
    }

    #[test]
    fn test_invalid_zero_payload() {
        assert_eq!(parse_and_align_packet(0, 64), None);
    }
}