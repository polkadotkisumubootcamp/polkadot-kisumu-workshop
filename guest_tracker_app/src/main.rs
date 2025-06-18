use std::io::{self, Write};

const BLUE: &str = "\x1b[34m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const RESET: &str = "\x1b[0m";


struct Guest {
    name: String,
}

fn main() {
    println!("{}\nWelcome to Guest Tracker{}", BLUE, RESET);

    // The vector will the helpful to storing guest data
    let mut guests: Vec<Guest> = Vec::new();

    // The significance of having the loop is to let the application run continuously until the user exits
    loop {
        print!("\n{}Enter guest name to continue or press 'q' to exit:{} ", YELLOW, RESET);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let name = input.trim();
        if name.to_lowercase() == "q" {
            println!("{}\nThank you for guest tracker.{}", BLUE, RESET);
            break;
        }

        // add_guest takes ownership of the guest's name
        add_guest(&mut guests, name.to_string());

        // display_guest does an immutable borrow to display all guests
        display_guests(&guests);

        // Borrowing last guest's name with lifetime
        if let Some(last_name) = get_last_guest_name(&guests) {
            println!("Latest Guest: {}", last_name);
        }
    }
} // The scope of the guests vector ends here dropping all instances.

// Mutable borrowing of guests vector to add new guest
fn add_guest(guests: &mut Vec<Guest>, name: String) {
    print!("\n{}Welcome to the club '{}'.{} ", BLUE, name, RESET);
    guests.push(Guest { name });
}

// Immutable borrowing to display all guests
fn display_guests(guests: &Vec<Guest>) {
    println!("{}Below is a list of current guests:{}", BLUE, RESET);
    for guest in guests {
        println!("{}{}{}", GREEN, guest.name, RESET);
    }
}

// Returns borrowed references with explicit lifetime
fn get_last_guest_name<'a>(guests: &'a Vec<Guest>) -> Option<&'a String> {
    guests.last().map(|g| &g.name)
}
