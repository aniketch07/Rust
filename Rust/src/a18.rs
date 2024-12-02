#[derive(Debug)]
struct Adult{
    name:String,
    age:i32,
}

impl Adult{
    fn new(name:&str,age:i32)->Result<Self,&str>{
        if age>=21{
            Ok(Self{
                age,
                name:name.to_string(),
            })
            
        }
        else{
            Err("Age is less than 21")
        }

    }
}
fn main(){
    let person1=Adult::new("aniket",22);
    let person2=Adult::new("stuti",17);

    match person1{
        Ok(person1)=>println!("{} is {} years old ",person1.name,person1.age),
        Err(e)=>println!("{}",e),
    }
    match person2{
        Ok(person2)=>println!("{} is {} years old ",person2.name,person2.age),
        Err(e)=>println!("{}",e),
    }
}