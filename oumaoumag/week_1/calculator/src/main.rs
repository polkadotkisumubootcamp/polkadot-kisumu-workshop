use std::io;

fn main() {
    println!("Hello, Welcome To Ouma's Custom Simple Math Calculator!");

    println!("Please input your simple math problem.");

    let mut problem = String::new();

    io::stdin()
    .read_line(&mut problem)
    .expect("Failed to read line");

    println!("You types in: {}", problem)
}
