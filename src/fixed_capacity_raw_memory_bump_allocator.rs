use std::mem::{align_of, size_of};
use std::ptr;

pub struct BumpAllocator {
    storage: Vec<u8>,
    offset: usize,
}

impl BumpAllocator {
    pub fn new(capacity: usize) -> Self {
        Self {
            storage: vec![0;capacity],
            offset: 0,
        }
    }

    pub fn alloc<T>(&mut self, value: T) -> Option<*mut T> {
        let align = align_of::<T>();
        let size = size_of::<T>();

        let base_ptr = self.storage.as_mut_ptr();
        let current_address = (base_ptr as usize) + self.offset;
        let padding = match current_address % align {
            0 => 0,
            remainder => align - remainder,
        };
        if self.offset + padding + size > self.storage.capacity() {
            return None;
        }
        // SAFETY: The previous if condition verifies that the allocation won't overflow the buffer
        let dest_ptr = unsafe { base_ptr.add(self.offset + padding) as *mut T };
        unsafe { ptr::write(dest_ptr, value); }
        self.offset += padding + size;
        Some(dest_ptr)
    }

    pub fn reset(&mut self) {
        self.offset = 0;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arena() {
        let mut arena = BumpAllocator::new(64);

        // Allocation 1: 1 byte (u8)
        let p1 = arena.alloc::<u8>(0xAB).expect("Allocation 1 failed");

        // Allocation 2: 4 bytes (u32) -> requires 4-byte alignment
        let p2 = arena.alloc::<u32>(0x12345678).expect("Allocation 2 failed");

        // Allocation 3: 8 bytes (u64) -> requires 8-byte alignment
        let p3 = arena.alloc::<u64>(0xDEADBEEFCAFEBABE).expect("Allocation 3 failed");

        // Verify pointer dereferencing and alignment constraints
        unsafe {
            assert_eq!(*p1, 0xAB);
            assert_eq!(*p2, 0x12345678);
            assert_eq!(*p3, 0xDEADBEEFCAFEBABE);

            // Memory addresses must satisfy alignment rules!
            assert_eq!((p2 as usize) % align_of::<u32>(), 0);
            assert_eq!((p3 as usize) % align_of::<u64>(), 0);
        }

        // Reset arena
        arena.reset();

        // Re-allocate after reset
        let p4 = arena.alloc::<u32>(0x99999999).expect("Allocation 4 failed");
        unsafe {
            assert_eq!(*p4, 0x99999999);
        }

        println!("Success! Arena Bump Allocator passed all alignment and bounds tests.");
    }
}