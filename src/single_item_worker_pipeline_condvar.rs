use std::sync::{Arc, Condvar, Mutex};
use std::thread;

pub fn run_condvar_pipeline(payload: String) -> String {
    // Tuple containing Mutex<Option<String>> and Condvar
    let pair = Arc::new((Mutex::new(None), Condvar::new()));
    let pair_clone = Arc::clone(&pair);

    // Spawn Consumer Thread
    let consumer_handle = thread::spawn(move || {
        let (lock, cvar) = &*pair_clone;
        let mut guard = lock.lock().unwrap();
        while guard.is_none() {
            guard = cvar.wait(guard).unwrap();
        }
        let payload = guard.take().unwrap();
        format!("Processed: {}", payload)
    });

    // Producer (Main Thread)
    let (lock, cvar) = &*pair;
    {
        let mut data = lock.lock().unwrap();
        *data = Some(payload); // Set data
    }
    cvar.notify_one(); // Signal waiting consumer thread!

    consumer_handle.join().unwrap_or_else(|e| format!("{:?}", e))
}

fn main() {
    let result = run_condvar_pipeline("Job_#42".to_string());
    assert_eq!(result, "Processed: Job_#42");

    println!("Condvar Worker Exercise Passed!");
}