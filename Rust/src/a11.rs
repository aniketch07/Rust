struct Item{
    quantity:i32,
    id:i32,
}

fn displayQuantity(item:&Item){
    println!("quantity :{}",item.quantity);
}

fn displayId(item:&Item){
    println!("id :{}",item.id);
}

fn main(){
    let sugar=Item{
        quantity:2,
        id:409,
    };

    displayQuantity(&sugar);
    displayId(&sugar);

    
}