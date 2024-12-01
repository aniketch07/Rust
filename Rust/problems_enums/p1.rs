//Problem: Define an enum Animal with variants Dog(String) and Cat(String) (both variants should hold a name). 
//In the main function, create an instance for each animal and print their names using a match expression.
enum Animal{
    Dog(String),
    Cat(String),
}
fn main(){
    let my_dog = Animal::Dog("Buddy".to_string()); 
    let my_cat = Animal::Cat("Whiskers".to_string());  

    match my_dog {
        Animal::Dog(name) => println!("The dog's name is {}", name),
        Animal::Cat(name) => println!("The cat's name is {}", name),
    }
    match my_cat {
        Animal::Dog(name) => println!("The dog's name is {}", name),
        Animal::Cat(name) => println!("The cat's name is {}", name),
    }

}