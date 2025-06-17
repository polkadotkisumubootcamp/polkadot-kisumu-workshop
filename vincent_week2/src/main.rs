use std::io::{self, Write};

fn main() {
    println!("🧳 Welcome to the Travel Clothing Advisor!");

    let temp: f64 = read_input("🌡️ Enter the current temperature at your destination in Celsius:")
        .trim()
        .parse()
        .expect("⚠️ Please enter a valid number.");

    println!("Temperature Recorded: {}°C", temp);

    let weather = read_input("🌤️ What's the weather like there? (e.g., sunny, rainy, cloudy, snowy):")
        .trim()
        .to_lowercase();

    let (suggestion, outfit) = travel_clothing_advisor(temp, &weather);
    println!("\n📝 Advice: {}\n👕 Recommended Outfit: {}", suggestion, outfit);
}

fn read_input(prompt: &str) -> String {
    print!("{} ", prompt);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input
}

fn travel_clothing_advisor(temp: f64, weather: &str) -> (&'static str, &'static str) {
    let suggestion = if temp < 0.0 {
        "It's extremely cold at your destination. Consider rescheduling if possible!"
    } else if temp < 10.0 {
        "Pack thermal wear and a heavy coat."
    } else if temp < 20.0 {
        "Mild weather — bring a light jacket or sweater."
    } else if temp < 30.0 {
        "Comfortable weather — casual wear is perfect."
    } else {
        "Hot destination! Bring breathable clothing and stay hydrated."
    };

    let outfit = match weather {
        "sunny" | "warm" => "Shorts and a T-shirt with sunglasses.",
        "rainy" => "Raincoat, umbrella, and waterproof shoes.",
        "cloudy" => "Long sleeves and a light jacket.",
        "snowy" => "Winter jacket, boots, gloves, and a beanie.",
        _ => "Weather unrecognized — pack flexible layers just in case.",
    };

    (suggestion, outfit)
}
