pub struct RingBuffer<T, const N: usize> {
    data: [Option<T>; N],
    head: usize,
    tail: usize,
    len: usize,
}

impl<T, const N: usize> RingBuffer<T, N> {
    pub fn new() -> Self {
        Self {
            data: [const {None}; N],
            head: 0,
            tail: 0,
            len: 0,
        }
    }
    pub fn push_back(&mut self, item: T) -> Result<(), T> {
        if self.len == N {
            return Err(item);
        }
        self.data[self.tail] = Some(item);
        self.tail = (self.tail + 1) % N;
        self.len += 1;
        Ok(())
    }
    pub fn pop_front(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        match self.data[self.head].take() {
            Some(item) => {
                self.head = (self.head + 1) % N;
                self.len = self.len.saturating_sub(1);
                Some(item)
            }
            None => {
                // Move forward until Some(T) or tail index is reached
                loop {
                    self.head = (self.head + 1) % N;
                    self.len = self.len.saturating_sub(1);
                    if self.data[self.head].is_some() || self.head == self.tail {
                        break;
                    }
                }
                None
            }
        }
    }
    pub fn is_empty(&self) -> bool {
        if self.len == 0 {
            return true;
        }
        false
    }
    pub fn is_full(&self) -> bool {
        if self.len == N {
            return true;
        }
        false
    }
}

impl<T, const N: usize> IntoIterator for RingBuffer<T, N> {
    type Item = Option<T>;
    type IntoIter = core::array::IntoIter<Self::Item, N>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_buffer() {
        let rb: RingBuffer<i32, 3> = RingBuffer::new();
        assert!(rb.is_empty());
        assert!(!rb.is_full());
    }

    #[test]
    fn test_push_and_pop_fifo() {
        let mut rb = RingBuffer::<i32, 3>::new();

        // Push elements until full
        assert_eq!(rb.push_back(1), Ok(()));
        assert_eq!(rb.push_back(2), Ok(()));
        assert!(!rb.is_empty());
        assert!(!rb.is_full());

        assert_eq!(rb.push_back(3), Ok(()));
        assert!(rb.is_full());

        // Pushing when full should return Err(item)
        assert_eq!(rb.push_back(4), Err(4));

        // Pop elements in FIFO order
        assert_eq!(rb.pop_front(), Some(1));
        assert_eq!(rb.pop_front(), Some(2));
        assert!(!rb.is_full());
        assert!(!rb.is_empty());

        assert_eq!(rb.pop_front(), Some(3));
        assert!(rb.is_empty());
        assert_eq!(rb.pop_front(), None);
    }

    #[test]
    fn test_wrapping_behavior() {
        let mut rb = RingBuffer::<i32, 3>::new();

        // Fill it up
        assert_eq!(rb.push_back(10), Ok(()));
        assert_eq!(rb.push_back(20), Ok(()));
        assert_eq!(rb.push_back(30), Ok(()));

        // Empty half of it
        assert_eq!(rb.pop_front(), Some(10));
        assert_eq!(rb.pop_front(), Some(20));

        // Push more to trigger wrap-around of the tail index
        assert_eq!(rb.push_back(40), Ok(()));
        assert_eq!(rb.push_back(50), Ok(()));
        assert!(rb.is_full());

        // Verify elements come out in correct order post-wrap
        assert_eq!(rb.pop_front(), Some(30));
        assert_eq!(rb.pop_front(), Some(40));
        assert_eq!(rb.pop_front(), Some(50));
        assert!(rb.is_empty());
    }
}