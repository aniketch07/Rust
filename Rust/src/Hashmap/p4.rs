// invert Key-Value Mapping
// Given a HashMap where keys are names and values are ages, 
//invert the mapping so that keys become ages and values become lists of names.

// Example Input:
// {"Alice": 25, "Bob": 30, "Charlie": 25}
// Output:
// {25: ["Alice", "Charlie"], 30: ["Bob"]}
use std::collections::HashMap;
fn main(){
    let input: HashMap<String, u32> = HashMap::from([
        ("Alice".to_string(), 25),
        ("Bob".to_string(), 30),
        ("Charlie".to_string(), 25),
    ]);

    let mut inverted:HashMap<u32,Vec<String>>=HashMap::new();

    for(name,age) in input{
        inverted.entry(age).or_insert(Vec::new()).push(name);
    }

    println!("{:?}", inverted);
}