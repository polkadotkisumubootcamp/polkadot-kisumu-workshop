use std::io::{self, Write};
use std::fmt;

const WELCOME_BANNER: &str = r#"
╔══════════════════════════════════════╗
║         Rusty Calculator v2.1        ║
╚══════════════════════════════════════╝
"#;

const MAX_NUMBER: f32 = 1e38;  
const MIN_NUMBER: f32 = -1e38; 

#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operation::Add => write!(f, "+"),
            Operation::Subtract => write!(f, "-"),
            Operation::Multiply => write!(f, "×"),
            Operation::Divide => write!(f, "÷"),
        }
    }
}

impl Operation {
    fn from_str(s: &str) -> Option<Self> {
        match s.trim() {
            "+" => Some(Operation::Add),
            "-" => Some(Operation::Subtract),
            "*" => Some(Operation::Multiply),
            "/" => Some(Operation::Divide),
            _ => None,
        }
    }

    fn calculate(&self, a: f32, b: f32) -> Result<f32, &'static str> {
        match self {
            Operation::Add => {
                if a.abs() > MAX_NUMBER || b.abs() > MAX_NUMBER {
                    Err("Numbers too large for calculation")
                } else {
                    Ok(a + b)
                }
            }
            Operation::Subtract => {
                if a.abs() > MAX_NUMBER || b.abs() > MAX_NUMBER {
                    Err("Numbers too large for calculation")
                } else {
                    Ok(a - b)
                }
            }
            Operation::Multiply => {
                if a.abs() > MAX_NUMBER || b.abs() > MAX_NUMBER {
                    Err("Numbers too large for calculation")
                } else {
                    Ok(a * b)
                }
            }
            Operation::Divide => {
                if b == 0.0 {
                    Err("Division by zero")
                } else if a.abs() > MAX_NUMBER || b.abs() > MAX_NUMBER {
                    Err("Numbers too large for calculation")
                } else {
                    Ok(a / b)
                }
            }
        }
    }
}

fn validate_number(num: f32) -> Result<f32, &'static str> {
    if num.is_infinite() {
        Err("Number is too large or too small")
    } else if num.is_nan() {
        Err("Invalid number (NaN)")
    } else if num.abs() > MAX_NUMBER {
        Err("Number exceeds maximum allowed value")
    } else if num.abs() < MIN_NUMBER {
        Err("Number is below minimum allowed value")
    } else {
        Ok(num)
    }
}

fn get_number(prompt: &str) -> f32 {
    loop {
        print!("\x1b[33m{}\x1b[0m", prompt);
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("\x1b[31mError reading input. Please try again.\x1b[0m");
            continue;
        }

        match input.trim().parse::<f32>() {
            Ok(num) => {
                match validate_number(num) {
                    Ok(valid_num) => return valid_num,
                    Err(e) => println!("\x1b[31m{}\x1b[0m", e),
                }
            }
            Err(_) => println!("\x1b[31mInvalid number. Please enter a valid number.\x1b[0m"),
        }
    }
}

fn get_operation() -> Operation {
    loop {
        print!("\x1b[33mEnter operation (+, -, *, /): \x1b[0m");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("\x1b[31mError reading input. Please try again.\x1b[0m");
            continue;
        }

        if let Some(op) = Operation::from_str(&input) {
            return op;
        }
        println!("\x1b[31mInvalid operation. Please use +, -, *, or /\x1b[0m");
    }
}

fn main() {
    println!("\x1b[36m{}\x1b[0m", WELCOME_BANNER);

    let num1 = get_number("Enter first number: ");
    let num2 = get_number("Enter second number: ");
    let operation = get_operation();

    match operation.calculate(num1, num2) {
        Ok(result) => {
            if result.is_infinite() {
                println!("\x1b[31mError: Result is too large or too small to represent\x1b[0m");
            } else if result.is_nan() {
                println!("\x1b[31mError: Result is undefined\x1b[0m");
            } else {
                println!("\x1b[32mResult: {} {} {} = {:.2}\x1b[0m", 
                    num1, operation, num2, result);
            }
        }
        Err(e) => {
            println!("\x1b[31mError: {}\x1b[0m", e);
        }
    }
}
