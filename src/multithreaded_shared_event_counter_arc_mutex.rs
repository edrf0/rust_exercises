use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

#[derive(Debug, Default, PartialEq)]
pub struct MetricCollector {
    pub total_events: usize,
    pub batches_processed: usize,
}

pub fn process_events(collector: Arc<Mutex<MetricCollector>>, event_batches: Vec<usize>) {
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    for event_batch in event_batches {
        let collector_clone = Arc::clone(&collector);
        let handle = thread::spawn(move || {
            let mut collector_guard =
                collector_clone.lock().unwrap_or_else(|e| e.into_inner());
            collector_guard.total_events += event_batch;
            collector_guard.batches_processed += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        if let Err(e) = handle.join() {
            eprintln!("Failed to join thread: {:?}", e);
        }
    }
}

fn main() {
    let collector = Arc::new(Mutex::new(MetricCollector::default()));
    let batches = vec![100, 250, 500, 150];

    process_events(Arc::clone(&collector), batches);

    let final_metrics = collector.lock().unwrap();
    assert_eq!(final_metrics.total_events, 1000);
    assert_eq!(final_metrics.batches_processed, 4);

    println!("Arc<Mutex<T>> Exercise Passed!");
}