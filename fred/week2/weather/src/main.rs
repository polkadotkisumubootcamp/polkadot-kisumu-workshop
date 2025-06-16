use std::io::{self, Write};

fn main() {
    // Get user input
    let temperature = get_temperature();
    let weather = get_weather();

    // Call function
    let (clothing, comment) = suggest_clothing_and_comment(temperature, &weather);

    // Output results
    println!("Weather comment: {}", comment);
    println!("Suggested clothing: {}", clothing);
}

// Get temperature from user
fn get_temperature() -> i32 {
    loop {
        print!("Enter the temperature (in Celsius): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<i32>() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number!"),
        }
    }
}

// Get weather condition from user
fn get_weather() -> String {
    loop {
        print!("Enter the weather condition (sunny, rain, cloudy, snowy): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let condition = input.trim().to_lowercase();

        match condition.as_str() {
            "sunny" | "rain" | "cloudy" | "snowy" => return condition,
            _ => println!("Please enter one of: sunny, rain, cloudy, snowy."),
        }
    }
}

// Core logic
fn suggest_clothing_and_comment(temp: i32, weather: &str) -> (String, String) {
    // Match weather to comment
    let comment = match weather {
        "sunny" => "It's bright and sunny outside!",
        "rain" => "Don't forget your umbrella!",
        "cloudy" => "A bit gloomy, but no rain—yet.",
        "snowy" => "It's snowing! Watch your step.",
        _ => "Weather unknown.",
    }.to_string();

    // Clothing based on temperature
    let clothing = if temp < 15 {
        "Wear a heavy jacket."
    } else if temp < 25 {
        "A hoodie or sweater should be fine."
    } else {
        "T-shirt weather!"
    }.to_string();

    (clothing, comment)
}
