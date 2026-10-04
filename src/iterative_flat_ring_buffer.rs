#[derive(Debug)]
pub struct RingBuffer {
    data: Vec<i32>,
    capacity: usize,
    head: usize, // Index of the oldest element (where we pop)
    tail: usize, // Index where the next insert will go (where we push)
    size: usize, // Current count of elements
}

impl RingBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: vec![0; capacity],
            capacity,
            head: 0,
            tail: 0,
            size: 0,
        }
    }

    pub fn push(&mut self, item: i32) {
        self.data[self.tail] = item;
        // Advance using modulo wrapping
        self.tail = (self.tail + 1) % self.capacity;
        if self.size < self.capacity {
            self.size += 1;
        } else if self.size == self.capacity { // Overwrite occurred
            // Advance using modulo wrapping
            self.head = (self.head + 1) % self.capacity;
        }
    }

    pub fn pop(&mut self) -> Option<i32> {
        if self.size == 0 {
            return None;
        }
        // Retrieve value
        let item = self.data[self.head];
        // Advance using modulo wrapping
        self.head = (self.head + 1) % self.capacity;
        // Actual popping by ignoring last ring buffer value
        self.size -= 1;
        Some(item)
    }
}

fn main() {
    let mut buffer = RingBuffer::new(3);

    buffer.push(1);
    buffer.push(2);
    buffer.push(3);

    // Buffer is full [1, 2, 3]. Pushing a 4th overwrites the oldest element (1).
    buffer.push(4);

    assert_eq!(buffer.pop(), Some(2));
    assert_eq!(buffer.pop(), Some(3));
    assert_eq!(buffer.pop(), Some(4));
    assert_eq!(buffer.pop(), None);

    println!("Success! Ring buffer works.");
}