use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone)]
pub struct RequestTracker {
    counts: HashMap<String, usize>,
}

impl RequestTracker {
    pub fn new() -> Self {
        Self {
            counts: HashMap::new(),
        }
    }

    pub fn record_hit(&mut self, endpoint: &str) {
        *self.counts.entry(endpoint.to_owned()).or_insert(0) += 1;
    }

    pub fn get_hits(&self, endpoint: &str) -> usize {
        *self.counts.get(endpoint).unwrap_or(&0)
    }

    pub fn total_hits(&self) -> usize {
        self.counts.values().sum()
    }
}

fn main() {
    let tracker = Arc::new(Mutex::new(RequestTracker::new()));
    let mut handles = Vec::new();

    let num_workers = match thread::available_parallelism() {
        Ok(n) if n.get() > 2 => n.get(),
        Ok(_) | Err(_) => 2,
    };

    for i in 0..num_workers {
        let tracker_clone = Arc::clone(&tracker);
        let handle = thread::spawn(move || {
            let mut guard = tracker_clone
                .lock()
                .unwrap_or_else(|e| e.into_inner());
            if i % 2 == 0 {
                guard.record_hit("/api/users");
            } else {
                guard.record_hit("/api/posts");
            }
            guard.record_hit("/health");
        });
        handles.push(handle);
    }

    for handle in handles {
        if let Err(e) = handle.join() {
            eprintln!("Thread panicked: {:?}", e);
        }
    }

    let (even_thread_job_count,odd_thread_job_count) = if num_workers % 2 == 0 {
        (num_workers / 2, num_workers / 2)
    } else {
        (num_workers / 2 + 1, num_workers / 2)
    };
    let guard = tracker
        .lock()
        .unwrap_or_else(|e| e.into_inner());

    assert_eq!(guard.get_hits("/api/users"), even_thread_job_count);
    assert_eq!(guard.get_hits("/api/posts"), odd_thread_job_count);
    assert_eq!(guard.get_hits("/health"), num_workers);
    assert_eq!(guard.total_hits(), num_workers + even_thread_job_count + odd_thread_job_count);

    println!("Success! Thread-safe request tracker passed all tests.");
}