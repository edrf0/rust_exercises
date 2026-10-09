use core::mem;
use core::ptr;
use std::mem::ManuallyDrop;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Slot {
    pub payload: Option<[u8; 8]>,
    pub access_count: usize,
}

impl Slot {
    pub fn take_and_replace(&mut self, new_payload: [u8; 8]) -> Option<[u8; 8]> {
        self.access_count += 1;
        mem::replace(&mut self.payload, Some(new_payload))
    }
}

pub unsafe fn swap_unaligned_blobs(a: *mut u8, b: *mut u8, len: usize) {
    for index in 0..len {
        unsafe {
            let a_ptr = a.add(index);
            let b_ptr = b.add(index);

            let a_byte = ptr::read_unaligned(a_ptr);
            let b_byte = ptr::read_unaligned(b_ptr);

            ptr::write_unaligned(a_ptr, b_byte);
            ptr::write_unaligned(b_ptr, a_byte);
        }
    }
}

pub unsafe fn stash_and_forget(payload: [u8; 8], dst: *mut u8) {
    let payload = ManuallyDrop::new(payload);
    unsafe { ptr::copy_nonoverlapping(payload.as_ptr(), dst, 8); }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slot_replace() {
        let mut slot = Slot {
            payload: Some([1, 2, 3, 4, 5, 6, 7, 8]),
            access_count: 0,
        };

        let old = slot.take_and_replace([8, 7, 6, 5, 4, 3, 2, 1]);
        assert_eq!(old, Some([1, 2, 3, 4, 5, 6, 7, 8]));
        assert_eq!(slot.payload, Some([8, 7, 6, 5, 4, 3, 2, 1]));
        assert_eq!(slot.access_count, 1);
    }

    #[test]
    fn test_swap_unaligned_blobs() {
        let mut buf_a: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
        let mut buf_b: [u8; 4] = [0x11, 0x22, 0x33, 0x44];

        unsafe {
            swap_unaligned_blobs(buf_a.as_mut_ptr(), buf_b.as_mut_ptr(), 4);
        }

        assert_eq!(buf_a, [0x11, 0x22, 0x33, 0x44]);
        assert_eq!(buf_b, [0xAA, 0xBB, 0xCC, 0xDD]);
    }

    #[test]
    fn test_stash_and_forget() {
        let payload: [u8; 8] = [0xDE, 0xAD, 0xBE, 0xEF, 0x00, 0x11, 0x22, 0x33];
        let mut target_buffer = [0u8; 8];

        unsafe {
            stash_and_forget(payload, target_buffer.as_mut_ptr());
        }

        assert_eq!(target_buffer, [0xDE, 0xAD, 0xBE, 0xEF, 0x00, 0x11, 0x22, 0x33]);
    }
}