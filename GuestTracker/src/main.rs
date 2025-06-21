use std::io;

// Function to greet a guest (borrows name immutably)
fn greet_guest(name: &String) {
    println!("Hello, {}! Welcome to the event!", name);
}

// Function to edit a guest's name (mutable borrow)
fn edit_guest(name: &mut String) {
    name.push_str(" Jr.");
}

// Function that returns a borrowed name (practices lifetimes)
fn get_first_guest<'a>(guests: &'a Vec<String>) -> &'a String {
    &guests[0]
}

fn main() {
    let mut guest_list: Vec<String> = Vec::new();

    println!("Enter guest names (type 'done' to finish):");

    loop {
        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read input");

        let name = name.trim().to_string();

        if name.to_lowercase() == "done" {
            break;
        }

        // Ownership of 'name' moves into the vector
        guest_list.push(name);
    }

    // Borrowing names immutably to greet each guest
    println!("\n--- Greeting Guests ---");
    for guest in &guest_list {
        greet_guest(guest);
    }

    // Mutably borrow the first guest to modify their name
    if let Some(first_guest) = guest_list.get_mut(0) {
        edit_guest(first_guest);
        println!("\nEdited first guest's name: {}", first_guest);
    }

    // Borrowing with lifetime: returning a borrowed guest name
    if !guest_list.is_empty() {
        let first = get_first_guest(&guest_list);
        println!("First guest (borrowed with lifetime): {}", first);
    }

    // Scope ends here -> all memory is automatically freed
}
