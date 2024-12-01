fn add(num1:i32,num2:i32)->i32{
    num1+num2
}
fn display(num:i32){
    println!("Result:{num}");
}
fn main(){
    display(add(3,4));
}