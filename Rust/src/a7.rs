enum Color{
    red,
    yellow,
    green,
    blue,
}

fn print_color_name(color:Color){
    match color{
        Color::red=>println!("red"),
        Color::yellow=>println!("yellow"),
        Color::green=>println!("green"),
        Color::blue=>println!("blue"),
       
    }
}
fn main(){
    print_color_name(Color::red);
}