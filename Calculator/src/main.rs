use std::io;

fn main() {
    println!("Welcome to the Rust Calculator");

    loop {
        println!("Enter Your First Number: ");
        let mut num1 = String::new();
        io::stdin().read_line(&mut num1).unwrap();
        let num1: f64 = match num1.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        println!("Enter your Operation");
        let mut op = String::new();
        io::stdin().read_line(&mut op).unwrap();
        let op = op.trim();

        println!("Enter Your second Number:");
        let mut num2 = String::new();
        io::stdin().read_line(&mut num2).unwrap();
        let num2: f64 = match num2.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        // Calculate and print result
        let result = match op {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => num1 / num2,
            _ => {
                println!("Invalid Operation");
                continue;
            }
        };
        
        println!("Result: {}", result);

        // Ask if user wants to continue
        println!("\nWould you like to do another calculation? (yes/no): ");
        let mut continue_calc = String::new();
        io::stdin().read_line(&mut continue_calc).unwrap();
        let continue_calc = continue_calc.trim();
        
        if continue_calc.to_lowercase() != "yes" {
            println!("Thank you for using the calculator!");
            break;
        }
    }
}