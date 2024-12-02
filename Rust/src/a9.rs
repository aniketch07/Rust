fn returntup()->(i32,i32){
    let coords=(2,4);
    return  coords
}

fn main(){
    let (x,y)=returntup();

    if y>5{
        println!(">5");
    }
    else if y<5{
        println!("<5");
    }
    else{
        println!("=5");
    }
}