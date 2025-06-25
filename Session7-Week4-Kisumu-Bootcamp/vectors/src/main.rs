
//  Basic Vector Usage with Indexing, Push, Pop, Len, and Iteration

fn main() {
    // 1. Creating a vector that stores integers . The Vec::new() means we are creating an empty list
    let mut numbers: Vec<i32> = Vec::new();

    // 2. Adding (pushing) values into the vector
    numbers.push(10);
    numbers.push(20);
    numbers.push(30);

    // 3. Accessing vector elements using indexing
    println!("The first number is: {}", numbers[0]); // Indexing starts at 0

    // 4. Checking the length of the vector
    println!("The vector has {} items.", numbers.len());

    // 5. Iterating through the vector (immutable borrow)
    println!("All numbers in the vector:");
    for num in numbers.iter() {
        println!("{}", num);
    }

    // 6. Removing the last item from the vector using pop
    // For beginners, we just pop without storing the removed item
    numbers.pop(); // We don't care what was removed for now

    // 7. Showing the vector after popping
    println!("Vector after popping:");
    for num in &numbers {
        println!("{}", num);
    }
}  

// Mutable Iteration and Upd ating Vector Items
fn main() {
    // 1. Creating a vector of Strings instead of &str
    let mut names = vec![
        String::from("Alice"),
        String::from("Bob"),
        String::from("Charlie"),
    ];

    // 2. Iterating mutably to change names . 
    for name in names.iter_mut() {
        // Update the owned String directly
        *name = format!("{} - updated", name);
    }

    // 3. Printing updated names
    println!("Updated names:");
    for name in &names {
        println!("{}", name);
    }
}

