// Generic function to print any list
fn print_list<T: std::fmt::Display>(list: &[T]) {
    for (i, item) in list.iter().enumerate() {
        println!("{}: {}", i + 1, item);
    }
}

fn main() {
    // Create a vector of contributor names
    let mut contributors: Vec<String> = vec![
        "Alice".into(),
        "Bob".into(),
        "Charlie".into(),
        "Diana".into(),
        "Eve".into(),
        "Frank".into(),
        "Grace".into(),
    ];

    println!("Original contributors:");
    print_list(&contributors);

    // Add a status tag to each contributor
    for name in contributors.iter_mut() {
        *name = format!("{} ✅ Active", name);
    }

    println!("\nContributors with status:");
    print_list(&contributors);

    // Preview the first 3 contributors using a slice
    println!("\nFirst 3 contributors:");
    let preview = &contributors[0..3];
    print_list(preview);

    // Display total number of contributors
    println!("\nTotal contributors: {}", contributors.len());

    // Push a new contributor
    println!("\nAdding new contributor...");
    contributors.push("Henry ✅ Active".into());

    println!("\nAfter push:");
    print_list(&contributors);

    // Pop the last contributor
    println!("\nRemoving last contributor...");
    contributors.pop();

    println!("\nAfter pop:");
    print_list(&contributors);
}
