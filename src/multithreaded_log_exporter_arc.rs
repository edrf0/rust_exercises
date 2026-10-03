use std::sync::Arc;
use std::thread;

#[derive(Debug, PartialEq)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

pub fn run_exporters(config: Arc<ServerConfig>, worker_ids: Vec<usize>) -> Vec<String> {
    worker_ids.into_iter().map(|id| {
        let config_clone = Arc::clone(&config);
        let handle = thread::spawn(move || {
            format!("Worker {} connected to {}:{}", id, config_clone.host, config_clone.port)
        });
        handle.join().unwrap_or("".to_string())
    }).collect()
}

fn main() {
    let config = Arc::new(ServerConfig {
        host: "127.0.0.1".to_string(),
        port: 8080,
    });

    let worker_ids = vec![1, 2, 3];
    let mut results = run_exporters(Arc::clone(&config), worker_ids);

    results.sort();

    assert_eq!(
        results,
        vec![
            "Worker 1 connected to 127.0.0.1:8080",
            "Worker 2 connected to 127.0.0.1:8080",
            "Worker 3 connected to 127.0.0.1:8080",
        ]
    );

    // Verify main thread still holds the original Arc reference!
    assert_eq!(Arc::strong_count(&config), 1);

    println!("Arc Multi-Threaded Exporter Exercise Passed!");
}