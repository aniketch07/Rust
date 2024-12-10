// Function to test
fn add_two(x: i32) -> i32 {
    x + 2
}
fn main(){
    println!("add:{}",add_two(1));
}

#[cfg(test)]
mod tests {
    // Import the outer scope (the function being tested)
    use super::*;

    #[test]
    fn test_add_two() {
        // Test case to check if the function adds 2 correctly
        assert_eq!(add_two(2), 4); // Should pass, because 2 + 2 = 4
        assert_eq!(add_two(-2), 0); // Should pass, because -2 + 2 = 0
        assert_eq!(add_two(0), 2);  // Should pass, because 0 + 2 = 2
    }
}
