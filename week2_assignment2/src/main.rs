// Build a tiny “Guest Tracker” app (console-based)

// Ask for guest names and store them (practice
// ownership).

// Let parts of your code borrow guest names to
// greet them (practice borrowing).

// Ensure only one part edits guest details at a
// time (mutable borrow).

// Add a note (comment) in code about where
// scope ends and memory is freed.

// Add a function that returns a borrowed guest
// name (practice lifetimes).

use std::io;
fn main() {
   println!("\n\t\t🌟🌟Welcome to guest tracker🤝🎉\n");
   println!("Kindly input your name below 👇👇");
    let mut guest_name = String::new();

    io::stdin().read_line(&mut guest_name).expect("Failed to read input");
    let guest_name = guest_name.trim(); // Remove any trailing newline or spaces
    // println!("\nHello, {}! Welcome to our guest tracker app!", guest_name);
    greet_guest(guest_name);

}

fn greet_guest(guest: &str) {
    println!("\nHello, {}! Welcome to our guest tracker app 🤝 \nIt's great to see you here! 🎉\n", guest);
}
