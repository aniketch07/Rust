//Problem: Define an enum Shape with the variants Circle(f64) and Rectangle(f64, f64).
//Implement a function area that calculates and returns the area of the shape. 
//If the shape is a Circle, use π * r^2. If it's a Rectangle, return width * height.

enum Shape{
    Circle(f64),
    Rectangle(f64,f64),
}

fn main(){
    let circle=Shape::Circle(2.0);
    let rect=Shape::Rectangle(2.0,2.0);

    match circle{
        Shape::Circle(r) => println!("{}", 3.141 * r * r), 
        Shape::Rectangle(l, b) => println!("{}", l * b), 
    }
    match rect{
        Shape::Circle(r) => println!("{}", 3.141 * r * r), 
        Shape::Rectangle(l, b) => println!("{}", l * b), 
    }
}