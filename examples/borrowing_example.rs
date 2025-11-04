fn greet(guest: &String) {
    println!("Welcome, {}!", guest);
}

fn main() {
    let guest = String::from("Bob");
    greet(&guest); // borrow guest
    println!("{} is still staying!", guest); // ✅ guest still valid
}