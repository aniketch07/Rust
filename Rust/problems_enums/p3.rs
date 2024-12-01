enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn display_message(tl: TrafficLight) {
    match tl {
        TrafficLight::Red => println!("Stop"),          // Changed to "Stop"
        TrafficLight::Yellow => println!("Get Ready"),  // Changed to "Get Ready"
        TrafficLight::Green => println!("Go"),          // Changed to "Go"
    }
}

fn main() {
    display_message(TrafficLight::Green); // Will print "Go"
    display_message(TrafficLight::Red);   // Will print "Stop"
    display_message(TrafficLight::Yellow);// Will print "Get Ready"
}
