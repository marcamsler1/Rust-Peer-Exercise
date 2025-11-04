fn main() {
    let mut guest = String::from("Bob");
    rename_guest(&mut guest); // Pass a mutable reference
    println!("Guest after rename: {}", guest); // ✅ guest still valid and updated
}

fn rename_guest(guest: &mut String) {
    guest.push_str(" (VIP)"); // Modify the borrowed string
    println!("Guest renamed inside function: {}", guest);
}