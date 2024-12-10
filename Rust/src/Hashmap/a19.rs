use std::collections::HashMap;

fn main(){
    let mut data=HashMap::new();

    data.insert("Chairs",5);
    data.insert("Beds",3);
    data.insert("Tables",2);
    data.insert("Couches",0);

    let mut total=0;

    for (name,&items) in data.iter(){
        total=total+items;
        if items!=0{
            println!("name:{} value:{}",name,items);
        }
        else{
            println!("Out of Stock");
        }
    }
    println!("Total items in stock:{}",total);
}