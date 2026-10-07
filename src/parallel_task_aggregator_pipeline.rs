use std::sync::mpsc;
use std::thread;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Task {
    pub id: usize,
    pub data: Vec<i64>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TaskResult {
    pub task_id: usize,
    pub sum: i64,
}

pub fn run_worker_pool(tasks: Vec<Task>, num_workers: usize) -> Vec<TaskResult> {
    let (tx, rx) = mpsc::channel();

    // Divide tasks into chunks for each worker
    let chunk_size = (tasks.len() + num_workers - 1).max(1) / num_workers.max(1);
    let task_chunks: Vec<Vec<Task>> = tasks
        .chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect();

    for chunk in task_chunks {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            for task in chunk {
                let sum = task.data.iter().sum();
                let task_result = TaskResult {
                    task_id: task.id,
                    sum,
                };
                tx_clone.send(task_result).unwrap_or(());
            }
        });
    }

    drop(tx);

    rx.into_iter().collect()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_worker_pool_pipeline() {
        let tasks = vec![
            Task { id: 1, data: vec![1, 2, 3] },      // sum: 6
            Task { id: 2, data: vec![10, 20] },       // sum: 30
            Task { id: 3, data: vec![100, 200, 300] },// sum: 600
            Task { id: 4, data: vec![-5, 5, 10] },    // sum: 10
        ];

        let mut results = run_worker_pool(tasks, 2);

        // Sort results by task_id because order across worker threads is non-deterministic!
        results.sort_by_key(|r| r.task_id);

        let expected = vec![
            TaskResult { task_id: 1, sum: 6 },
            TaskResult { task_id: 2, sum: 30 },
            TaskResult { task_id: 3, sum: 600 },
            TaskResult { task_id: 4, sum: 10 },
        ];

        assert_eq!(results, expected);
    }
}