// Phone Book
// Implement a phone book application using a HashMap where:
// Names are keys, and phone numbers are values.
// Support adding, updating, and deleting entries.
// Allow searching for a phone number by name.
use std::collections::HashMap;

// Define the PhoneBook structure
struct PhoneBook {
    data: HashMap<String, i32>, // Names are Strings, phone numbers are i32
}

impl PhoneBook {
    // Constructor to create a new PhoneBook
    fn new() -> Self {
        PhoneBook {
            data: HashMap::new(),
        }
    }

    // Add a new entry to the phone book
    fn add_number(&mut self, name: String, number: i32) {
        self.data.insert(name, number);
    }

    // Get a phone number by name
    fn get_number(&self, name: &str) -> Option<&i32> {
        self.data.get(name)
    }

    // Update an existing number
    fn update_number(&mut self, name: String, new_number: i32) -> bool {
        if self.data.contains_key(&name) {
            self.data.insert(name, new_number);
            true
        } else {
            false
        }
    }

    // Delete an entry by name
    fn delete_number(&mut self, name: &str) -> bool {
        self.data.remove(name).is_some()
    }
}

fn main() {
    let mut phone_book = PhoneBook::new();

    // Add some entries to the phone book
    phone_book.add_number("Alice".to_string(), 1234567);
    phone_book.add_number("Bob".to_string(), 98765432);
    phone_book.add_number("Charlie".to_string(), 5555555);

    // Retrieve a phone number
    if let Some(number) = phone_book.get_number("Alice") {
        println!("Alice's phone number: {}", number);
    } else {
        println!("Alice not found");
    }

    // Update a phone number
    if phone_book.update_number("Bob".to_string(), 1111111) {
        println!("Bob's number updated.");
    } else {
        println!("Bob not found.");
    }

    // Delete an entry
    if phone_book.delete_number("Charlie") {
        println!("Charlie removed from the phone book.");
    } else {
        println!("Charlie not found.");
    }

    // Display the remaining entries
    println!("Current phone book: {:?}", phone_book.data);
}
