use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock, Condvar};
use std::thread;
use std::time::Duration;

/// 1. Safely read a value from a shared RwLock-protected HashMap.
pub fn read_cache(cache: &RwLock<HashMap<String, String>>, key: &str) -> Option<String> {
    let read_cache = cache
        .read()
        .unwrap_or_else(|e| e.into_inner());
    read_cache.get(key).cloned()
}

/// 2. Safely insert or update a value in a shared RwLock-protected HashMap.
pub fn write_cache(cache: &RwLock<HashMap<String, String>>, key: String, value: String) {
    let mut write_cache = cache
        .write()
        .unwrap_or_else(|e| e.into_inner());
    write_cache.insert(key, value);
}

/// 3. Wait on a Condvar and Mutex pair until the boolean flag becomes true.
pub fn worker_wait(pair: &Arc<(Mutex<bool>, Condvar)>) {
    let (mutex, cvar) = &**pair;
    let mut mutex_guard = mutex
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    while !*mutex_guard {
        mutex_guard = cvar.wait(mutex_guard).unwrap_or_else(|e| e.into_inner());
    }
}

/// 4. Update the boolean flag to true under the Mutex and notify the waiting thread via Condvar.
pub fn trigger_signal(pair: &Arc<(Mutex<bool>, Condvar)>) {
    let (mutex, cvar) = &**pair;
    let mut mutex_guard = mutex
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    *mutex_guard = true;
    cvar.notify_one();
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rwlock_cache() {
        let cache = Arc::new(RwLock::new(HashMap::new()));

        write_cache(&cache, "host".to_string(), "localhost".to_string());
        let val = read_cache(&cache, "host");

        assert_eq!(val, Some("localhost".to_string()));
    }

    #[test]
    fn test_condvar_signaling() {
        let pair = Arc::new((Mutex::new(false), Condvar::new()));
        let pair_clone = Arc::clone(&pair);

        let handle = thread::spawn(move || {
            worker_wait(&pair_clone);
        });

        // Give the worker a moment to enter the wait state
        thread::sleep(Duration::from_millis(200));
        trigger_signal(&pair);

        handle.join().expect("Worker thread panicked");
        println!("Worker thread done");
    }
}