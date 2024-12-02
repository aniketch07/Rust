struct Locker{
    name:String,
    assignmet:Option<i32>,
}

fn main(){
    let student1=Locker{
        name:"Aniket".to_string(),
        assignmet:None,
    };

    let student2=Locker{
        name:"Madhav".to_string(),
        assignmet:Some(12),
    };

    println!("name:{}",student1.name);

    match student1.assignmet{
        Some(num)=>println!("locker number:{}",num),
        Nome=>println!("No locker assigned"),
    };

    println!("name:{}",student1.name);
    match student2.assignmet{
        Some(num)=>println!("locker number:{}",num),
        Nome=>println!("No locker assigned"),
    };
}