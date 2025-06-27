use std::fmt::Display;

fn main() {
    // 1. Create a vector of 5-7 contributor names
    let mut contributors = vec![
        String::from("Anita"),
        String::from("Clay"),
        String::from("Challoh"),
        String::from("Niver"),
        String::from("Eve"),
        String::from("Frank"),
    ];

    println!("=== Initial Contributor List ===");
    print_list(&contributors);

    // 2. Add status tags using mutable iteration
    for contributor in &mut contributors {
        *contributor = format!("{} ✅ Active", contributor);
    }

    println!("\n=== After Adding Status Tags ===");
    print_list(&contributors);

    // 3. Preview first 3 contributors using a slice
    println!("\nFirst 3 contributors:");
    for contributor in &contributors[0..3] {
        println!("{}", contributor);
    }

    // 4. Display total number of contributors
    println!("\nTotal contributors: {}", contributors.len());

    // 5. Add a new contributor
    contributors.push(String::from("Frank 🆕"));
    println!("\n=== After Adding New Contributor ===");
    print_list(&contributors);

    // 6. Remove the last contributor
    let removed = contributors.pop();
    println!("\nRemoved: {:?}", removed);
    println!("=== After Removing Last Contributor ===");
    print_list(&contributors);

    // Bonus: Test generic function with different types
    let numbers = vec![1, 2, 3];
    println!("\nTesting generic print function with numbers:");
    print_list(&numbers);
}

// Bonus: Generic function to print any list
fn print_list<T: Display>(list: &[T]) {
    for (i, item) in list.iter().enumerate() {
        println!("{}. {}", i + 1, item);
    }
}