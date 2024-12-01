// Enum with Complex Data (Real-World Example)
// Problem: Create an enum Event with variants:
// Meeting { time: String, location: String }
// Conference { name: String, speakers: Vec<String> }
// Holiday { name: String, date: String }
// Write a function describe_event that takes an Event and prints a description based on the variant. 
//For Meeting, print the time and location; for Conference, print the name and the list of speakers; for Holiday, print the name and date.

enum Event{
    Meeting { time: String, location: String },
    Conference { name: String, speakers: Vec<String> },
    Holiday { name: String, date: String }
}

fn describe_event(event:Event){

}