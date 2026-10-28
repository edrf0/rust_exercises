use std::ptr;

pub struct NetworkBitField {
    byte_buffer: *mut u8,
    length: usize,
}

impl NetworkBitField {
    pub fn new(byte_buffer: *mut u8, length: usize) -> Self {
        Self {
            byte_buffer,
            length,
        }
    }
    fn read_write_prelude(&self, bit_position: usize) -> Result<(usize, u8), &'static str> {
        let word_index = bit_position / 8;
        if word_index >= self.length {
            return Err("Bitfield out of range");
        }
        let bit_index = (bit_position % 8) as u8;
        let bit_mask = 1u8 << bit_index;
        Ok((word_index, bit_mask))
    }
    pub fn read_bit(&self, bit_position: usize) -> Result<bool, &'static str> {
        let (word_index, bit_mask) = self.read_write_prelude(bit_position)?;
        let word = unsafe { ptr::read(self.byte_buffer.add(word_index)) };
        Ok(word & bit_mask != 0)
    }
    pub fn write_bit(&self, bit_position: usize, value: bool) -> Result<(), &'static str> {
        let (word_index, bit_mask) = self.read_write_prelude(bit_position)?;
        unsafe {
            let word = ptr::read(self.byte_buffer.add(word_index));
            if value {
                ptr::write(self.byte_buffer.add(word_index), word | bit_mask);
            } else {
                ptr::write(self.byte_buffer.add(word_index), word & !bit_mask);
            }
        }
        Ok(())
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_bit_operations() {
        // Allocate a small buffer of 2 bytes (16 bits)
        let mut buffer = [0u8; 2];
        let bitfield = NetworkBitField::new(buffer.as_mut_ptr(), buffer.len());

        // Verify initial state is all false (0)
        assert_eq!(bitfield.read_bit(0), Ok(false));
        assert_eq!(bitfield.read_bit(15), Ok(false));

        // Set specific bits across different bytes
        assert!(bitfield.write_bit(0, true).is_ok()); // First bit of byte 0
        assert!(bitfield.write_bit(7, true).is_ok()); // Last bit of byte 0
        assert!(bitfield.write_bit(10, true).is_ok()); // Middle bit of byte 1

        // Verify the written values can be read back correctly
        assert_eq!(bitfield.read_bit(0), Ok(true));
        assert_eq!(bitfield.read_bit(1), Ok(false));
        assert_eq!(bitfield.read_bit(7), Ok(true));
        assert_eq!(bitfield.read_bit(10), Ok(true));
        assert_eq!(bitfield.read_bit(11), Ok(false));

        // Clear a bit and verify
        assert!(bitfield.write_bit(0, false).is_ok());
        assert_eq!(bitfield.read_bit(0), Ok(false));
    }

    #[test]
    fn test_out_of_bounds() {
        let mut buffer = [0u8; 1]; // 8 bits total (indices 0 to 7)
        let bitfield = NetworkBitField::new(buffer.as_mut_ptr(), buffer.len());

        // Within bounds
        assert!(bitfield.read_bit(7).is_ok());
        assert!(bitfield.write_bit(7, true).is_ok());

        // Out of bounds
        assert_eq!(bitfield.read_bit(8), Err("Bitfield out of range"));
        assert_eq!(bitfield.write_bit(8, true), Err("Bitfield out of range"));
    }
}