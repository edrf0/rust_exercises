use std::sync::{Arc, Mutex, mpsc};
use std::thread;

/// 1. Spawn `num_threads` threads. Each thread increments a shared counter
/// `increments_per_thread` times using Arc<Mutex<i32>>. Wait for all threads to finish,
/// then return the final counter value.
pub fn spawn_counter_threads(num_threads: usize, increments_per_thread: usize) -> usize {
    let mut handles = Vec::with_capacity(num_threads);
    let counter = Arc::new(Mutex::new(0));
    for _ in 0..num_threads {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut counter = counter_clone.lock()
                .unwrap_or_else(|err| err.into_inner());
            *counter += increments_per_thread;
        });
        handles.push(handle);
    }
    for handle in handles {
        if let Err(err) = handle.join() {
            eprintln!("{:?}", err);
        }
    }
    *counter
        .lock()
        .unwrap_or_else(|err| err.into_inner())
}

/// 2. Spawn a background thread that sends a vector of numbers over a mpsc channel
/// to the main thread. The main thread receives them and returns them as a Vec<i32>.
pub fn producer_consumer_channel(numbers: Vec<i32>) -> Vec<i32> {
    let (tx, rx) = mpsc::channel();
    let tx_clone = tx.clone();
    let handle = thread::spawn(move || {
        tx_clone.send(numbers)
    });
    drop(tx);
    let result = rx.recv();
    if let Err(_) = handle.join() {
        eprintln!("Error from thread");
    }
    result.unwrap_or_else(|_| Vec::new())
}

/// 1. Use std::thread::scope to sum two halves of a slice concurrently
/// by borrowing local data without Arc.
pub fn scoped_thread_sum(data: &[i32]) -> i32 {
    let mid = data.len() / 2;
    let (left, right) = data.split_at(mid);

    let result = thread::scope(|s| {
        // Spawn a scoped thread for the left half
        let handle_left = s.spawn(|| {
            left.iter().sum::<i32>()
        });

        // Compute or spawn for the right half
        let handle_right = s.spawn(|| {
            right.iter().sum::<i32>()
        });

        handle_left
            .join()
            .unwrap_or(0)
            +
            handle_right
                .join()
                .unwrap_or(0)
    });
    result
}

/// 2. Spawn multiple producer threads that send their ID through cloned senders
/// into a single mpsc channel. The main thread collects and returns them.
pub fn multi_producer_pipeline(num_producers: usize) -> Vec<i32> {
    let (tx, rx) = mpsc::channel();

    for id in 0..num_producers {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            let _ = tx_clone.send(id as i32);
        });
    }

    // Drop the original sender so the receiver knows when all producers are done
    drop(tx);

    rx.iter().collect()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_threads() {
        let result = spawn_counter_threads(5, 100);
        assert_eq!(result, 500);
    }

    #[test]
    fn test_channel_pipeline() {
        let input = vec![10, 20, 30, 40];
        let output = producer_consumer_channel(input.clone());
        assert_eq!(output, input);
    }

    #[test]
    fn test_scoped_thread_sum() {
        let data = vec![10, 20, 30, 40, 50];
        let total = scoped_thread_sum(&data);
        assert_eq!(total, 150);
    }

    #[test]
    fn test_multi_producer() {
        let mut results = multi_producer_pipeline(4);
        results.sort(); // Order from threads can be nondeterministic
        assert_eq!(results, vec![0, 1, 2, 3]);
    }
}