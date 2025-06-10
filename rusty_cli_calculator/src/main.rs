use std::io;

const BANNER: &str = "\x1b[34m=== Welcome to Rusty CLI Calculator ===\x1b[0m";
const EXIT_OPTION: &str = "4";

fn main() {
    println!("{}", BANNER);
    println!("Enter {} to exit the calculator\n", EXIT_OPTION);
    
    loop {
        match perform_calculation() {
            Ok(should_continue) => {
                if !should_continue {
                    println!("\x1b[36mThank you for using Rusty Calculator! Goodbye!\x1b[0m");
                    break;
                }
            }
            Err(e) => println!("\x1b[31m{}\x1b[0m", e),
        }
        println!();
    }
}

fn perform_calculation() -> Result<bool, String> {
    let num1 = get_number_input("Enter the first number:", EXIT_OPTION)?;
    if let Some(n) = num1 {
        let num2 = get_number_input("Enter the second number:", "")?;
        if let Some(n2) = num2 {
            let operation = get_operation_input()?;
            let result = calculate(n, n2, &operation)?;
            display_result(n, n2, &operation, result);
            Ok(true)
        } else {
            Ok(true) // Continue if second number input failed
        }
    } else {
        Ok(false) // Exit requested
    }
}

fn get_number_input(prompt: &str, exit_option: &str) -> Result<Option<f32>, String> {
    println!("\x1b[33m{}\x1b[0m", prompt);
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .map_err(|_| "Failed to read input")?;
    
    let input = input.trim();
    
    // Check for exit condition
    if !exit_option.is_empty() && input == exit_option {
        return Ok(None);
    }
    
    match input.parse::<f32>() {
        Ok(num) => Ok(Some(num)),
        Err(_) => Err("Invalid number! Please enter a valid number.".to_string()),
    }
}

fn get_operation_input() -> Result<String, String> {
    println!("\x1b[33mEnter operation (+, -, *, /):\x1b[0m");
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .map_err(|_| "Failed to read input")?;
    
    let operation = input.trim().to_string();
    
    match operation.as_str() {
        "+" | "-" | "*" | "/" => Ok(operation),
        _ => Err("Invalid operation! Please use +, -, *, or /.".to_string()),
    }
}

fn calculate(num1: f32, num2: f32, operation: &str) -> Result<f32, String> {
    match operation {
        "+" => Ok(num1 + num2),
        "-" => Ok(num1 - num2),
        "*" => Ok(num1 * num2),
        "/" => {
            if num2 == 0.0 {
                Err("Error: Division by zero is not allowed!".to_string())
            } else {
                Ok(num1 / num2)
            }
        }
        _ => Err("Invalid operation!".to_string()),
    }
}

fn display_result(num1: f32, num2: f32, operation: &str, result: f32) {
    println!("\x1b[32mResult: {} {} {} = {}\x1b[0m", num1, operation, num2, result);
}
