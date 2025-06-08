use std::io;

const WELCOME_BANNER: &str = "
    WELCOME TO POLKADOT KISUMU WORKSHOP CALCULATOR
";

fn main() {
    println!("{}", WELCOME_BANNER);
    println!("Enter first number:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let num1: f32 = input.trim().parse().expect("Please enter a valid number");

    println!("Enter second number:");

    // Shadowing the input variable
    input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let num2: f32 = input.trim().parse().expect("Please enter a valid number");

    println!("Choose operation (+, -, *, /):");

    // Shadowing the input variable
    input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let operation = input.trim();

    let result = match operation {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0.0 {
                println!("Error: Division by zero!");
                return;
            } else {
                num1 / num2
            }
        }
        _ => {
            println!("Invalid operation!");
            return;
        }
    };

    println!("\n{} {} {} = {}", num1, operation, num2, result);
}
