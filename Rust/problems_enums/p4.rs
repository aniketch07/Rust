// Problem: Create an enum Product with variants:
// Book { title: String, price: f32 }
// Electronics { brand: String, price: f32 }
// Implement a function print_product_info that prints the name and price of the product. 
// Use a match expression to handle each variant differently.

enum Product{
    Book{title:String,price:i32},
    Electronics { brand: String, price: i32 },

}
fn print_product_info(product:Product){
    match product{
        Product::Book { title, price } =>println!("Book: '{}' costs ${:.2}", data, price),
        Product::Electronics { brand, price } =>println!("Electronics: '{}' costs ${:.2}", data, price),
    
    }
}

fn main(){
    let book=Product::Book{
        title:"Buddy".to_string(),
        price:24,
    };
    let electronics = Product::Electronics {
        brand: String::from("Samsung"),
        price: 399,
    };

    print_product_info(book);
}