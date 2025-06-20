// Import the Guest type from the parent module's guest submodule
use crate::guest::Guest;

/// Finds a guest by name in the guest list
/// 
/// # Arguments
/// * `guests` - Immutable reference to vector of Guests (borrows the list)
/// * `search_name` - String slice to search for (borrows the name)
/// 
/// # Returns
/// Option containing borrowed reference to the guest's name if found,
/// or None if no match exists. The reference has same lifetime as input guests.
pub fn find_guest_by_name<'a>(guests: &'a Vec<Guest>, search_name: &str) -> Option<&'a str> {
    for guest in guests {
        if guest.get_name() == search_name {
            // Return borrowed reference to the matched name
            return Some(guest.get_name());
        }
    }
    None  // No match found
}

/// Prints greetings for all guests in the list
/// 
/// # Arguments
/// * `guests` - Immutable reference to vector of Guests (borrows the list)
pub fn greet_all_guests(guests: &Vec<Guest>) {
    println!("\n=== Greeting All Guests ===");
    for guest in guests {
        println!("Hello, {}! Welcome to our event!", guest.get_name());
    }
    if guests.is_empty() {
        println!("No guests to greet yet!");
    }
}

/// Displays detailed information about all guests
/// 
/// # Arguments
/// * `guests` - Immutable reference to vector of Guests (borrows the list)
pub fn display_guest_details(guests: &Vec<Guest>) {
    println!("\n=== Guest Details ===");
    // Enumerate to get index numbers
    for (index, guest) in guests.iter().enumerate() {
        println!("{}. Name: {}", index + 1, guest.get_name());
        // Only show notes if they exist
        if !guest.get_notes().is_empty() {
            println!("   Notes: {}", guest.get_notes());
        }
    }
    if guests.is_empty() {
        println!("No guests registered yet.");
    }
}