use std::io;
use colored::*;

fn main() {
    println!();
    println!("{}", "⭐⭐⭐🖩CalcOnchain🖩⭐⭐⭐".bright_yellow().bold());
    println!();

    println!("{}", "first number: ".to_string().blue());
    let mut num1 = String::new();
    io::stdin()
        .read_line(&mut num1)
        .expect("Failed to read line");
    let first: f32 = num1.trim().parse().expect("Please enter a valid number");

    println!("{}", "second number: ".to_string().green());
    let mut num2 = String::new();
    io::stdin()
        .read_line(&mut num2)
        .expect("Failed to read line");
    let second: f32 = num2.trim().parse().expect("Please enter a valid number");

    println!("Enter operation: ");
    let mut operation = String::new();
    io::stdin()
        .read_line(&mut operation)
        .expect("Failed to read line");
    let operation = operation.trim();

    let _ans: f32 = 0.0;

    if operation == "+" {
        let ans = add(first, second);
        println!("{} + {} = {}", first.to_string().blue(), second.to_string().green(), ans.to_string().bright_yellow().bold());
    } else if operation == "-" {
        let ans = sub(first, second);  
        println!("{} - {} = {}", first.to_string().blue(), second.to_string().green(), ans.to_string().bright_yellow().bold());
    } else if operation == "*" {
        let ans = mul(first, second);  
        println!("{} * {} = {}", first.to_string().blue(), second.to_string().green(), ans.to_string().bright_yellow().bold());
    } else if operation == "/" {
        let ans = div(first, second);  
        println!("{} / {} = {}", first.to_string().blue(), second.to_string().green(), ans.to_string().bright_yellow().bold());
    } else if operation == "%" {
        let ans = modu(first, second);  
        println!("{} % {} = {}", first.to_string().blue(), second.to_string().green(), ans.to_string().bright_yellow().bold());
    } else {
        println!("{}", format!("Invalid operation: {}", operation).bright_red().bold());
        return;
    }
}

fn add(a: f32, b: f32) -> f32 {
    a + b
}

fn sub(a: f32, b: f32) -> f32 {
    a - b
}

fn mul(a: f32, b: f32) -> f32 {
    if (a == 0.0) || (b == 0.0) {
        return 1.0
    }
    a * b
}

fn div(a: f32, b: f32) -> f32 {
    a / b
}

fn modu(a: f32, b: f32) -> f32 {
    a % b
}