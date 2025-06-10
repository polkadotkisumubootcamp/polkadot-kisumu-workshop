use std::io;

// ANSI color codes struct
struct Colors {
    red: &'static str,
    green: &'static str,
    yellow: &'static str,
    blue: &'static str,
    magenta: &'static str,
    cyan: &'static str,
    reset: &'static str,
}

impl Colors {
    fn new() -> Self {
        Colors {
            red: "\x1b[31m",
            green: "\x1b[32m",
            yellow: "\x1b[33m",
            blue: "\x1b[34m",
            magenta: "\x1b[35m",
            cyan: "\x1b[36m",
            reset: "\x1b[0m",
        }
    }
}

const WELCOME_BANNER: &str = "
WELCOME TO POLKADOT KISUMU WORKSHOP CALCULATOR
";

fn get_user_input(prompt: &str, colors: &Colors) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

fn parse_number(input: &str, colors: &Colors) -> Result<f32, ()> {
    match input.parse() {
        Ok(num) => Ok(num),
        Err(_) => {
            println!("{}Invalid number! Try again.{}", colors.red, colors.reset);
            Err(())
        }
    }
}

fn calculate(num1: f32, num2: f32, operation: &str, colors: &Colors) -> Result<f32, ()> {
    match operation {
        "+" => Ok(num1 + num2),
        "-" => Ok(num1 - num2),
        "*" => Ok(num1 * num2),
        "/" => {
            if num2 == 0.0 {
                println!("{}Error: Division by zero!{}", colors.red, colors.reset);
                Err(())
            } else {
                Ok(num1 / num2)
            }
        }
        _ => {
            println!("{}Invalid operation!{}", colors.red, colors.reset);
            Err(())
        }
    }
}

fn get_operation_color(operation: &str, colors: &Colors) -> &str {
    match operation {
        "+" => colors.blue,
        "-" => colors.magenta,
        "*" => colors.yellow,
        "/" => colors.green,
        _ => colors.red,
    }
}

fn main() {
    let colors = Colors::new();

    println!("{}{}{}", colors.cyan, WELCOME_BANNER, colors.reset);

    // Main calculator loop
    loop {
        // Get first number
        let input = get_user_input("Enter first number:", &colors);
        let num1 = match parse_number(&input, &colors) {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Get second number
        let input = get_user_input("Enter second number:", &colors);
        let num2 = match parse_number(&input, &colors) {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Get operation
        let operation = get_user_input("Choose operation (+, -, *, /):", &colors);

        // Calculate result
        let result = match calculate(num1, num2, &operation, &colors) {
            Ok(result) => result,
            Err(_) => continue,
        };

        // Get color for operation
        let op_color = get_operation_color(&operation, &colors);

        // Display result
        println!(
            "\n{} {}{}{} {} = {}{}{}",
            num1,
            op_color, operation, colors.reset,
            num2,
            colors.green, result, colors.reset
        );

        // Ask to continue
        let continue_input = get_user_input("\nDo you wish to perform another calculation? (Y/N)", &colors);
        if continue_input.to_lowercase() != "y" {
            println!("{}Thank you for using Polkadot Kisumu Workshop Calculator!{}", colors.cyan, colors.reset);
            break;
        }
    }
}