use std::thread;

fn parallel_sum(data: &[u64]) -> u64 {
    let mid = data.len() / 2;
    let (left, right) = data.split_at(mid);

    thread::scope(|s| {
        // Spawn thread 1 to sum `left`
        let handle_left = s.spawn(|| {
            left.iter().sum::<u64>()
        });

        // Spawn thread 2 to sum `right`
        let handle_right = s.spawn(|| {
            right.iter().sum::<u64>()
        });

        // Join handles and combine results
        handle_left.join().unwrap_or(0) + handle_right.join().unwrap_or(0)
    })
}

fn main() {
    let numbers: Vec<u64> = (1..=100).collect();
    let total = parallel_sum(&numbers);

    assert_eq!(total, 5050); // Sum of 1..=100 is 5050
    println!("Scoped Threads Exercise Passed!");
}