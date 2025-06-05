use std::thread;
use std::time::Duration;
use rand::Rng;

fn colorize(text: &str, color_code: u8) -> String {
    format!("\x1b[38;5;{}m{}\x1b[0m", color_code, text)
}

fn animate_text(text: &str) {
    let mut rng = rand::thread_rng();
    
    // Clear screen
    print!("\x1B[2J\x1B[1;1H");
    
    for char in text.chars() {
        let color = rng.gen_range(1..255);
        print!("{}", colorize(&char.to_string(), color));
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        thread::sleep(Duration::from_millis(100));
    }
    println!();
}

fn main() {
    let greetings = vec![
        "Habari ya Kisumu! 🌞",
        "Welcome to Lakeside! 🌊",
        "Kisumu ni Nyumbani! 🌈",
        "Silicon Savannah! 💻",
        "Hello Kisumu! 🎉"
    ];

    for greeting in greetings {
        animate_text(greeting);
        thread::sleep(Duration::from_millis(500));
    }
}