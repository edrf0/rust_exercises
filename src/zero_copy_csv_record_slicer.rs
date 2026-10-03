#[derive(Debug, PartialEq)]
pub struct CsvRecord<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub category: &'a str,
}

fn parse_csv_record(line: &str) -> Option<CsvRecord<'_>> {
    let tokens: Vec<&str> = line
        .split(',')
        .map(|token| token.trim())
        .filter(|token| !token.is_empty())
        .collect();
    if tokens.len() < 3 {
        return None;
    }
    Some(CsvRecord {
        id: tokens[0],
        name: tokens[1],
        category: tokens[2],
    })
}

fn main() {
    let csv_data = " 101 , Alice Smith , Engineering ";

    let record = parse_csv_record(csv_data);

    assert_eq!(
        record,
        Some(CsvRecord {
            id: "101",
            name: "Alice Smith",
            category: "Engineering",
        })
    );

    // Verify lifetime safety: record holds references pointing into csv_data!
    println!("Zero-copy record parsed: {:?}", record.unwrap());
}