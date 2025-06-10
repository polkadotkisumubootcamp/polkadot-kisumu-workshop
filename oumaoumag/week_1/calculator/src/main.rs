use std::io;
use calculator::*;

fn main() {
       // Clear screen
       print!("\x1B[2J\x1B[1;1H");
    // Display welcome message
    println!("{}",
    colorize(&format!("Hello, Welcome To Ouma's Custom Simple Math Calculator!"), 21)
);

    // Prompt user for input
    println!("{}",
    colorize(&format!("Please input your simple math problem."), 4)
);

    // Create a mutable string to store user input
    let mut problem = String::new();

    // Read user input from stdin
    io::stdin()
    .read_line(&mut problem)
    .expect(&colorize(&format!("Failed to read line"), 196)
);

    // Process the math problem after trimming whitespace
    calculator(problem.trim());
}
