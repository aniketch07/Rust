#[derive(Debug)]
struct Person{
    name:String,
    age:i32,
    color:String,
}

fn main(){
    let people=vec![
        Person{
            name:"Aniket".to_string(),
            age:22,
            color:String::from("Blue"),
        },
        Person{
            name:"Madhav".to_string(),
            age:14,
            color:String::from("Red"),
        },
        Person{
            name:"Wynona".to_string(),
            age:23,
            color:String::from("Blue"),
        },
    ];

    for person in &people{
        if person.age>18{
            println!("{:?}",person);
        }
        
    }
}
