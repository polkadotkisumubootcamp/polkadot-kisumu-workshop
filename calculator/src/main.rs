use std::io;

const WELCOME_BANNER: &str = "
    WELCOME TO POLKADOT KISUMU WORKSHOP CALCULATOR
";

fn main() {
    println!("{}", WELCOME_BANNER);

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
                println!("Invalid number! Try again.");
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
                println!("Invalid number! Try again.");
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

        let result = match operation {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 == 0.0 {
                    println!("Error: Division by zero!");
                    continue;
                } else {
                    num1 / num2
                }
            }
            _ => {
                println!("Invalid operation!");
                continue;
            }
        };

        println!("\n{} {} {} = {}", num1, operation, num2, result);

        println!("\nDo you wish to perform another calculation? (Y/N)");
        input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        if input.trim().to_lowercase() != "y" {
            println!("Thank you for using Polkadot Kisumu Workshop Calculator!");
            break;
        }
    }
}
