struct Survey{
    q1:Option<i32>,
    q2:Option<bool>,
    q3:Option<String>,
}

fn main(){
    let data=Survey{
        q1:Some(12),
        q2:None,
        q3:Some("my name is aniket".to_string()),
    };

    match data.q1{
        Some(v)=>println!("q1 value:{}",v),
        None=>println!("no ans provided for q1"),
    }
    match data.q2{
        Some(v)=>println!("q2 value:{}",v),
        None=>println!("no ans provided for q2"),
    }
    match data.q3{
        Some(v)=>println!("q3 value:{}",v),
        None=>println!("no ans provided for q3"),
    }
}