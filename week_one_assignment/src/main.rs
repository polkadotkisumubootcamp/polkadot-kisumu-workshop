use std:: env;

fn main() {
   
    // let mut args = std::env::args();
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("Usage: <num1> <operation> <num2>");
        return;
    }

}

// The calculator function takes two integers and a string representing an operation.
fn calculator(a: i32, b: i32, op : &str ) -> i32 {

    if op == "+" {
        return a + b;
    } else if op == "-" {
        return a - b;
    } else if op == "*" {
        return a * b;
    } else if op == "/" {
        if b != 0 {
            return a / b;
        } else {
            panic!("Division by zero is not allowed.");
        }
    } else if op == "%" {
        if b != 0 {
            return a % b;
        } else {
            panic!("Division by zero is not allowed.");
        }
        } else {
        panic!("Unsupported operation: {}", op);
    }   
}