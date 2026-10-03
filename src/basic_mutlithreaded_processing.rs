use std::thread;

fn run_parallel_work(numbers: Vec<u32>) -> Vec<u32> {
    numbers.into_iter().map(|c| {
        let handle = thread::spawn(move || {
            c * 2
        });
        handle.join().unwrap_or(0)
    }).collect()
}

fn main() {
    let input = vec![10, 20, 30, 40];
    let mut results = run_parallel_work(input);

    // Threads may finish out of order, so sort for comparison
    results.sort();

    assert_eq!(results, vec![20, 40, 60, 80]);
    println!("Standard Library Multithreading Exercise Passed!");
}