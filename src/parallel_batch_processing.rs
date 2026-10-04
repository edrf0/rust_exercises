use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

pub fn parallel_sum_of_squares(data: &[u64], num_workers: usize) -> Result<u64, String> {
    // Wrap vector into Arc to safely share read access across threads
    let shared_data = Arc::new(data.to_vec());
    let mut handles: Vec<JoinHandle<u64>> = Vec::new();

    // Calculate size of each chunk
    let chunk_size = (shared_data.len() + num_workers - 1) / num_workers;

    for worker_id in 0..num_workers {
        let start_index = worker_id * chunk_size;
        let end_index = (start_index + chunk_size).min(shared_data.len());

        let shared_data_clone = Arc::clone(&shared_data);
        let handle = thread::spawn(move || {
            shared_data_clone[start_index..end_index].iter().map(|&x| x * x).sum::<u64>()
        });

        handles.push(handle);
    }
    let mut sum = 0u64;
    for handle in handles {
        match handle.join() {
            Ok(result) => sum += result,
            Err(e) => return Err(format!("Thread panicked: {:?}", e)),
        }
    }
    Ok(sum)
}

fn main() {
    let numbers: Vec<u64> = (1..=1000).collect(); // 1, 2, ..., 1000

    // Total sum of squares from 1..=1000 is 333,833,500
    let total = parallel_sum_of_squares(&numbers, 4);
    assert_eq!(total, Ok(333_833_500));

    let small_numbers = vec![1, 2, 3, 4, 5]; // 1 + 4 + 9 + 16 + 25 = 55
    let small_total = parallel_sum_of_squares(&small_numbers, 2);
    assert_eq!(small_total, Ok(55));

    println!("Success! Parallel batch worker completed successfully.");
}