use std:: env;

// To run this calculator, you need three command line arguments:
// 1. The first number (num1)
// 2. The operator (op) which can be one of: +, -, *, /, %
// 3. The second number (num2) 
// Example usage: cargo run "10" "*" "5"

// The calculator function takes two integers and a string representing an operator.
fn calculator(a: f64, b: f64, op : &str ) -> f64 {

    if op == "+" {
        return a + b;
    } else if op == "-" {
        return a - b;
    } else if op == "*" {
        return a * b;
    } else if op == "/" {
        if b != 0.0 {
            return a / b;
        } else {
            panic!("Division by zero is not allowed.");
        }
    } else if op == "%" {
        if b != 0.0 {
            return a % b;
        } else {
            panic!("Division by zero is not allowed.");
        }
        } else {
        panic!("Unsupported operator: {}", op);
    }   
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        println!("Usage: <num1> <operator> <num2>");
        return;
    }

       let v1 = &args[1];
       let operator = &args[2];
        let v2 = &args[3];

        let num1 : f64 = v1.parse::<f64>().expect("Invalid number input for num1: {}");
        let num2 : f64 = v2.parse::<f64>().expect("Invalid number input for num2: {}");
        let result = calculator(num1, num2, operator);

        println!("The result is : {:.3}", result);
}