# Rust-Peer-Exercise

## Overview
This repository accompanies the Peer Exercise.
It provides short examples and a project structure template to help you understand the key Rust concepts used in the assignment, especially **ownership**, **borrowing**, and **safe mutation**.

The main exercise is to build a **command-line hotel management simulation** in Rust, where each room can be either empty or occupied by a guest. You’ll apply Rust’s ownership and borrowing rules to safely manage the hotel’s state.

---

### 1. Install Rust
Follow the official guide:  
[https://doc.rust-lang.org/book/ch01-01-installation.html](https://doc.rust-lang.org/book/ch01-01-installation.html)

Then verify your installation:

```bash
rustc --version
cargo --version
```

---

## Core Rust Concepts

### 1. Ownership Example
Every value in Rust has exactly one owner. When that owner goes out of scope, the value is freed.

```rust
fn main() {
    let guest = String::from("Alice");
    let room = guest; // ownership moves from guest to room
    // println!("{}", guest); // ❌ Error: guest no longer owns the data
    println!("{}", room); // ✅ room is now the owner
}
```

---

### 2. Borrowing Example
Borrowing allows temporary access to data without transferring ownership.

```rust
fn greet(guest: &String) {
    println!("Welcome, {}!", guest);
}

fn main() {
    let guest = String::from("Bob");
    greet(&guest); // borrow guest
    println!("{} is still staying!", guest); // ✅ guest still valid
}
```

### 3. Mutable Borrowing Example
Mutable borrowing allows temporary write access to data without transferring ownership. Pay attention, only one mutable borrow to a value can exist at a time.

```rust
fn main() {
    let mut guest = String::from("Bob");
    rename_guest(&mut guest); // Pass a mutable reference
    println!("Guest after rename: {}", guest); // ✅ guest still valid and updated
}

fn rename_guest(guest: &mut String) {
    guest.push_str(" (VIP)"); // Modify the borrowed string
    println!("Guest renamed inside function: {}", guest);
}
```

---

### 4. Pattern Matching Example
Rust’s `match` statement is very powerful. It ensures that all possible cases are handled.

```rust
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
```

---

### 5. Simple CLI Input Example
In the example below you can see how to read input from the terminal:

```rust
use std::io;

fn main() {
    println!("Enter your name:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    println!("Hello, {}!", input.trim());
}
```

---

### 5. Cargo Basics
Below you can find some useful **Cargo** commands for this exercise: 

```bash
cargo new memory_hotel      # Create a new project
cargo run                   # Build and run the current project
cargo build                 # Only build the current project
cargo check                 # Check for errors without compiling binaries
cargo fmt                   # Format code
```

---

## Project Template

Here’s the structure that I'd recommend for your project. But feel free to handle things differently.

```
/memory_hotel/
├── Cargo.toml
└── src/
    ├── main.rs     # CLI and command handling
    └── hotel.rs    # Hotel and room logic
```

---

## Example Output

```
Welcome to the Memory Hotel!
> list
Room 0: empty
Room 1: empty
Room 2: empty
Room 3: empty
Room 4: empty
> check-in 1 Alice
Guest checked into room 1
> check-out 1
Alice checked out of room 1
> exit
Goodbye!
```
---


## Author
**Marc Amsler**  
marctheodorfelix.amsler@uzh.ch

