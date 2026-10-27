use std::ops::{Deref, DerefMut};

pub struct InlineBuffer<T: Clone, const N: usize> {
    buffer: [Option<T>; N],
}

impl<T: Clone, const N: usize> InlineBuffer<T, N> {
    pub fn new() -> Self {
        Self {
            buffer: [const {None}; N],
        }
    }
    pub fn with_buffer(buffer: [T; N]) -> Self {
        let mut option_buffer: [Option<T>;N] = [const {None}; N];
        for index in 0..N {
            option_buffer[index] = Some(buffer[index].clone());
        }
        Self {
            buffer: option_buffer
        }
    }
    pub fn with_option_buffer(buffer: [Option<T>; N]) -> Self {
        Self { buffer }
    }
}

impl<T: Clone, const N: usize> Default for InlineBuffer<T, N> {
    fn default() -> Self {
        Self {
            buffer: [const {None}; N],
        }
    }
}

impl<T: Clone, const N: usize> Deref for InlineBuffer<T, N> {
    type Target = [Option<T>; N];
    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

impl<T: Clone, const N: usize> DerefMut for InlineBuffer<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buffer
    }
}

impl<T: Clone, const N: usize> Drop for InlineBuffer<T, N> {
    fn drop(&mut self) {
        for index in 0..N {
            self.buffer[index] = None;
        }
    }
}

fn main() {}

