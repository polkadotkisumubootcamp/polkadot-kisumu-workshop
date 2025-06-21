use std::io;
use std::collections::HashMap;

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

    // Takes ownership of name and notes
    fn add_guest(&mut self, name: String, notes: String) {
        let guest = Guest { name: name.clone(), notes };
        self.guests.insert(name, guest);
    }

    // Borrows guest name to greet them
    fn greet_guest(&self, name: &str) -> Option<String> {
        self.guests.get(name).map(|g| format!("Hello, {}!", g.name))
    }

    // Only one mutable borrow at a time
    fn update_notes(&mut self, name: &str, notes: String) -> bool {
        match self.guests.get_mut(name) {
            Some(guest) => {
                guest.notes = notes;
                true
            }
            None => false,
        }
    }

    fn list_guests(&self) {
        for guest in self.guests.values() {
            println!("{}: {}", guest.name, guest.notes);
        }
    }

    // Returns borrowed guest name 
    fn get_name(&self, name: &str) -> Option<&str> {
        self.guests.get(name).map(|g| g.name.as_str())
    }
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let mut tracker = GuestTracker::new();

    loop {
        println!("\n1. Add Guest 2. Greet Guest  3. Update Guest 4. List All 5. Get name  6. Exit");
        
        match get_input("Choice:").as_str() {
            "1" => {
                let name = get_input("Name:");
                let notes = get_input("Notes:");
                tracker.add_guest(name, notes);
            }
            "2" => {
                let name = get_input("Name:");
                match tracker.greet_guest(&name) {
                    Some(msg) => println!("{}", msg),
                    None => println!("Not found"),
                }
            }
            "3" => {
                let name = get_input("Name:");
                let notes = get_input("New notes:");
                if !tracker.update_notes(&name, notes) {
                    println!("Not found");
                }
            }
            "4" => tracker.list_guests(),
            "5" => {
                let name = get_input("Name:");
                match tracker.get_name(&name) {
                    Some(n) => println!("Found: {}", n),
                    None => println!("Not found"),
                }
            }
            "6" => break,
            _ => println!("Invalid choice"),
        }
    }
} 