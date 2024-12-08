fn get_value() -> Option<i32> {
    let value = Some(10);
    let result = value?;  // Propagates the value inside Some, or None if value is None
    Some(result)
}

fn main() {
    let value = get_value();
    match value {
        Some(v) => println!("Value: {}", v),
        None => println!("No value found"),
    }
}
