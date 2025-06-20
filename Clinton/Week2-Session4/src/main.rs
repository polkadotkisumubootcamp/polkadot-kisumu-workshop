use std::io;

// A function that borrows a guest name and greets them
fn greet_guest(name: &String) {
    println!("Hello, {}! Welcome to the event 🎉", name);
}

// A function that returns a borrowed guest name (for lifetime practice)
fn get_first_guest<'a>(guests: &'a Vec<String>) -> Option<&'a String> {
    guests.get(0)
}

fn main() {
    let mut guests: Vec<String> = Vec::new();

    println!("--- Guest Tracker App ---");

    loop {
        println!("\n1. Add Guest");
        println!("2. Greet Guests");
        println!("3. Edit Guest Name");
        println!("4. Show First Guest");
        println!("5. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => {
                println!("Enter guest name:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                guests.push(name.trim().to_string()); // Ownership happens here
            }
            "2" => {
                for name in &guests {
                    greet_guest(name); // Borrowing guest name
                }
            }
            "3" => {
                println!("Enter index of guest to edit:");
                let mut index_input = String::new();
                io::stdin().read_line(&mut index_input).unwrap();

                if let Ok(index) = index_input.trim().parse::<usize>() {
                    if let Some(name) = guests.get_mut(index) { // Mutable borrow
                        println!("Enter new name:");
                        let mut new_name = String::new();
                        io::stdin().read_line(&mut new_name).unwrap();
                        *name = new_name.trim().to_string();
                    } else {
                        println!("No guest at that index.");
                    }
                } else {
                    println!("Invalid input.");
                }
            }
            "4" => {
                if let Some(name) = get_first_guest(&guests) {
                    println!("First guest is: {}", name);
                } else {
                    println!("Guest list is empty.");
                }
            }
            "5" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Try again."),
        }

        // NOTE: At the end of each loop, any temporary values go out of scope and are freed
    }

    // When main ends, the guest list goes out of scope and Rust frees memory (ownership ends here)
}