fn main(){
    let value=10;

    let myBool=value>100;
    display(myBool);

}

fn display(myBool:bool){
    match myBool{
        true=>println!(">100"),
        false=>println!("<100"),
    }
}