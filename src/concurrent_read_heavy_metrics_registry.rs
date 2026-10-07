use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Clone, Default)]
pub struct MetricsRegistry {
    store: Arc<RwLock<HashMap<String, Vec<f64>>>>,
}

impl MetricsRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_latency(&self, metric: &str, latency_ms: f64) {
        let mut rwlock = self.store
            .write()
            .unwrap_or_else(|e| e.into_inner());
        rwlock
            .entry(metric.to_string())
            .or_insert_with(Vec::new)
            .push(latency_ms);
    }

    pub fn get_average_latency(&self, metric: &str) -> Option<f64> {
        let rwlock = self.store
            .read()
            .unwrap_or_else(|e| e.into_inner());
        match rwlock.get(metric) {
            Some(latency_vector) => {
                if latency_vector.len() == 0 {
                    // Here sum / count would be 0.0 / 0.0
                    return None;
                }
                let sum = latency_vector.iter().sum::<f64>();
                let count_f64 = latency_vector.len() as f64;
                Some(sum / count_f64)
            },
            None => None
        }
    }

    pub fn get_metric_count(&self, metric: &str) -> usize {
        let rwlock = self.store
            .read()
            .unwrap_or_else(|e| e.into_inner());
        match rwlock.get(metric) {
            Some(count) => count.len(),
            None => 0
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_concurrent_metrics_registry() {
        let registry = MetricsRegistry::new();
        let mut handles = vec![];

        // Spawn 8 worker threads recording metrics concurrently
        for i in 0..8 {
            let reg_clone = registry.clone();
            let handle = thread::spawn(move || {
                let latency = (i + 1) as f64 * 10.0; // 10.0, 20.0, ... 80.0
                reg_clone.record_latency("http_request", latency);
                reg_clone.record_latency("db_query", 5.0);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        // Validate write counts
        assert_eq!(registry.get_metric_count("http_request"), 8);
        assert_eq!(registry.get_metric_count("db_query"), 8);

        // Validate average calculation: sum(10..80) / 8 = 360 / 8 = 45.0
        let avg_http = registry.get_average_latency("http_request").unwrap();
        assert!((avg_http - 45.0).abs() < f64::EPSILON);

        let avg_db = registry.get_average_latency("db_query").unwrap();
        assert!((avg_db - 5.0).abs() < f64::EPSILON);

        assert_eq!(registry.get_average_latency("missing_metric"), None);
    }

    #[test]
    fn test_parallel_readers() {
        let registry = MetricsRegistry::new();
        registry.record_latency("cache_hit", 1.5);
        registry.record_latency("cache_hit", 2.5);

        let mut handles = vec![];

        // Spawn 10 parallel reader threads
        for _ in 0..10 {
            let reg_clone = registry.clone();
            let handle = thread::spawn(move || {
                assert_eq!(reg_clone.get_metric_count("cache_hit"), 2);
                assert_eq!(reg_clone.get_average_latency("cache_hit"), Some(2.0));
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}