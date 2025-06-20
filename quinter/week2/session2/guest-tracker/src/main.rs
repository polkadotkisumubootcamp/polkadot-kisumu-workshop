// main.rs

// Import the guest module which contains the Guest struct definition
mod guest;
// Import the guest_operations module which contains utility functions
mod guest_operations;

// Import standard library modules we'll need
use std::io;
// Bring Guest struct into scope from our guest module
use guest::Guest;
// Bring utility functions into scope from guest_operations
use guest_operations::{greet_all_guests, display_guest_details, find_guest_by_name};

// Main entry point of the program
fn main() {
    // Create a vector to store guests - this will own all the Guest structs
    let mut guest_list: Vec<Guest> = Vec::new();
    
    // Print welcome message and available commands
    println!("Welcome to the Guest Tracker!");
    println!("Commands: add, greet, find, note, list, quit");
    
    // Main program loop - runs until user enters "quit"
    loop {
        // Prompt for command
        println!("\nEnter command:");
        let mut input = String::new();
        // Read user input from stdin
        io::stdin().read_line(&mut input).expect("Failed to read input");
        // Clean and normalize the input
        let command = input.trim().to_lowercase();
        
        // Match against possible commands
        match command.as_str() {
            // Handle "add" command to create new guest
            "add" => {
                println!("Enter guest name:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read name");
                // Create owned String from input
                let name = name.trim().to_string();
                
                // Create new Guest instance (takes ownership of name)
                let guest = Guest::new(name);
                println!("Added guest: {}", guest.get_name());
                
                // Add guest to our list (transfers ownership to the vector)
                guest_list.push(guest);
            }
            
            // Handle "greet" command to greet all guests
            "greet" => {
                // Call greet function with immutable reference to guest list
                greet_all_guests(&guest_list);
            }
            
            // Handle "find" command to search for a guest
            "find" => {
                println!("Enter name to find:");
                let mut search_name = String::new();
                io::stdin().read_line(&mut search_name).expect("Failed to read name");
                let search_name = search_name.trim();
                
                // Call find function with immutable reference to guest list
                match find_guest_by_name(&guest_list, search_name) {
                    Some(found_name) => println!("Found guest: {}", found_name),
                    None => println!("Guest not found."),
                }
            }
            
            // Handle "note" command to add notes to a guest
            "note" => {
                println!("Enter guest name to add note:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read name");
                let name = name.trim();
                
                // Flag to track if we found the guest
                let mut found = false;
                // Iterate with mutable references to guests
                for guest in &mut guest_list {
                    if guest.get_name() == name {
                        println!("Enter note for {}:", guest.get_name());
                        let mut note = String::new();
                        io::stdin().read_line(&mut note).expect("Failed to read note");
                        let note = note.trim().to_string();
                        
                        // Add note to guest (requires mutable access)
                        guest.add_note(&note);
                        println!("Note added successfully!");
                        found = true;
                        break;
                    }
                }
                
                if !found {
                    println!("Guest not found.");
                }
            }
            
            // Handle "list" command to show all guests
            "list" => {
                // Call display function with immutable reference
                display_guest_details(&guest_list);
            }
            
            // Handle "quit" command to exit program
            "quit" => {
                println!("Goodbye!");
                break;
            }
            
            // Handle unknown commands
            _ => {
                println!("Unknown command. Use: add, greet, find, note, list, quit");
            }
        }
    }
    // When main() ends, guest_list goes out of scope and all memory is automatically freed
}