use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;
use std::thread::JoinHandle;

pub fn run_cache_workloads(
    cache: Arc<RwLock<HashMap<String, String>>>,
    reader_keys: Vec<String>,
    writer_entries: Vec<(String, String)>,
) {
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    for reader_key in reader_keys {
        let cache_clone = Arc::clone(&cache);
        let handle = thread::spawn(move || {
            let cache_read_guard =
                cache_clone.read().unwrap_or_else(|e| e.into_inner());
            println!("{}",cache_read_guard.get(&reader_key).unwrap_or(&String::from("")));
        });
        handles.push(handle);
    }
    for writer_key in writer_entries {
        let cache_clone = Arc::clone(&cache);
        let handle = thread::spawn(move || {
            let mut cache_write_guard =
                cache_clone.write().unwrap_or_else(|e| e.into_inner());
            *cache_write_guard.entry(writer_key.0).or_insert(writer_key.1.clone()) = writer_key.1.clone();
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
    let cache = Arc::new(RwLock::new(HashMap::new()));

    // Seed initial cache data
    {
        let mut map = cache.write().unwrap();
        map.insert("session_1".to_string(), "active".to_string());
        map.insert("session_2".to_string(), "idle".to_string());
    }

    let reader_keys = vec!["session_1".to_string(), "session_2".to_string(), "session_3".to_string()];
    let writer_entries = vec![("session_3".to_string(), "active".to_string())];

    run_cache_workloads(Arc::clone(&cache), reader_keys, writer_entries);

    // Verify cache has all 3 entries in the end
    let final_map = cache.read().unwrap();
    assert_eq!(final_map.len(), 3);
    assert_eq!(final_map.get("session_3").unwrap(), "active");

    println!("Arc<RwLock<T>> Exercise Passed!");
}