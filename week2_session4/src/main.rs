use std::io;

// Function that returns a borrowed guest name (lifetime annotation)
fn get_guest<'a>(guests: &'a Vec<String>, index: usize) -> Option<&'a String> {
    guests.get(index)
}


fn main() {
    let mut guests: Vec<String> = Vec::new();

    loop {
        println!("Enter guest name (or type 'done' to finish):");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let name = input.trim().to_string();

        if name.eq_ignore_ascii_case("done") {
            break;
        }

        // Ownership of `name` is moved into the vector
        guests.push(name);
    }

    println!("\nGuests entered:");

    // Immutable borrow to greet guests (borrowing)
    for guest in &guests {
        println!("Hello, {}!", guest);
    }

    // Mutable borrow to edit a guest's name
    println!("\nWould you like to edit a guest's name? (yes/no)");

    let mut response = String::new();
    io::stdin().read_line(&mut response).expect("Failed to read input");

    if response.trim().eq_ignore_ascii_case("yes") {
        println!("Enter the index of the guest to edit (starting from 0):");

        let mut index_input = String::new();
        io::stdin().read_line(&mut index_input).expect("Failed to read input");

        if let Ok(index) = index_input.trim().parse::<usize>() {
            if index < guests.len() {
                println!("Enter new name for {}:", guests[index]);

                let mut new_name = String::new();
                io::stdin().read_line(&mut new_name).expect("Failed to read input");

                // Mutable borrow to modify guest name
                if let Some(name_ref) = guests.get_mut(index) {
                    *name_ref = new_name.trim().to_string();
                }
            } else {
                println!("Invalid index.");
            }
        } else {
            println!("Invalid input.");
        }
    }

    // Scope ends here, so memory for inputs is freed
    // The guests vector will be cleaned up when `main` ends

    println!("\nFinal guest list:");
    for (i, guest) in guests.iter().enumerate() {
        println!("{}. {}", i + 1, guest);
    }

    // Borrowing guest name with lifetime
    println!("\nFetching a guest using lifetime example:");
    if let Some(guest) = get_guest(&guests, 0) {
        println!("First guest is: {}", guest);
    } else {
        println!("No guests found.");
    }
}
