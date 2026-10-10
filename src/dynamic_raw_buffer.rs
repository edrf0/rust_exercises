use std::alloc::{alloc, dealloc, Layout, realloc};
use std::{mem, ptr};
use std::error::Error;

/// A safe RAII wrapper for the raw heap buffer to guarantee deallocation.
pub struct RawBuffer<T> {
    ptr: *mut T,
    layout: Layout,
    length: usize,
    capacity: usize,
}

impl<T> RawBuffer<T> {
    pub fn new(capacity: usize) -> Result<Self, Box<dyn Error>> {
        let layout = Layout::array::<T>(capacity)?;
        if capacity == 0 {
            Ok(Self {
                ptr: ptr::dangling_mut(),
                layout,
                length: 0,
                capacity,
            })
        } else {
            let ptr = unsafe { alloc(layout) as *mut T };
            if ptr.is_null() {
                return Err("Failed to allocate memory".into());
            }
            Ok(Self {
                ptr,
                layout,
                length: 0,
                capacity
            })
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn grow(&mut self, new_capacity: usize) -> Result<(), Box<dyn Error>> {
        if new_capacity <= self.capacity {
            return Err("New capacity must be greater than current capacity".into());
        }
        let new_layout = Layout::array::<T>(new_capacity)?;
        let new_ptr: *mut T;
        unsafe {
            new_ptr = if self.capacity == 0 {
                alloc(new_layout) as *mut T
            } else {
                realloc(self.ptr as *mut u8, self.layout, new_layout.size()) as *mut T
            }
        }
        if new_ptr.is_null() {
            return Err("Failed to allocate memory".into());
        }
        self.ptr = new_ptr;
        self.layout = new_layout;
        self.capacity = new_capacity;
        Ok(())
    }

    pub fn push(&mut self, value: T) -> Result<(), Box<dyn Error>> {
        if self.length == self.capacity {
            let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
            self.grow(new_capacity)?;
        }
        unsafe { ptr::write(self.ptr.add(self.length), value) };
        self.length += 1;
        Ok(())
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.length { return None; }
        Some(unsafe { &*self.ptr.add(index) })
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.length { return None; }
        Some(unsafe { &mut *self.ptr.add(index) })
    }
}

impl<T> Drop for RawBuffer<T> {
    fn drop(&mut self) {
        if mem::needs_drop::<T>() {
            for index in 0..self.length {
                unsafe { ptr::drop_in_place(self.ptr.add(index)) };
            }
        }
        if self.layout.size() > 0 {
            unsafe { dealloc(self.ptr as *mut u8, self.layout) };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocation_and_write_read() {
        let count = 10;
        let mut buffer = RawBuffer::<usize>::new(count).unwrap();
        // Write sequential data
        for i in 0..count {
            buffer.push(i * 100).unwrap();
        }

        // Read and verify data
        for i in 0..count {
            let val = buffer.get(i);
            assert_eq!(*val.unwrap(), i * 100);
        }
    }

    #[test]
    fn test_zero_size_or_edge_layouts() {
        // Layout array with 0 elements should be handled safely
        let layout = Layout::array::<usize>(0).unwrap();
        assert_eq!(layout.size(), 0);
    }

    #[test]
    fn test_drop_with_strings() {
        let count = 3;
        let mut buffer = RawBuffer::<String>::new(count).unwrap();

        buffer.push(String::from("Hello")).unwrap();
        buffer.push(String::from("Rust")).unwrap();
        buffer.push(String::from("Alloc")).unwrap();

        // When `buffer` goes out of scope here, Drop will automatically
        // call drop_in_place on all three Strings, preventing memory leaks!
    }

    #[test]
    fn test_dynamic_growth_and_push() {
        let mut buffer = RawBuffer::<usize>::new(2).unwrap();
        assert_eq!(buffer.capacity(), 2);

        buffer.push(10).unwrap();
        buffer.push(20).unwrap();
        buffer.push(30).unwrap(); // Triggers reallocation (capacity grows to 4)

        assert_eq!(buffer.len(), 3);
        assert_eq!(buffer.capacity(), 4);
        assert_eq!(*buffer.get(0).unwrap(), 10);
        assert_eq!(*buffer.get(1).unwrap(), 20);
        assert_eq!(*buffer.get(2).unwrap(), 30);
    }
}

fn main() {}