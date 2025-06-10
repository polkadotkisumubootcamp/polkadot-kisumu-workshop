use std::io;

// Constant for welcome banner
const WELCOME_BANNER: &str = "\x1b[1;34m--- Welcome to My Rust Calculator! ---\x1b[0m";

fn main() {
    println!("{}", WELCOME_BANNER);

    // Get first number
    println!("\x1b[1;33mEnter the first number:\x1b[0m");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let input1: f32 = input1.trim().parse().expect("Please enter a valid number"); // Shadowing

    // Get second number
    println!("\x1b[1;33mEnter the second number:\x1b[0m");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let input2: f32 = input2.trim().parse().expect("Please enter a valid number"); // Shadowing

    // Get operation
    println!("\x1b[1;35mEnter operation (+, -, *, /):\x1b[0m");
    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("Failed to read input");
    let operation = operation.trim(); // Shadowing again

    // Perform calculation
    let result = match operation {
        "+" => input1 + input2,
        "-" => input1 - input2,
        "*" => input1 * input2,
        "/" => {
            if input2 == 0.0 {
                println!("\x1b[1;31mError: Division by zero!\x1b[0m");
                return;
            } else {
                input1 / input2
            }
        },
        _ => {
            println!("\x1b[1;31mInvalid operation!\x1b[0m");
            return;
        }
    };

    println!("\x1b[1;32mResult: {} {} {} = {}\x1b[0m", input1, operation, input2, result);
}
