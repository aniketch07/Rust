enum Ticket{
    Backstage(String,i32),
    Vip(String,i32),
    Standard(i32),
}

fn main(){
    let ticket:Vec<Ticket>=vec![
        Ticket::Backstage("Aniket".to_string(),50),
        Ticket::Vip("Madhav".to_string(),30),
        Ticket::Standard(10),

    ];

    for data in ticket{
        match data{
            Ticket::Backstage(holder,price)=>println!("holder:{} price:{}",holder,price),
            Ticket::Vip(holder,price)=>println!("holder:{} price:{}",holder,price),
            Ticket::Standard(price)=>println!("price:{}",price),
        }
    }
}