use colored::*; // For colorful terminal output
use std::io;     // For handling user input

// Welcome message shown when the program starts
const WELCOME: &str = "
╔══════════════════════╗
║   Rust Calculator    ║
╚══════════════════════╝
";

fn main() {
    // Print the welcome message
    println!("{}", WELCOME.bright_cyan().bold());

    // Get the numbers only once
    let num1 = get_number("Enter first number ➤");
    let num2 = get_number("Enter second number ➤");

    // Loop for multiple operations until the user exits
    loop {
        // Show the operation menu
        println!("\n{}", "╔══════════════════════╗".bright_blue());
        println!("{}", "║   Choose Operation   ║".bright_blue().bold());
        println!("{}", "╚══════════════════════╝".bright_blue());
        println!("{}", "1. Add".bright_green().bold());
        println!("{}", "2. Subtract".bright_magenta().bold());
        println!("{}", "3. Multiply".bright_yellow().bold());
        println!("{}", "4. Divide".bright_cyan().bold());
        println!("{}", "5. Exit".bright_red().bold()); // Exit option

        // Get user choice
        let choice = get_choice();

        // Match and perform the operation
        match choice {
            1 => show_result(num1, num2, "+", num1 + num2, "bright_green"),
            2 => show_result(num1, num2, "-", num1 - num2, "bright_magenta"),
            3 => show_result(num1, num2, "×", num1 * num2, "bright_yellow"),
            4 => {
                if num2 != 0.0 {
                    show_result(num1, num2, "÷", num1 / num2, "bright_cyan")
                } else {
                    println!("\n{}", "✖ Error: Division by zero!".white().on_red().bold());
                }
            }
            5 => {
                println!("\n{}", "👋 Goodbye!".bright_white().on_bright_black());
                break; // Exit the loop
            }
            _ => println!("{}", "⚠ Invalid choice!".bright_red().bold()),
        }
    }
}


// Displays the result of the operation in a styled format
fn show_result(a: f64, b: f64, op: &str, result: f64, color: &str) {
    println!("\n{}", "╔══════════════════════╗".bright_blue());
    println!("{}", "║       Result        ║".bright_blue().bold());
    println!("{}", "╚══════════════════════╝".bright_blue());
    
    // Set color for the result based on operation type
    let colored_result = match color {
        "bright_green" => result.to_string().bright_green().bold(),
        "bright_magenta" => result.to_string().bright_magenta().bold(),
        "bright_yellow" => result.to_string().bright_yellow().bold(),
        "bright_cyan" => result.to_string().bright_cyan().bold(),
        _ => result.to_string().white().bold(),
    };
    
    // Print final styled result
    println!(" {} {} {} = {}",
        a.to_string().bright_white().bold(),
        op.bright_white().bold(),
        b.to_string().bright_white().bold(),
        colored_result
    );
}

// Repeatedly prompts the user until a valid number is entered
fn get_number(prompt: &str) -> f64 {
    loop {
        println!("\n{} {}", "▸".bright_blue(), prompt.bright_white());
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        
        // Try to convert input to a number
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("{}", "Please enter a valid number".bright_red()),
        }
    }
}

// Repeatedly prompts the user until a valid choice between 1 and 4 is entered
fn get_choice() -> u32 {
    loop {
        println!("\n{} {}", "▸".bright_blue(), "Enter choice (1-5)".bright_white());
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        
        match input.trim().parse() {
            Ok(1..=5) => return input.trim().parse().unwrap(),
            _ => println!("{}", "Please enter 1, 2, 3, 4, or 5".bright_red()),
        }
    }
}

