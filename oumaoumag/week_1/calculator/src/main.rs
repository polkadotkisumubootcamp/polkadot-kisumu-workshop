use std::io;
use calculator::*;

fn main() {
    println!("{}",
    colorize(&format!("Hello, Welcome To Ouma's Custom Simple Math Calculator!"), 21)
);

    println!("{}",
    colorize(&format!("Please input your simple math problem."), 4)
);

    let mut problem = String::new();

    io::stdin()
    .read_line(&mut problem)
    .expect(&colorize(&format!("Failed to read line"), 196)
);

     calculator(problem.trim());
}
