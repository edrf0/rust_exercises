/// 1. Parse strings to i32, keep even numbers, double them, and collect into a Vec.
pub fn parse_and_double_evens(raw_inputs: &[&str]) -> Vec<i32> {
    raw_inputs.iter()
        .filter_map(|&input| {
            let parsed = input.trim().parse::<i32>().ok()?;
            if parsed % 2 != 0 {
                return None;
            }
            Some(parsed * 2)
        })
        .collect()
}

/// 2. Find the index of the first element satisfying the predicate. Return Option<usize>.
pub fn find_first_matching_index<T, F>(items: &[T], predicate: F) -> Option<usize>
where
    F: Fn(&T) -> bool,
{
    items.iter()
        .position(|item| predicate(item))
}

/// 3. Compute the dot product (sum of products of corresponding elements) of two slices.
pub fn calculate_weighted_sum(values: &[i32], weights: &[i32]) -> i32 {
    values.iter()
        .enumerate()
        .map(|(idx, value)| value * weights[idx])
        .sum::<i32>()
}

/// 1. Flatten a slice of vectors, square each element, and collect into a Vec.
pub fn flatten_and_square(nested: &[Vec<i32>]) -> Vec<i32> {
    nested.iter()
        .flatten()
        .map(|&value| value.pow(2))
        .collect()
}

/// 2. Partition integers into a tuple of (evens, odds) vectors using `.partition()`.
pub fn partition_even_odd(items: &[i32]) -> (Vec<i32>, Vec<i32>) {
    items.iter()
        .partition(|&&value| value % 2 == 0)
}

/// 3. Compute running/cumulative sums using `.scan()`.
pub fn running_sums(items: &[i32]) -> Vec<i32> {
    items.iter()
        .scan(0, |acc, &x| {
            *acc += x;
            Some(*acc)
        })
        .collect()
}

/// 4. Take elements while the running cumulative sum does not exceed `max_sum`.
pub fn take_until_threshold(items: &[i32], max_sum: i32) -> Vec<i32> {
    let mut current_sum = 0;
    items.iter()
        .copied()
        .take_while(|&value| {
            current_sum += value;
            current_sum <= max_sum
        })
        .collect()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_double() {
        let inputs = vec!["1", "2", "invalid", "4", "10"];
        let result = parse_and_double_evens(&inputs);
        // "2" -> 2 -> doubled is 4
        // "4" -> 4 -> doubled is 8
        // "10" -> 10 -> doubled is 20
        assert_eq!(result, vec![4, 8, 20]);
    }

    #[test]
    fn test_find_index() {
        let items = vec![10, 20, 35, 40];
        let idx = find_first_matching_index(&items, |&x| x > 30);
        assert_eq!(idx, Some(2)); // 35 is at index 2
    }

    #[test]
    fn test_weighted_sum() {
        let v = vec![1, 2, 3];
        let w = vec![10, 20, 30];
        // (1*10) + (2*20) + (3*30) = 10 + 40 + 90 = 140
        assert_eq!(calculate_weighted_sum(&v, &w), 140);
    }

    #[test]
    fn test_flatten_and_square() {
        let nested = vec![vec![1, 2], vec![3], vec![4, 5]];
        let result = flatten_and_square(&nested);
        assert_eq!(result, vec![1, 4, 9, 16, 25]);
    }

    #[test]
    fn test_partition_even_odd() {
        let items = vec![1, 2, 3, 4, 5, 6];
        let (evens, odds) = partition_even_odd(&items);
        assert_eq!(evens, vec![2, 4, 6]);
        assert_eq!(odds, vec![1, 3, 5]);
    }

    #[test]
    fn test_running_sums() {
        let items = vec![1, 2, 3, 4];
        let sums = running_sums(&items);
        assert_eq!(sums, vec![1, 3, 6, 10]);
    }

    #[test]
    fn test_take_until_threshold() {
        let items = vec![2, 3, 4, 10, 1];
        // Running sums: 2, 5, 9, 19 (exceeds 10), stop before 10
        let result = take_until_threshold(&items, 10);
        assert_eq!(result, vec![2, 3, 4]);
    }
}