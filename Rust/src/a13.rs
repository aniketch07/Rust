fn main(){
    let myVec=vec![10,20,30,40];

    for num in &myVec{
        if *num==30{
            println!("thrity");
        }
        else{
            println!("{}",num);
        }
    }

    println!("total elements inside the vector:{}",myVec.len());
}