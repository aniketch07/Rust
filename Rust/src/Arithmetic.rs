fn add(a:i32,b:i32)->i32{
    return a+b;
}
fn sub(a:i32,b:i32)->i32{
    return a-b;
}
fn mul(a:i32,b:i32)->i32{
    return a*b;
}
fn div(a:i32,b:i32)->i32{
    return a/b;
}

fn main(){
    println!("the sum of 2 numbers passed: {}",add(2,3));
    println!("the sub of 2 numbers passed: {}",sub(9,3));
    println!("the mul of 2 numbers passed: {}",mul(2,3));
    println!("the div of 2 numbers passed: {}",div(9,3));
}