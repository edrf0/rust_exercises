use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

pub fn update_global_peak(peak: &AtomicUsize, new_val: usize) {
    let mut current = peak.load(Ordering::Relaxed);
    loop {
        if current >= new_val {
            break;
        }
        match peak.compare_exchange(current, new_val, Ordering::SeqCst, Ordering::Relaxed) {
            Ok(_) => break,
            Err(actual) => current = actual,
        }
    }
}

fn main() {
    let global_peak = Arc::new(AtomicUsize::new(10));
    let mut handles = vec![];

    // Array of candidate values submitted by concurrent threads
    let candidates = vec![15, 8, 42, 23, 99, 50, 12];

    for val in candidates {
        let peak_clone = Arc::clone(&global_peak);
        handles.push(thread::spawn(move || {
            update_global_peak(&peak_clone, val);
        }));
    }

    for handle in handles {
        if let Err(e) = handle.join() {
            eprintln!("Error on updating global peak: {:?}", e);
        }
    }

    // Peak should be 99!
    assert_eq!(global_peak.load(Ordering::Relaxed), 99);

    println!("Lock-Free CAS Loop Exercise Passed!");
}