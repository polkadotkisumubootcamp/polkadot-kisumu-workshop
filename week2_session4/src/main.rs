use std::io;

// Function that returns a borrowed guest name (lifetime annotation)
fn get_guest<'a>(guests: &'a Vec<String>, index: usize) -> Option<&'a String> {
    guests.get(index)
}
