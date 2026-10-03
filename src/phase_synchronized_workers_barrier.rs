use std::sync::{mpsc, Arc, Barrier};
use std::thread;
use std::thread::JoinHandle;

pub fn run_phased_workers(num_workers: usize) -> Vec<String> {
    let (tx, rx) = mpsc::channel::<String>();
    let barrier = Arc::new(Barrier::new(num_workers));
    let mut handles: Vec<JoinHandle<()>> = Vec::with_capacity(num_workers);
    for id in 0..num_workers {
        let barrier_clone = Arc::clone(&barrier);
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            let _ = tx_clone.send(format!("Worker {} completed Phase 1",id));
            barrier_clone.wait();
            let _ = tx_clone.send(format!("Worker {} completed Phase 2",id));
        });
        handles.push(handle);
    }
    for handle in handles {
        if let Err(e) = handle.join() {
            eprintln!("Error in thread: {:?}", e);
        }
    }
    drop(tx);
    rx.into_iter().collect()
}

fn main() {
    let logs = run_phased_workers(3);

    assert_eq!(logs.len(), 6); // 3 workers * 2 phase messages = 6 total
    println!("{:?}", logs);

    println!("Phased Worker Barrier Exercise Passed!");
}