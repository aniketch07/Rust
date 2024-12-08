enum Option<T>{
    Some(T),
    None,
}

fn main(){
    let value=Option::Some(12);
    let no_value:Option<i32>=Option::None;

    match value{
        Option::Some(v)=>println!("Value is :{}",v),
        Option::None=>println!("None"),
    };

    match no_value{
        Option::Some(v)=>println!("Value is :{}",v),
        Option::None=>println!("None"),
    };
}