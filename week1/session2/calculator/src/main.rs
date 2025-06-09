use std::io;

mod calculator;
use calculator::{get_expression, parse_and_calculate, COLORS, WELCOME_BANNER};

fn main() {
    // Display welcome banner when the program ran
    println!("{}{}{}", COLORS.cyan, WELCOME_BANNER, COLORS.reset);
    
    loop {
        // Get expression from user
        let expression = get_expression();
        
        // Parse and calculate result
        match parse_and_calculate(&expression) {
            Ok(result) => {
                // Shadow the result variable for formatting
                let result = format!("{:.6}", result).trim_end_matches('0').trim_end_matches('.').to_string();
                println!("{}Result: {}{}{}", COLORS.green, COLORS.yellow, result, COLORS.reset);
            }
            Err(error) => {
                println!("{}Error: {}{}", COLORS.red, error, COLORS.reset);
            }
        }
        println!("\n{}Do you want to perform another calculation? (y/n): {}", COLORS.blue, COLORS.reset);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        
        // Shadow the input variable to trim it
        let input = input.trim().to_lowercase();
        
        if input != "y" && input != "yes" {
            println!("{}Thanks for using the calculator! Goodbye! 👋{}", COLORS.magenta, COLORS.reset);
            break;
        }
        
        println!(); // Add spacing between calculations
    }
}