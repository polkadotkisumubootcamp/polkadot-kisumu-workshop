use std::io;

fn main() {
    // Created a mutable (mut) vector (Vec) to store String values
    // Vec::new() initializes an empty vector that will grow as we add guests
    // This is our "database" to store all guest names
    let mut guests: Vec<String> = Vec::new();

    // Created an infinite loop that keeps asking for names until user types "quit"

    loop{
        println!("Enter your Guest Name (or 'exit' to quit): ");
        let mut name = String::new();
        io::stdin().read_line(&mut name).unwrap();
        let name = name.trim().to_string();

        if name == "exit" || name == "quit"{
            break;
        }
        guests.push(name);
    }
    // &guests creates an immutable borrow - we're reading but not modifying

    println!("\nGuest list:");
    for char in &guests{
        println!("-{}", char);
    }
    if !guests.is_empty(){
        println!("\n Enter index to update (0-{})", guests.len() - 1);
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if let Ok(index) = input.trim().parse::<usize>() {
            println!("Enter new name:");
            let mut new_name = String::new();
            io::stdin().read_line(&mut new_name).unwrap();
            
            if let Some(guest) = guests.get_mut(index) {
                *guest = new_name.trim().to_string();
            }
        }
        println!("\nUpdated guest list:");
        for char in &guests{
            println!("-{}", char);
        }
    }
}