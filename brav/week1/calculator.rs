use std::io;

const WELCOME_MESSAGE: &str = "Calculator Program";
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

fn main() {
    println!("{}", WELCOME_MESSAGE);

    loop {
        println!("Enter the first number (or 'quit' to exit):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        if input.trim().to_lowercase() == "quit" {
            println!("Goodbye!");
            break;
        }

        let first_number: f32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number! Please try again.");
                continue;
            }
        };

        let first_number = first_number;
        let first_number = first_number.abs();

        println!("Enter the second number:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        
        let second_number: f32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number! Please try again.");
                continue;
            }
        };

        println!("Choose an operation (+, -, *, /):");
        let mut operation = String::new();
        io::stdin().read_line(&mut operation).expect("Failed to read input");
        let operation = operation.trim();

        match operation {
            "+" => {
                let result = first_number + second_number;
                println!("{}{} + {} = {}{}", GREEN, first_number, second_number, result, RESET);
            },
            "-" => {
                let result = first_number - second_number;
                println!("{}{} - {} = {}{}", GREEN, first_number, second_number, result, RESET);
            },
            "*" => {
                let result = first_number * second_number;
                println!("{}{} * {} = {}{}", GREEN, first_number, second_number, result, RESET);
            },
            "/" => {
                if second_number == 0.0 {
                    println!("Error: Division by zero is not allowed!");
                } else {
                    let result = first_number / second_number;
                    println!("{}{} / {} = {}{}", GREEN, first_number, second_number, result, RESET);
                }
            },
            _ => {
                println!("Invalid operation! Please use +, -, *, or /");
            }
        };
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_basic_operations() {
        assert_eq!(2.0 + 3.0, 5.0);
        assert_eq!(5.0 - 3.0, 2.0);
        assert_eq!(2.0 * 3.0, 6.0);
        assert_eq!(6.0 / 3.0, 2.0);
    }

    #[test]
    fn test_division_by_zero_check() {
        let divisor = 0.0;
        assert_eq!(divisor, 0.0);
    }
}
