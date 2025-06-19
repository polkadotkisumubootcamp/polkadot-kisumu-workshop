use std::io::{self, Write};

// Main guest tracker structure
struct GuestTracker {
    guests: Vec<String>,
}

impl GuestTracker {
    fn add_guest(&mut self, name: String) {
        println!("✅ Added guest: {}", name);
        self.guests.push(name);
    }

    fn greet_guests(&self) {
        println!("\n👋 Greeting all guests:");
        for guest in &self.guests {
            println!("Hello, {}!", guest); // Immutable borrow
        }
    }

    fn edit_guest(&mut self, index: usize, new_name: String) {
        if let Some(guest) = self.guests.get_mut(index) {
            println!("\n✏️ Changing {} to {}", guest, new_name);
            *guest = new_name; // Mutable borrow
        } else {
            println!("❌ Invalid guest index!");
        }
    }

    fn find_longest_name<'a>(&'a self) -> &'a str {
        self.guests
            .iter()
            .max_by_key(|name| name.len())
            .map(|s| s.as_str())
            .unwrap_or("No guests yet")
    }
}

fn main() {
    println!("🎉 Welcome to Guest Tracker!");

    {
        let mut tracker = GuestTracker { guests: Vec::new() };

        // Adding guests (transferring ownership)
        loop {
            print!("\nEnter guest name (or 'done' to finish): ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let name = input.trim().to_string();

            if name.eq_ignore_ascii_case("done") {
                break;
            }

            tracker.add_guest(name); // Ownership transferred here
        }

        tracker.greet_guests();

        if !tracker.guests.is_empty() {
            print!(
                "\nEnter index of guest to edit (0-{}): ",
                tracker.guests.len() - 1
            );
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            if let Ok(index) = input.trim().parse::<usize>() {
                print!("Enter new name: ");
                io::stdout().flush().unwrap();

                let mut new_name = String::new();
                io::stdin()
                    .read_line(&mut new_name)
                    .expect("Failed to read line");

                tracker.edit_guest(index, new_name.trim().to_string());
            } else {
                println!("❌ Invalid index!");
            }

            tracker.greet_guests();
        }

        let longest = tracker.find_longest_name();
        println!(
            "\n🏆 Longest name: {} ({} characters)",
            longest,
            longest.len()
        );

        println!("\n🛑 Ending this guest tracking session...");
    }
    println!("\n✨ All guest data has been cleared. Goodbye!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_name() {
        let mut tracker = GuestTracker { guests: Vec::new() };
        tracker.add_guest("Alice".to_string());
        tracker.add_guest("Bob".to_string());
        tracker.add_guest("Christopher".to_string());

        assert_eq!(tracker.find_longest_name(), "Christopher");
    }
}
