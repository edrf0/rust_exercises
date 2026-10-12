/// 1. Append all elements from `incoming` to `main_list` in-place.
pub fn merge_batches(main_list: &mut Vec<i32>, incoming: &mut Vec<i32>) {
    main_list.append(incoming);
}

/// 2. Remove an item by index in O(1) time using `.swap_remove()`.
pub fn fast_remove(items: &mut Vec<i32>, index: usize) -> Option<i32> {
    if index >= items.len() {
        return None;
    }
    Some(items.swap_remove(index))
}

/// 3. Split the vector at `at_index` using `.split_off()` and return the split-off part.
pub fn split_workload(items: &mut Vec<i32>, at_index: usize) -> Option<Vec<i32>> {
    if at_index >= items.len() {
        return None;
    }
    Some(items.split_off(at_index))
}

/// 4. Compute sliding window sums of size `k` using `.windows()`.
pub fn sliding_window_sums(items: &[i32], k: usize) -> Option<Vec<i32>> {
    if k == 0 || k > items.len() {
        return None;
    }
    let mut sums: Vec<i32> = Vec::with_capacity(items.len() - k + 1);
    for window in items.windows(k) {
        sums.push(window.iter().sum());
    }
    Some(sums)
}

/// 5. Truncate the vector to a maximum length using `.truncate()`.
pub fn trim_excess(items: &mut Vec<i32>, max_len: usize) {
    items.truncate(max_len);
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advanced_vec_operations() {
        let mut main_list = vec![1, 2];
        let mut incoming = vec![3, 4];
        merge_batches(&mut main_list, &mut incoming);
        assert_eq!(main_list, vec![1, 2, 3, 4]);
        assert!(incoming.is_empty());

        let mut items = vec![10, 20, 30, 40];
        let removed = fast_remove(&mut items, 1);
        assert_eq!(removed, Some(20));
        assert_eq!(items.len(), 3);

        let mut work = vec![1, 2, 3, 4, 5];
        let second_half = split_workload(&mut work, 2);
        assert_eq!(work, vec![1, 2]);
        assert_eq!(second_half, Some(vec![3, 4, 5]));

        let data = vec![1, 2, 3, 4];
        let sums = sliding_window_sums(&data, 2);
        assert_eq!(sums, Some(vec![3, 5, 7]));

        let mut list = vec![1, 2, 3, 4, 5];
        trim_excess(&mut list, 2);
        assert_eq!(list, vec![1, 2]);
    }
}