use std::io;

const BANNER: &str = "\x1b[36m🧮 Welcome to RustCalc!\x1b[0m";

fn main() {
    println!("{}", BANNER);

    // Get first number
    println!("Enter the first number:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).unwrap();
    let input1: f32 = input1.trim().parse().unwrap(); // shadowing

    // Get second number
    println!("Enter the second number:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).unwrap();
    let input2: f32 = input2.trim().parse().unwrap(); // shadowing

    // Get operation
    println!("Enter operation (+, -, *, /):");
    let mut op = String::new();
    io::stdin().read_line(&mut op).unwrap();
    let op = op.trim();

    // Calculate result
    let result = match op {
        "+" => input1 + input2,
        "-" => input1 - input2,
        "*" => input1 * input2,
        "/" => {
            if input2 == 0.0 {
                println!("\x1b[31mError: Division by zero!\x1b[0m");
                return;
            } else {
                input1 / input2
            }
        }
        _ => {
            println!("\x1b[31mInvalid operation\x1b[0m");
            return;
        }
    };

    println!("\x1b[32mResult: {}\x1b[0m", result);
}
