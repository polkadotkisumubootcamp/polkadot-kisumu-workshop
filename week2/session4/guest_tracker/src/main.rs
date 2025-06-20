use std::io::{self, Write};

// Function that returns a borrowed guest name using lifetime annotation
fn get_first_guest<'a>(guests: &'a Vec<String>) -> Option<&'a String> {
    guests.get(0)
}

fn main() {
    let mut guests: Vec<String> = Vec::new(); // Ownership of guest list
    
    loop {
        print!("Enter guest name (or 'done' to finish): ");
        io::stdout().flush().unwrap();

        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read line");
        let name = name.trim();

        if name.eq_ignore_ascii_case("done") {
            break;
        }

        // name is moved into the vector (ownership)
        guests.push(name.to_string());
    } // name goes out of scope here and memory is freed

    println!("\nGuest list:");
    for guest in &guests {
        greet_guest(guest); // Immutable borrow
    } // guest goes out of scope here (borrow ends)

    // Mutate a guest name (mutable borrow)
    if let Some(first) = guests.get_mut(0) {
        println!("\nUpdating the first guest's name...");
        *first = "Updated Guest".to_string();
    } // Mutable borrow ends here; safe to use guests againfeat:add

    // Borrow guest list to retrieve first guest (practice lifetime)
    if let Some(first_guest) = get_first_guest(&guests) {
        println!("First guest is: {}", first_guest);
    }

    println!("\nAll guests:");
    for guest in &guests {
        println!("- {}", guest);
    }
}

// Function that borrows guest name
fn greet_guest(name: &String) {
    println!("Hello, {}!", name);
}
