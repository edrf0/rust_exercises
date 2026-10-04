pub fn filter_and_total_category(logs: &[&str], target_category: &str) -> Option<f64> {
    logs.iter()
        .filter_map(|&log| {
            let fields: Vec<&str> = log.split(',').collect();
            if fields.len() != 4 {
                return None;
            }
            let target_category = target_category.to_lowercase();
            let category = fields[1].trim().to_lowercase();
            if category != target_category {
                return None;
            }
            let quantity: f64 = fields[2].trim().parse().ok()?;
            let price: f64 = fields[3].trim().parse().ok()?;

            Some(quantity * price)
        })
        .reduce(|acc, x| acc + x)
}

pub fn get_high_value_order_ids(logs: &[&str], threshold: f64) -> Vec<u32> {
    logs.iter()
        .filter_map(|&log| {
            let fields: Vec<&str> = log.split(',').collect();
            if fields.len() != 4 {
                return None;
            }
            let order_id: u32 = fields[0].trim().parse().ok()?;
            let quantity: f64 = fields[2].trim().parse().ok()?;
            let price: f64 = fields[3].trim().parse().ok()?;
            if quantity * price <= threshold {
                return None;
            }
            Some(order_id)
        })
        .collect()
}

fn main() {
    let raw_logs = vec![
        "101,Electronics,2,299.99",
        "102,Books,1,15.50",
        "103,electronics,1,500.00",
        "104,Clothing,3,25.00",
        "105,Books,4,12.00",
    ];

    // Electronics total: (2 * 299.99 = 599.98) + (1 * 500.00) = 1099.98
    let elec_total = filter_and_total_category(&raw_logs, "electronics");
    assert_eq!(elec_total, Some(1099.98));

    // Non-existent category returns None
    let food_total = filter_and_total_category(&raw_logs, "Food");
    assert_eq!(food_total, None);

    // High value orders (> 100.0): 101 (599.98) and 103 (500.00)
    let high_val = get_high_value_order_ids(&raw_logs, 100.0);
    assert_eq!(high_val, vec![101, 103]);

    println!("Success! Iterator pipeline passed all tests.");
}