struct Locker{
    name:String,
    locker_assignment:Option<i32>,
}

fn main(){
    let student1:Locker=Locker{
        name:"Aniket".to_string(),
        locker_assignment:Some(10),
    };

    let student2:Locker=Locker{
        name:"Madhav".to_string(),
        locker_assignment:None,
    };

    match student1.locker_assignment{
        Some(v)=>println!("locker number for student {} is :{}",student1.name,v),
        None=>println!("Student {} does not have any locker ",student1.name),
    }

    println!("name:{}",student1.name);
    match student2.locker_assignment{
        Some(v)=>println!("locker number for student {} is :{}",student2.name,v),
        None=>println!("Student {} does not have any locker ",student2.name),
    }
}