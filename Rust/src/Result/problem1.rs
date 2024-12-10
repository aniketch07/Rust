//Write a function safe_divide(a: f64, b: f64) -> Result<f64, String>
// that returns the result of division or an error message if b is zero.

fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 { // Compare to 0.0 directly
        Err("Cannot divide by 0".to_string()) // Return an error
    } else {
        Ok(a / b) // Return success
    }
}

fn main() {
    let ans = safe_divide(2.0, 0.0).expect(""); // Use 2.0 and 0.0 for f64 values
    println!("ans:{:?}",ans);

    // match ans {
    //     Ok(v) => println!("Result: {}", v),
    //     Err(e) => println!("Error: {}", e),
    // };
}
