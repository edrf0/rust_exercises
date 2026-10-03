#[derive(Debug, PartialEq)]
pub struct EngineConfig {
    pub max_connections: u32,
    pub buffer_size: usize,
}

fn initialize_engine() {
    // 1. Allocate EngineConfig on the heap using Box::new
    let heap_config: Box<EngineConfig> = Box::new(EngineConfig {
        max_connections: 10_000,
        buffer_size: 64_000,
    });

    // 2. Turn heap_config into a &'static EngineConfig reference using Box::leak
    let static_config: &'static EngineConfig = Box::leak(heap_config);

    assert_eq!(static_config.max_connections, 10_000);

    // 3. Create a temporary Box and extract its inner value using Box::into_inner
    let temp_box: Box<u32> = Box::new(42);
    let raw_value: u32 = *temp_box;

    assert_eq!(raw_value, 42);

    println!("Box Methods Exercise Passed!");
}

fn main() {
    initialize_engine();
}