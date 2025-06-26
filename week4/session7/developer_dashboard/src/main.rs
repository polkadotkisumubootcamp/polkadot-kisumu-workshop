fn main() {
      // Create a vector of 6 contributor names
    let mut contributors: Vec<String> = vec![
        "Cynthia".to_string(),
        "Brian".to_string(),
        "Aisha".to_string(),
        "Kelvin".to_string(),
        "Grace".to_string(),
        "Liam".to_string(),
    ];

        // Add a status tag using mutable iteration
    for name in contributors.iter_mut() {
        *name = format!("Active: {}", name);
    }


    // Preview the first 3 contributors using a slice
    println!("First 3 contributors preview:");
    let preview = &contributors[0..3];
    for person in preview {
        println!("{}", person);
    }

     // Display total number of contributors
    println!("\nTotal contributors: {}", contributors.len());

   
    println!("\nContributors before push/pop:");
    print_list(&contributors);

    // Implement Push a new contributor and pop (remove) the last contributor
    contributors.push("Active: Zainab".to_string());

    println!("\nContributors after push ");
    print_list(&contributors);


    contributors.pop();

    println!("\nContributors after pop:");
    print_list(&contributors);
}

// Generic function to print any list
fn print_list<T: std::fmt::Display>(list: &Vec<T>) {
    for item in list {
        println!("{}", item);
    }
}