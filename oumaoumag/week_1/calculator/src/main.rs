use std::io;
use std::thread;
use std::io::Write;
use std::time::Duration;
use rand::Rng;

use calculator::*;

// Constant Welcome Banner
const WELCOME_BANNER: &str = "Hello, Welcome To Ouma's Custom Simple Math Calculator!";

// Animate Text with Random Colors
fn animate_text(text: &str) {
    let mut rng = rand::thread_rng();

    // Clear screen
    print!("\x1B[2J\x1B[1;1H");
    
    for char in text.chars() {
        let color = rng.gen_range(1..=255);
        print!("{}", colorize(&char.to_string(), color));
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(50)); // Adjust speed here
    }
    println!();
}

fn main() {
    // Animate the welcome banner
    animate_text(WELCOME_BANNER);
    thread::sleep(Duration::from_millis(500)); // Pause before prompt

    // Prompt for input
    println!("{}", colorize("Please input your simple math problem.", 4));

    //  Read user input
    let mut problem = String::new();
    io::stdin()
        .read_line(&mut problem)
        .expect(&colorize("Failed to read line", 196));

    // Process input
    calculator(problem.trim());
}