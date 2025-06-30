use rand::Rng;
use std::io;
use colored::*;
use chrono::Local;

fn main() {
    println!("{}", "Welcome to the Rust Guessing Game!".bold().cyan());

    let now = Local::now();
    println!("{}", format!("Current time: {}", now.format("%Y-%m-%d %H:%M:%S")).dimmed());

    let secret_number = rand::thread_rng().gen_range(1..=100);
    

    loop {
        println!("\n{}", "Please input your guess:".yellow());

        let mut guess = String::new();

        match io::stdin().read_line(&mut guess) {
            Ok(_) => {
                let guess: Option<u32> = guess.trim().parse::<u32>().ok();

                match guess {
                    Some(num) if num >= 1 && num <= 100 => {
                        println!("You guessed: {}", num);

                        if num < secret_number {
                            println!("{}", "Too small!".blue());
                        } else if num > secret_number {
                            println!("{}", "Too big!".magenta());
                        } else {
                            println!("{}", "You win!".green().bold());
                            break;
                        }
                    }
                    Some(_) => {
                        println!("{}", "Please enter a number between 1 and 100.".red());
                    }
                    None => {
                        println!("{}", "Invalid input! Please type a number.".red());
                    }
                }
            }
            Err(e) => {
                println!("{}", format!("Failed to read input: {}", e).red());
                break;
            }
        }
    }
}
