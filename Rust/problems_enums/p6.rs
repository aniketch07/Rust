// Problem: Define an enum Result<T, E> that mimics the standard library's Result. It should have:

// Ok(T) for success, where T is some type (e.g., an integer).
// Err(E) for errors, where E is some error message (e.g., String).
// Write a function divide that returns a Result<i32, String> based on whether the second number is zero.
//  If it's zero, return an error message. If it's non-zero, return the result of the division as Ok.

enum Result<T, E>{
    Ok(T),
    Err(E),
}

fn divide(num1:i32,num2:i32)->Result<i32,String>{
    if num2==0{
        Result::Err(String::from("Cannot divide by zero"))

    }
    else {
        Result::Ok(num1/num2)
    }
}

fn main(){
    let result=divide(10,2);
    match result{
        Result::Ok(value) => println!("Division result: {}", value),
        Result::Err(e) => println!("Error: {}", e),
    }
}
