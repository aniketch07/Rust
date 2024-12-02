enum Color{
    Red,
    Green,
}

impl Color{
    fn print(&self){
        match self{
            Color::Green=>println!("Green"),
            Color::Red=>println!("Red"),
        }
    }
}

struct Dimensions{
    width:f64,
    height:f64,
    depth:f64,
}

impl Dimensions {
    fn print(&self){
        println!("width:{},height:{},weight:{}",self.width,self.height,self.depth);
    }
}
struct Box{
    color:Color,
    weight:f64,
    dimensions:Dimensions,
}

impl Box {
    fn new(color:Color,weight:f64,dimensions:Dimensions)->Self{
        Self{
            color,
            weight,
            dimensions,
        }
    }

    fn print(&self){
        self.color.print();
        self.dimensions.print();
        println!("weight:{}",self.weight)
    }
}

fn main(){
    let small_dia=Dimensions{
        width:2.5,
        height:4.2,
        depth:1.6,
    };

    let small_box=Box::new(Color::Red, 3.5, small_dia);
    small_box.print();
}

