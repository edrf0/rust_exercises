use std::alloc::{alloc, Layout, LayoutError, dealloc};
use core::ptr;

#[derive(Debug)]
pub enum MemoryError {
    LayoutErr(LayoutError),
    NullPointerErr,
}

#[derive(Debug)]
pub struct UnsafeBuffer<T> {
    buffer: *mut T,
    len: usize,
    capacity: usize,
    layout: Layout,
}

impl<T> UnsafeBuffer<T> {
    pub fn new() -> Self {
        Self {
            buffer: ptr::null_mut(),
            len: 0,
            capacity: 0,
            layout: Layout::new::<T>(),
        }
    }

    fn realloc(&mut self) -> Result<(), MemoryError> {
        let new_capacity = if self.capacity > 0 {self.capacity * 2} else {1};
        let new_layout = Layout::array::<T>(new_capacity).map_err(MemoryError::LayoutErr)?;
        let new_buffer_ptr = unsafe { alloc(new_layout) };
        if new_buffer_ptr.is_null() {
            return Err(MemoryError::NullPointerErr);
        }
        let new_buffer_ptr = new_buffer_ptr as *mut T;
        if !self.buffer.is_null() {
            // Copy old buffer contents to new buffer
            for index in 0..self.len {
                unsafe {
                    ptr::write(new_buffer_ptr.add(index), ptr::read(self.buffer.add(index)));
                }
            }
            // Dealloc old buffer
            unsafe {
                dealloc(self.buffer as *mut u8, self.layout);
            }
        }
        self.buffer = new_buffer_ptr;
        self.capacity = new_capacity;
        self.layout = new_layout;
        Ok(())
    }

    pub fn push(&mut self, item: T) -> Result<(), MemoryError> {
        if self.len == self.capacity {
            if let Err(e) = self.realloc() {
                return Err(e);
            }
        }
        unsafe {
            ptr::write(self.buffer.add(self.len), item);
        }
        self.len += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        Some(unsafe { ptr::read(self.buffer.add(self.len)) })
    }

    pub fn clear(&mut self) {
        while self.pop().is_some() {}
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }
        Some(unsafe { &*self.buffer.add(index) })
    }
}

impl<T> Drop for UnsafeBuffer<T> {
    fn drop(&mut self) {
        if !self.buffer.is_null() {
            self.clear();
            unsafe {
                dealloc(self.buffer as *mut u8, self.layout);
            }
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_buffer() {
        let buf = UnsafeBuffer::<usize>::new();
        assert_eq!(buf.len(), 0);
        assert_eq!(buf.capacity(), 0);
        assert!(buf.get(0).is_none());
    }

    #[test]
    fn test_push_and_get() {
        let mut buf = UnsafeBuffer::<usize>::new();
        assert!(buf.push(10).is_ok());
        assert!(buf.push(20).is_ok());
        assert!(buf.push(30).is_ok());

        assert_eq!(buf.len(), 3);
        assert_eq!(buf.get(0), Some(&10));
        assert_eq!(buf.get(1), Some(&20));
        assert_eq!(buf.get(2), Some(&30));
        assert_eq!(buf.get(3), None);
    }

    #[test]
    fn test_pop() {
        let mut buf = UnsafeBuffer::<usize>::new();
        buf.push(1).unwrap();
        buf.push(2).unwrap();

        assert_eq!(buf.pop(), Some(2));
        assert_eq!(buf.len(), 1);
        assert_eq!(buf.pop(), Some(1));
        assert_eq!(buf.len(), 0);
        assert_eq!(buf.pop(), None);
    }

    #[test]
    fn test_capacity_growth() {
        let mut buf = UnsafeBuffer::<usize>::new();
        for i in 0..100 {
            buf.push(i).unwrap();
        }

        assert_eq!(buf.len(), 100);
        assert!(buf.capacity() >= 100);

        for i in 0..100 {
            assert_eq!(buf.get(i), Some(&i));
        }
    }

    #[test]
    fn test_clear() {
        let mut buf = UnsafeBuffer::<usize>::new();
        buf.push(5).unwrap();
        buf.push(10).unwrap();

        buf.clear();
        assert_eq!(buf.len(), 0);
        assert_eq!(buf.pop(), None);
    }
}