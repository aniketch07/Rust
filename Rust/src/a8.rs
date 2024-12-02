
enum Flavor{
    Sweet,
    Sour,
    Fruity,
}

struct Drink{
    flavor:Flavor,
    ounce:f32,
}

fn display_drink(drink:Drink){
    match drink.flavor{
        Flavor::Sweet=>println!("Sweet"),
        Flavor::Sour=>println!("Sour"),
        Flavor::Fruity=>println!("Fruity"),
    }
    println!("oz:{}",drink.ounce);
}

fn main(){
    let drink=Drink{
        flavor:Flavor::Sweet,
        ounce:3.2,
    };

    display_drink(drink);
}
