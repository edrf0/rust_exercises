use core::mem::size_of;
use core::ptr::read_unaligned;

pub fn read_u16_be_at(buffer: &[u8], offset: usize) -> Option<u16> {
    if offset + size_of::<u16>() > buffer.len() {
        return None;
    }
    let two_bytes_buffer_offset: [u8;2];
    // SAFETY: if offset + size_of::<u16>() > buffer.len() prevents buffer overflow
    unsafe {
        let offset_buffer_ptr = buffer.as_ptr().add(offset);
        two_bytes_buffer_offset = read_unaligned(offset_buffer_ptr as *const [u8;2]);
    }
    Some(u16::from_be_bytes(two_bytes_buffer_offset))
}

pub fn read_u32_le_at(buffer: &[u8], offset: usize) -> Option<u32> {
    if offset + size_of::<u32>() > buffer.len() {
        return None;
    }
    let four_bytes_buffer_offset: [u8;4];
    // SAFETY: if offset + size_of::<u32>() > buffer.len() prevents buffer overflow
    unsafe {
        let offset_buffer_ptr = buffer.as_ptr().add(offset);
        four_bytes_buffer_offset = read_unaligned(offset_buffer_ptr as *const [u8;4]);
    }
    Some(u32::from_le_bytes(four_bytes_buffer_offset))
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raw_pointer_unaligned_reads() {
        // Raw byte array containing mixed endian payload
        let raw_bytes: [u8; 8] = [
            0x00,
            0xDE, 0xAD, // u16 Big-Endian at offset 1 -> 0xDEAD (57005)
            0x78, 0x56, 0x34, 0x12, // u32 Little-Endian at offset 3 -> 0x12345678 (305419896)
            0xFF
        ];

        assert_eq!(read_u16_be_at(&raw_bytes, 1), Some(0xDEAD));
        assert_eq!(read_u32_le_at(&raw_bytes, 3), Some(0x12345678));

        // Out of bounds check
        assert_eq!(read_u16_be_at(&raw_bytes, 7), None);
        assert_eq!(read_u32_le_at(&raw_bytes, 5), None);
    }
}