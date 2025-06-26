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
}
