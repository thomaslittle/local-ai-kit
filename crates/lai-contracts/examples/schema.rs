fn main() { println!("{}", serde_json::to_string_pretty(&lai_contracts::schema_document()).unwrap()); }
