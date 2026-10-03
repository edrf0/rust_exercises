use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct DatabaseConfig {
    pub connection_string: String,
    pub timeout_ms: u64,
}

#[derive(Debug, PartialEq)]
pub struct Worker {
    pub id: usize,
    pub config: Rc<DatabaseConfig>,
}

fn create_worker_pool(config: Rc<DatabaseConfig>, worker_count: usize) -> Vec<Worker> {
    (0..worker_count).map(|c| Worker {id: c, config: Rc::clone(&config)}).collect()
}

fn main() {
    let shared_config = Rc::new(DatabaseConfig {
        connection_string: "postgres://admin:secret@localhost:5432/production".to_string(),
        timeout_ms: 5000,
    });

    // Initial reference count should be 1
    assert_eq!(Rc::strong_count(&shared_config), 1);

    let workers = create_worker_pool(Rc::clone(&shared_config), 3);

    // 1 original + 1 passed to function + 3 in vector = 4 strong references total!
    assert_eq!(Rc::strong_count(&shared_config), 4);
    assert_eq!(workers.len(), 3);
    assert_eq!(workers[0].config.timeout_ms, 5000);

    // Drop the worker pool
    drop(workers);

    // Reference count should drop back down to 1!
    assert_eq!(Rc::strong_count(&shared_config), 1);

    println!("Rc Worker Pool Exercise Passed!");
}