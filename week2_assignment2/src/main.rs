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
use colored::*;
fn main() {
      println!(
        "\n\t\t{}",
        "🌟🌟Welcome to guest tracker🤝🎉\n".yellow().bold()
    );
   println!("{}", "Kindly input your first name below 👇👇".truecolor(154, 205, 50));
    let mut guest_name = String::new();

    io::stdin().read_line(&mut guest_name).expect("Failed to read input");
    let guest_name = guest_name.trim(); // Remove any trailing newline or spaces
    greet_guest(guest_name);

    println!("{}","\n Kindly input your second name below 👇👇\n".truecolor(154, 205, 50));

    let mut second_name = String::new();
    io::stdin().read_line(&mut second_name).expect("Failed to read input");
    let second_name = second_name.trim(); // Remove any trailing newline or spaces

    full_name(&mut guest_name.to_string(), &second_name.to_string());

}

fn greet_guest(guest: &str) {
    println!("\nHello, {}! Welcome to our guest tracker app 🤝 \nIt's great to see you here! 🎉\n", guest);
}

fn full_name(s_name: &mut String, sec_name: &String) {
    s_name.push_str(" ");
    s_name.push_str(sec_name);
    // The scope of s_name ends here, and memory will be freed when it goes out
    // of scope, which is at the end of this function.
    // sec_name is borrowed and will not be freed until it goes out of scope in the main function.
    println!("\nYour full guest name is: {}", s_name);
}