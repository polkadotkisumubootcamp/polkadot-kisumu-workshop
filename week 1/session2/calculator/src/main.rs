use std::io;

// ANSI color codes
const RESET: &str = "\x1b[0m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const CYAN: &str = "\x1b[36m";
const BANNER: &str = "\x1b[35m--- Welcome to the Rust Calculator ---\x1b[0m";

fn main() {
    println!("{BANNER}");

    // Read first number
    println!("{CYAN}Enter the first number:{RESET}");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num1: f32 = input.trim().parse().expect("Please enter a valid number");

    // Shadow input variable
    input = String::new();

    // Read second number
    println!("{CYAN}Enter the second number:{RESET}");
    io::stdin().read_line(&mut input).unwrap();
    let num2: f32 = input.trim().parse().expect("Please enter a valid number");

    // Shadow input again
    input = String::new();

    // Read operation
    println!("{CYAN}Enter operation (+, -, *, /):{RESET}");
    io::stdin().read_line(&mut input).unwrap();
    let op = input.trim();

    // Perform calculation
    let result = match op {
        "+" => Some(num1 + num2),
        "-" => Some(num1 - num2),
        "*" => Some(num1 * num2),
        "/" => {
            if num2 == 0.0 {
                None
            } else {
                Some(num1 / num2)
            }
        }
        _ => {
            println!("{RED}Invalid operation!{RESET}");
            return;
        }
    };

    // Display result
    match result {
        Some(val) => println!("{GREEN}Result: {val}{RESET}"),
        None => println!("{RED}Error: Division by zero!{RESET}"),
    }
}
