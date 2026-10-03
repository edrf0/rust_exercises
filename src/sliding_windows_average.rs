fn window_averages(data: &[i32; 6]) -> [f64; 4] {
    let mut averages = [0.0; 4];

    // windows(3) gives iterators over 3-element slices: &[10, 20, 30], &[20, 30, 40], ...
    for (i, window) in data.windows(3).enumerate() {
        let sum: i32 = window.iter().sum();
        averages[i] = sum as f64 / 3.0;
    }

    averages
    // let mut current_sum;
    // let mut float64_array_4_windows = [0f64; 4];
    // for start_index in 0..=(data.len() - 3) {
    //     current_sum = 0;
    //     for index in start_index..start_index + 3 {
    //         current_sum += data[index];
    //     }
    //     float64_array_4_windows[start_index] = current_sum as f64 / 3.0;
    // }
    // float64_array_4_windows
}

fn main() {
    let readings = [10, 20, 30, 40, 50, 60];
    let averages = window_averages(&readings);

    assert_eq!(averages, [20.0, 30.0, 40.0, 50.0]);
    println!("Array Exercise Passed!");
}