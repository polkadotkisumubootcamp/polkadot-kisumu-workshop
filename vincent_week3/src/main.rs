use std::collections::HashMap;
use std::io;

struct Guest {
    name: String,
    notes: String,
}

struct GuestTracker {
    guests: HashMap<String, Guest>,
}

impl GuestTracker {
    fn new() -> Self {
        Self {
            guests: HashMap::new(),
        }
    }

    fn add_guest(&mut self, name: String, notes: String) {
        let guest = Guest {
            name: name.clone(),
            notes,
        };
        self.guests.insert(name, guest);
    }

    fn greet_guest(&self, name: &str) -> Option<String> {
        self.guests.get(name).map(|g| format!("Hello, {}!", g.name))
    }

    fn update_notes(&mut self, name: &str, notes: String) -> bool {
        if let Some(guest) = self.guests.get_mut(name) {
            guest.notes = notes;
            true
        } else {
            false
        }
    }

    fn list_guests(&self) {
        if self.guests.is_empty() {
            println!("No guests available.");
        } else {
            for guest in self.guests.values() {
                println!("{}: {}", guest.name, guest.notes);
            }
        }
    }

    fn get_name(&self, name: &str) -> Option<&str> {
        self.guests.get(name).map(|g| g.name.as_str())
    }
}

fn get_input(prompt: &str) -> String {
    print!("{} ", prompt);
    // Flush to ensure prompt prints before input
    use std::io::Write;
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn main() {
    let mut tracker = GuestTracker::new();

    loop {
        println!("\n--- Guest Tracker Menu ---");
        println!("1. Add Guest");
        println!("2. Greet Guest");
        println!("3. Update Guest Notes");
        println!("4. List All Guests");
        println!("5. Get Guest Name");
        println!("6. Exit");

        match get_input("Enter your choice:").as_str() {
            "1" => {
                let name = get_input("Guest name:");
                let notes = get_input("Guest notes:");
                tracker.add_guest(name, notes);
            }
            "2" => {
                let name = get_input("Guest name to greet:");
                match tracker.greet_guest(&name) {
                    Some(msg) => println!("{}", msg),
                    None => println!("Guest not found."),
                }
            }
            "3" => {
                let name = get_input("Guest name to update:");
                let notes = get_input("New notes:");
                if tracker.update_notes(&name, notes) {
                    println!("Notes updated.");
                } else {
                    println!("Guest not found.");
                }
            }
            "4" => tracker.list_guests(),
            "5" => {
                let name = get_input("Enter guest name to search:");
                match tracker.get_name(&name) {
                    Some(name) => println!("Found guest: {}", name),
                    None => println!("Guest not found."),
                }
            }
            "6" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}
