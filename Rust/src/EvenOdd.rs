fn main(){
   let num=9;
    if(even_odd(num)){
        println!("{num} is even");
    }
    else{
        println!("{num} is Odd");
    }

}

fn even_odd(num:i32)->bool{
    num%2==0
}
      