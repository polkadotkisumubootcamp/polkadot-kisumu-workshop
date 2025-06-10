use std::io;

// Bonus: Create a const for a welcome banner
const WELCOME_BANNER: &str = "\x1b[36m\n=== POLKADOT CALCULATOR ===\x1b[0m"; // \x1b[36m is cyan color

fn main() {
    // Print the welcome banner with color
    println!("{}", WELCOME_BANNER);

    loop {
        // Get first number
        let num1 = get_number("Enter first number: ");

        // Get operation
        let operation = get_operation();

        // Get second number (shadowing the previous num2 if it existed)
        let num2 = get_number("Enter second number: ");

        // Perform calculation
        let result = calculate(num1, num2, operation);

        // Print result with color
        println!("\x1b[32mResult: {}\x1b[0m", result); // \x1b[32m is green color

        // Ask if user wants to continue
        if !continue_calculating() {
            break;
        }
    }
}

fn get_number(prompt: &str) -> f32 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Shadowing the input variable by parsing it
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("\x1b[31mPlease enter a valid number!\x1b[0m"), // \x1b[31m is red
        };
    }
}

fn get_operation() -> char {
    loop {
        println!("Enter operation (+, -, *, /): ");
        let mut operation = String::new();
        
        io::stdin()
            .read_line(&mut operation)
            .expect("Failed to read line");

        let operation = operation.trim().chars().next(); // Shadowing operation variable

        match operation {
            Some('+') | Some('-') | Some('*') | Some('/') => return operation.unwrap(),
            _ => println!("\x1b[31mPlease enter a valid operation!\x1b[0m"),
        }
    }
}

fn calculate(num1: f32, num2: f32, operation: char) -> f32 {
    match operation {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => {
            // Handle division by zero
            if num2 == 0.0 {
                println!("\x1b[31mError: Division by zero!\x1b[0m");
                0.0 // Return 0 as a safe default
            } else {
                num1 / num2
            }
        }
        _ => {
            println!("\x1b[31mInvalid operation!\x1b[0m");
            0.0
        }
    }
}

fn continue_calculating() -> bool {
    loop {
        println!("Do you want to perform another calculation? (y/n): ");
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().to_lowercase().as_str() {
            "y" => return true,
            "n" => return false,
            _ => println!("\x1b[31mPlease enter 'y' or 'n'!\x1b[0m"),
        }
    }
}