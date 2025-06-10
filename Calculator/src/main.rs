use std::io;
fn main(){
    println!("Welcome to the Rust Calculator");
    println!("Enter Your First Number: ");
    let mut num1= String::new();
    io::stdin().read_line(&mut num1).unwrap();
    let num1: f64 = num1.trim().parse().unwrap();

    println!("Enter your Operation");
    let mut op = String::new();
    io::stdin().read_line(&mut op).unwrap();
    let op = op.trim();

    println!("Enter Your second Number:");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).unwrap();
    let num2: f64 = num2.trim().parse().unwrap();

    //calculate and print result
    let result = match op {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => num1 / num2,
        _ => {
            println!("Invalid Operation");
            return;
        }
    };
    println!("Result: {}", result);
    println!("Try another arithmetic function");
}