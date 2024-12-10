use std::collections::HashMap;

#[derive(Debug)]
struct Locker{
    content:String,
}

fn main(){
    let mut locker=HashMap::new();

    locker.insert(1,Locker{content:"this is aniket's locker".to_string()});
    locker.insert(2,Locker{content:"this is madhav's locker".to_string()});
    locker.insert(3,Locker{content:"this is aditi's locker".to_string()});

    for(number,content) in locker{
        println!("Locker number:{} Content:{:?}",number,content);
    }

}
