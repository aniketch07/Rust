struct Adult{
    name:String,
    age:i32,
}
impl Adult{
    fn new(name:&str,age:i32)->Result<Self,String>{
        if age>=21{
            Ok(Self{
                name.to_string(),age,
            })
            
        }
        else{
            Err("Age must be atleast 21".to_string())
        }
    }
}