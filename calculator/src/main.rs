use std::io;

// ANSI color codes
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const MAGENTA: &str = "\x1b[35m";
const CYAN: &str = "\x1b[36m";
const RESET: &str = "\x1b[0m";

const WELCOME_BANNER: &str = "
    WELCOME TO POLKADOT KISUMU WORKSHOP CALCULATOR
";

fn main() {
    println!("{}{}{}", CYAN, WELCOME_BANNER, RESET);

    // The significance of the infinite loop at this point is to make the calculator run continuously until the user decides to exit.
    loop {
        println!("Enter first number:");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let num1: f32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}Invalid number! Try again.{}", RED, RESET);
                continue;
            }
        };

        println!("Enter second number:");

        // Shadowing the input variable
        input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let num2: f32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}Invalid number! Try again.{}", RED, RESET);
                continue;
            }
        };

        println!("Choose operation (+, -, *, /):");

        // Shadowing the input variable
        input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let operation = input.trim();

        // Color code for each operation
        let op_color = match operation {
            "+" => BLUE,
            "-" => MAGENTA,
            "*" => YELLOW,
            "/" => GREEN,
            _ => RED,
        };

        let result = match operation {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 == 0.0 {
                    println!("{}Error: Division by zero!{}", RED, RESET);
                    continue;
                } else {
                    num1 / num2
                }
            }
            _ => {
                println!("{}Invalid operation!{}", RED, RESET);
                continue;
            }
        };

        println!("\n{} {}{}{} {} = {}{}{}", 
            num1, 
            op_color, operation, RESET, 
            num2, 
            GREEN, result, RESET
        );

        println!("\nDo you wish to perform another calculation? (Y/N)");
        input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        if input.trim().to_lowercase() != "y" {
            println!("{}Thank you for using Polkadot Kisumu Workshop Calculator!{}", CYAN, RESET);
            break;
        }
    }
}
