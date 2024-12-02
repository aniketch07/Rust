//Define a structure Rectangle that has width and height as fields.
//Implement a method area that calculates the area of the rectangle.

struct Rectangle{
    width:i32,
    height:i32,
}

fn area(rect:Rectangle)->i32{
    rect.width*rect.height
}

fn main(){
    let rect=Rectangle{
        width:20,
        height:10,
    };

    println!("Area of rect :{}",area(rect));
}