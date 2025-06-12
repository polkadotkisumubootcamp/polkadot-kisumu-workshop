use std:: env;

fn main() {
   
    // let mut args = std::env::args();
    let args: Vec<String> = env::args().collect();
    //Confirms that we have three arguements. The first arguement(at index zero), is the program name, so it is not parsed.
    if args.len() != 4 {
        println!("Usage: <num1> <operator> <num2>");
        return;
    }

    // Parse the command line arguments from the second arguement, which is at index zero.
    // The first arguement is the first value, and should be an integer
    // The second arguement should be an operator. Always ensure that the multiplication "*" sign is inside quotation marks to avoid missinterpretation by cli
    // The third arguement should be the second value, which must also be a number. 
   let value1: i32 = args[1].parse().expect("Please provide a valid integer for num1.");
    let operator: &str = &args[2];
    let value2: i32 = args[3].parse().expect("Please provide a valid integer for num2.");
    println!("{:?}", args);

    let result = calculator(value1.into(), value2.into(), operator);
    println!("The result is: {}", result) //result
}

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