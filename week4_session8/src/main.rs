use rand::Rng;
use std::io;
use colored::*;
use chrono::Local;

fn main() {
    println!("{}", "Welcome to the Guessing Game!".bold().cyan());
    println!(
        "{}",
        format!("Current time: {}", Local::now().format("%Y-%m-%d %H:%M:%S"))
            .dimmed()
    );

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attempts = 0;

    loop {
        println!("{}", "\nPlease input your guess (1-100):".yellow());

        let mut guess = String::new();

        match io::stdin().read_line(&mut guess) {
            Ok(_) => {
                let guess = guess.trim().parse::<u32>();

                match guess {
                    Ok(num) => {
                        attempts += 1;
                        println!("You guessed: {}", num);

                        if num < 1 || num > 100 {
                            println!("{}", "Please guess a number between 1 and 100.".red());
                            continue;
                        }

                        if num < secret_number {
                            println!("{}", "Too small!".blue());
                        } else if num > secret_number {
                            println!("{}", "Too big!".magenta());
                        } else {
                            println!("{}", "You win!".green().bold());
                            println!("Total attempts: {}", attempts);
                            break;
                        }
                    }
                    Err(_) => {
                        println!("{}", "Invalid number! Please enter a valid integer.".red());
                        continue;
                    }
                }
            }
            Err(e) => {
                println!(
                    "{}",
                    format!("Error reading input: {}", e).red().bold()
                );
                continue;
            }
        }
    }
}
