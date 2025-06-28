use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
use chrono::Local;

fn main() {
    // Generate random number between 1-100
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    // Print welcome message with current time
    let current_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    println!(
        "{}",
        format!("\nWelcome to the Guessing Game! (Current time: {})", current_time)
            .bright_blue()
            .bold()
    );
    println!("{}", "I'm thinking of a number between 1 and 100...".bright_white());

    loop {
        println!("\n{}", "Please input your guess:".bright_cyan());

        let mut guess = String::new();

        // Read user input with proper error handling
        match io::stdin().read_line(&mut guess) {
            Ok(_) => (),
            Err(e) => {
                println!(
                    "{} {}",
                    "Error reading input:".bright_red(),
                    e.to_string().bright_white()
                );
                continue;
            }
        }

        // Parse the input to a number with error handling
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!(
                    "{}",
                    "Please enter a valid number between 1 and 100!".bright_red()
                );
                continue;
            }
        };

        // Validate the number is in range
        if guess < 1 || guess > 100 {
            println!(
                "{}",
                "Please enter a number between 1 and 100!".bright_red()
            );
            continue;
        }

        println!("You guessed: {}", guess.to_string().bright_yellow());

        // Compare guess to secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".bright_red()),
            Ordering::Greater => println!("{}", "Too big!".bright_red()),
            Ordering::Equal => {
                println!(
                    "{}",
                    format!("You win! The number was {}.", secret_number)
                        .bright_green()
                        .bold()
                );
                break;
            }
        }
    }
}