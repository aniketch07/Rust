// Word Frequency Counter
// Write a program that counts the frequency of each word in a given sentence. 
//Use a HashMap where keys are words and values are their counts.

// Example Input:
// "hello world hello"
// Output:
// {"hello": 2, "world": 1}

use std::collections::HashMap;

fn countWords(input:&str)-> HashMap<String, i32>{

    let mut frequencies = HashMap::new();

    for word in input.split_whitespace() {
        let word = word.to_string();
        *frequencies.entry(word).or_insert(0) += 1;
    }

    frequencies
}

fn main(){
    let sentence="Hello Hello my name is aniket Hello Hello aniket my my my name name is";

    let output=countWords(sentence);
    println!("{:?}",output);
}