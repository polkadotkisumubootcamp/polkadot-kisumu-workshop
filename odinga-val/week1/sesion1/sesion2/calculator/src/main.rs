use std::io;

// Bonus: Welcome banner constant
const WELCOME_BANNER: &str = "\x1b[1;36m\n=== RUST CALCULATOR ===\x1b[0m";

fn main() {
    println!("{}", WELCOME_BANNER);
    
    // Get first number
    let num1 = get_number("Enter first number: ");
    
    
    // Get operation
    let op = get_operation();
    
    // Get second number (shadowing the original variable)
    let num1 = get_number("Enter second number: ");
    let num2 = get_number("Enter second number: ");
    // Perform calculation
    let result = match op {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => {
            if num2 == 0.0 {
                println!("\x1b[1;31mError: Division by zero!\x1b[0m");
                return;
            }
            num1 / num2
        }
        _ => {
            println!("\x1b[1;31mInvalid operation!\x1b[0m");
            return;
        }
    };
    
    // Display result with color
    println!("\x1b[1;32mResult: {} {} {} = {:.2}\x1b[0m", num1, op, num2, result);
}

fn get_number(prompt: &str) -> f32 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
            
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("\x1b[1;33mPlease enter a valid number!\x1b[0m"),
        }
    }
}

fn get_operation() -> char {
    loop {
        println!("\nEnter operation (+, -, *, /):");
        let mut op = String::new();
        
        io::stdin()
            .read_line(&mut op)
            .expect("Failed to read line");
            
        let op = op.trim().chars().next();
        
        match op {
            Some('+' | '-' | '*' | '/') => return op.unwrap(),
            _ => println!("\x1b[1;33mPlease enter a valid operation!\x1b[0m"),
        }
    }
}