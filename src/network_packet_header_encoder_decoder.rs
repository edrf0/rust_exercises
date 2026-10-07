#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PacketHeader {
    pub protocol_id: u8,       // 8 bits
    pub flags: u8,             // 4 bits used (0x01 = Encrypted, 0x02 = Compressed, 0x04 = Priority)
    pub sequence_number: u16,  // 16 bits
}

impl PacketHeader {
    pub const FLAG_ENCRYPTED: u8 = 0x01;
    pub const FLAG_COMPRESSED: u8 = 0x02;
    pub const FLAG_PRIORITY: u8 = 0x04;

    pub fn encode(&self) -> u32 {
        (self.protocol_id as u32)
            | (((self.flags & 0x0F) as u32) << 8)
            | ((self.sequence_number as u32) << 16)
    }

    pub fn decode(raw: u32) -> Self {
        Self {
            protocol_id: (raw & 0xFF) as u8,
            flags: ((raw & 0x0F_00) >> 8) as u8,
            sequence_number: ((raw & 0xFF_FF_00_00) >> 16) as u16,
        }
    }

    pub fn is_encrypted(&self) -> bool {
        self.flags & Self::FLAG_ENCRYPTED == Self::FLAG_ENCRYPTED
    }

    pub fn is_compressed(&self) -> bool {
        self.flags & Self::FLAG_COMPRESSED == Self::FLAG_COMPRESSED
    }

    pub fn is_priority(&self) -> bool {
        self.flags & Self::FLAG_PRIORITY == Self::FLAG_PRIORITY
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet_header_bit_manipulation() {
        let header = PacketHeader {
            protocol_id: 42,
            flags: PacketHeader::FLAG_ENCRYPTED | PacketHeader::FLAG_PRIORITY, // 0x01 | 0x04 = 0x05
            sequence_number: 1024,
        };

        // Validate flag checkers
        assert!(header.is_encrypted());
        assert!(!header.is_compressed());
        assert!(header.is_priority());

        // Encode to u32 binary integer
        let encoded = header.encode();

        // Decode back from u32
        let decoded = PacketHeader::decode(encoded);

        assert_eq!(decoded.protocol_id, 42);
        assert_eq!(decoded.flags, 0x05);
        assert_eq!(decoded.sequence_number, 1024);

        assert!(decoded.is_encrypted());
        assert!(!decoded.is_compressed());
        assert!(decoded.is_priority());
    }
}