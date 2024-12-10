use std::collections::HashMap;
fn main(){
    let mut myPeople=HashMap::new();
    myPeople.insert(1,"Aniket");
    myPeople.insert(2,"Madhav");
    myPeople.insert(3,"Manvi");
    myPeople.insert(4,"Stuti");

    for(id,people) in myPeople.iter(){
        println!("person:{} id:{}",people,id);
    }

    for id in myPeople.keys(){
        println!(" id:{}",id);    
    }
    
    
    for people in myPeople.values(){
        println!("person:{} ",people);    
    }

    for(id,people) in myPeople.iter(){
        println!("person:{:?} id:{:?}",people,id);
    }

    for id in myPeople.keys(){
        println!(" id:{:?}",id);    
    }
    
    
    for people in myPeople.values(){
        println!("person:{:?} ",people);    
    }
}