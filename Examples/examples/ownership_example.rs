fn main() {
    let guest = String::from("Alice");
    let room = guest;           // ownership moves from guest to room
    // println!("{}", guest);   // ❌ Error: guest no longer owns the data
    println!("{}", room);       // ✅ room is now the owner
}