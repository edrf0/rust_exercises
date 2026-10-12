use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Product {
    pub name: String,
    pub price: f64,
}

/// 1. Group a vector of products into a HashMap keyed by category string.
pub fn group_by_category(products: Vec<(String, Product)>) -> HashMap<String, Vec<Product>> {
    products.into_iter()
        .fold(HashMap::new(), |mut acc, (category, product)| {
            acc.entry(category).or_insert_with(Vec::new).push(product);
            acc
        })
}

/// 2. Add a single product to a specific category in the catalog using the Entry API.
pub fn add_product_to_category(catalog: &mut HashMap<String, Vec<Product>>, category: &str, product: Product) {
    catalog.entry(category.to_string()).or_insert_with(Vec::new).push(product);
}

/// 3. Multiply the price of all products in a given category by `multiplier` (e.g., 0.9 for 10% off).
pub fn apply_discount_to_category(catalog: &mut HashMap<String, Vec<Product>>, category: &str, multiplier: f64) {
    catalog.entry(category.to_string()).and_modify(|v| {
        v.iter_mut().for_each(|product| product.price *= multiplier)
    });
}

/// 4. Remove any category entries that contain an empty vector of products.
pub fn remove_empty_categories(catalog: &mut HashMap<String, Vec<Product>>) {
    catalog.retain(|_, v| !v.is_empty());
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_catalog() {
        let items = vec![
            ("Electronics".to_string(), Product { name: "Laptop".to_string(), price: 1000.0 }),
            ("Electronics".to_string(), Product { name: "Mouse".to_string(), price: 50.0 }),
            ("Books".to_string(), Product { name: "Rust Programming".to_string(), price: 40.0 }),
        ];

        let mut catalog = group_by_category(items);
        assert_eq!(catalog.get("Electronics").unwrap().len(), 2);
        assert_eq!(catalog.get("Books").unwrap().len(), 1);

        add_product_to_category(
            &mut catalog,
            "Books",
            Product { name: "Algorithms".to_string(), price: 60.0 },
        );
        assert_eq!(catalog.get("Books").unwrap().len(), 2);

        apply_discount_to_category(&mut catalog, "Electronics", 0.8);
        let laptop = catalog.get("Electronics")
            .unwrap().iter().find(|p| p.name == "Laptop").unwrap();
        assert_eq!(laptop.price, 800.0);

        catalog.insert("EmptyCat".to_string(), vec![]);
        remove_empty_categories(&mut catalog);
        assert!(!catalog.contains_key("EmptyCat"));
    }
}