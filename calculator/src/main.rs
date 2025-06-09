use std::io::{self, Write};

fn get_number(prompt: &str) -> f64 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number!"),
        }
    }
}

fn main() {

    const HELLO_BANNER: &str = "Welcome to the Kisumu Polkadot calculator!";
    println!("{}", HELLO_BANNER);
    
    loop {
        let num1 = get_number("Enter first number: ");
        let num2 = get_number("Enter second number: ");
        
        // get the operation
        print!("Enter operation (+, -, *, /): ");
        io::stdout().flush().unwrap();
        
        let mut operation = String::new();
        io::stdin().read_line(&mut operation).unwrap();
        
        // perform the operation based on the operation variable
        let result = match operation.trim() {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 == 0.0 {
                    println!("Error: Division by zero!");
                    continue;
                }
                num1 / num2
            },
            // if the operation is not valid, print an error message and return
            _ => {
                println!("Invalid operation!");
                continue;
            }
        };
        
        // print the result
        println!("Result: {}", result);
        
        print!("Do you want to perform another calculation? (y/n): ");
        io::stdout().flush().unwrap();
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        
        if choice.trim().to_lowercase() != "y" {
            println!("Thank you for using the calculator!");
            break;
        }
        println!(); // Add a blank line for better readability
    }
}
