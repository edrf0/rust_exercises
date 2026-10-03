use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

pub fn track_parallel_requests(counter: Arc<AtomicUsize>, num_threads: usize) {
    let mut handles = Vec::with_capacity(num_threads);
    for _ in 0..num_threads {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            counter_clone.fetch_add(1, Ordering::Relaxed);
        });
        handles.push(handle);
    }
    for handle in handles {
        if let Err(e) = handle.join() {
            eprintln!("Error: {:?}", e);
        }
    }
}

fn main() {
    let counter = Arc::new(AtomicUsize::new(0));

    track_parallel_requests(Arc::clone(&counter), 12);

    // Load final count
    let total_requests = counter.load(Ordering::Relaxed);
    assert_eq!(total_requests, 12);

    println!("Lock-Free Atomic Counter Exercise Passed!");
}