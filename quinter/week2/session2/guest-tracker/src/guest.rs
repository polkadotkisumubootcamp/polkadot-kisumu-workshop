// Represents a guest with name and associated notes
#[derive(Debug, Clone)]  // Enables debug printing and cloning behavior
pub struct Guest {
    name: String,   // Owned string for guest name
    notes: String,  // Owned string for accumulated notes
}

impl Guest {
    // Creates new Guest instance, taking ownership of the name String
    pub fn new(name: String) -> Guest {
        Guest {
            name,  // name is moved here
            notes: String::new()  // Initialize empty notes
        }
    }

    // Returns immutable reference to guest's name (borrows self)
    pub fn get_name(&self) -> &str {
        &self.name  // Reference tied to struct's lifetime
    }

    // Returns immutable reference to guest's notes (borrows self)
    pub fn get_notes(&self) -> &str {
        &self.notes  // Reference tied to struct's lifetime
    }

    // Appends new note to existing notes (requires mutable access)
    pub fn add_note(&mut self, note: &str) {
        if !self.notes.is_empty() {
            self.notes.push_str(", ");  // Add separator for multiple notes
        }
        self.notes.push_str(note);  // Append new note content
    }
}