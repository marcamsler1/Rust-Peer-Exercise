fn main() {
    let current = TrafficLight::Green;
    action(current);
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn action(light: TrafficLight) {
    match light {
        TrafficLight::Red => println!("Stop!"),
        TrafficLight::Yellow => println!("Slow down..."),
        TrafficLight::Green => println!("Go!"),
    }
}