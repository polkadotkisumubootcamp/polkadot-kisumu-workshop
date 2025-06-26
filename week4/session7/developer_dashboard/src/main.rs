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
}
