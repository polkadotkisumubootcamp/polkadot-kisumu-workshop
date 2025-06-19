use std::io;

// Struct to represent a guest with name and optional note
struct Guest {
    name: String,
    note: Option<String>,
}

impl Guest {
    // Constructor for Guest
    fn new(name: String) -> Self {
        Guest { name, note: None }
    }

    // Method to add a note (requires mutable borrow)
    fn add_note(&mut self, note: String) {
        self.note = Some(note);
    }
}

// Function to get a guest by index (demonstrates lifetimes)
// The returned reference lives as long as the guests vector
fn get_guest_name<'a>(guests: &'a [Guest], index: usize) -> &'a str {
    &guests[index].name
}

fn main() {
    println!("Welcome to Guest Tracker!");

    let mut guests: Vec<Guest> = Vec::new();

    loop {
        println!("\nMenu:");
        println!("1. Add guest");
        println!("2. List guests");
        println!("3. Add note to guest");
        println!("4. Exit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "1" => {
                println!("Enter guest name:");
                let mut name = String::new();
                io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read line");
                
                // Ownership of name is moved to the Guest struct
                guests.push(Guest::new(name.trim().to_string()));
            }
            "2" => {
                println!("\nCurrent guests:");
                
                // Immutable borrow of guests to print them
                for (i, guest) in guests.iter().enumerate() {
                    // Using our lifetime-annotated function
                    let name = get_guest_name(&guests, i);
                    println!("{}. {}", i + 1, name);
                    
                    if let Some(note) = &guest.note {
                        println!("   Note: {}", note);
                    }
                }
            }

            "3" => {
                println!("Enter guest number to add note:");
                let mut num = String::new();
                io::stdin()
                    .read_line(&mut num)
                    .expect("Failed to read line");
                
                // Trim and parse the input
                match num.trim().parse::<usize>() {
                    Ok(index) if index > 0 && index <= guests.len() => {
                        println!("Enter note for {}:", guests[index-1].name);
                        let mut note = String::new();
                        io::stdin()
                            .read_line(&mut note)
                            .expect("Failed to read line");
                        
                        guests[index-1].add_note(note.trim().to_string());
                    }
                    Ok(_) => {
                        println!("Invalid guest number. Please enter between 1 and {}", guests.len());
                    }
                    Err(_) => {
                        println!("Please enter a valid number");
                    }
                }
            }
            "4" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice, please try again"),
        }
    }

    // Scope ends here - all memory is automatically freed
    // The Vec and all its contents (Strings) will be dropped
    // Rust's ownership system ensures proper cleanup
}