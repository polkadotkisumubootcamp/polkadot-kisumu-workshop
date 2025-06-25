fn main() {
    // 1. Create a vector of contributor names
    let mut contributors: Vec<String> = vec![
        "Clinton".to_string(),
        "Ochieng".to_string(),
        "Polkadot".to_string(),
        "Kisumu".to_string(),
        "Eve".to_string(),
        "Frank".to_string(),
    ];

    println!("📋 Initial Contributor List:");
    print_list(&contributors);

    // 2. Add "✅ Active" tag using mutable iteration
    for name in &mut contributors {
        *name = format!("✅ Active - {}", name);
    }

    // 3. Preview the first 3 contributors using a slice
    println!("\n👀 Previewing first 3 contributors:");
    let preview = &contributors[0..3];
    print_list(preview);

    // 4. Display total contributors using .len()
    println!("\n👥 Total contributors: {}", contributors.len());

    // 5. Simulate adding a new contributor
    contributors.push("✅ Active - Grace".to_string());
    println!("\n➕ After adding Grace:");
    print_list(&contributors);

    // 6. Simulate removing the last contributor
    if let Some(removed) = contributors.pop() {
        println!("\n➖ Removed contributor: {}", removed);
    }

    println!("\n📦 Final Contributor List:");
    print_list(&contributors);
}

// 7. Bonus: Generic function to print any list (Vec<String> or Vec<i32>)
fn print_list<T: std::fmt::Display>(list: &[T]) {
    for item in list {
        println!("- {}", item);
    }
}
