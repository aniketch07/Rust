//Write a program that checks if a specific key exists in a HashMap. 
//If it doesn’t, insert the key with a default value.
use std::collections::HashMap;
fn main(){
    let mut map = HashMap::new();

    // Initial data in the HashMap
    map.insert("Alice".to_string(), 90);
    map.insert("Bob".to_string(), 85);

     // Key to check
     let key_to_check = "Charlie";

     if !map.contains_key(key_to_check) {
        println!("Key '{}' does not exist. Adding it with a default value.", key_to_check);
        map.insert(key_to_check.to_string(), 0); // Default value is 0
    } else {
        println!("Key '{}' exists with value {}.", key_to_check, map[key_to_check]);
    }

    println!("Updated map: {:?}", map);
}