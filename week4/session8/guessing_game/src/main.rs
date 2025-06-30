use rand::Rng;
use std::io;
use colored::*;
use chrono::Local;

fn main() {
    println!("{}", "Welcome to the Rust Guessing Game!".bold().cyan());

    let now = Local::now();
    println!("{}", format!("Current time: {}", now.format("%Y-%m-%d %H:%M:%S")).dimmed());

    let secret_number = rand::thread_rng().gen_range(1..=100);
    


}
