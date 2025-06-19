use std::io::{self, Write};

fn main() {
    println!("🌤️ Weather Clothing Advisor");

    loop {
        let temperature = get_temperature();
        let weather_condition = get_weather_condition();

        let (clothing, comment) = get_recommendation(temperature, &weather_condition);

        println!("\nRecommendation:");
        println!("💬 {}", comment);
        println!("👕 {}", clothing);

        if !ask_to_continue() {
            println!("👋 Goodbye! Stay prepared for any weather!");
            break;
        }
    }
}

fn get_temperature() -> i32 {
    loop {
        print!("🌡️ Enter temperature in Celsius: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("❌ Please enter a valid number!"),
        }
    }
}

fn get_weather_condition() -> String {
    loop {
        println!("\n🌦️ Choose weather condition:");
        println!("1) Sunny\n2) Cloudy\n3) Rainy\n4) Snowy\n5) Windy");
        print!("➡️ Enter your choice (1-5): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim() {
            "1" => return "sunny".to_string(),
            "2" => return "cloudy".to_string(),
            "3" => return "rainy".to_string(),
            "4" => return "snowy".to_string(),
            "5" => return "windy".to_string(),
            _ => println!("❌ Invalid choice! Please enter 1-5"),
        }
    }
}

fn get_recommendation(temperature: i32, condition: &str) -> (String, String) {
    // Determine clothing based on temperature
    let clothing = if temperature < 0 {
        "Heavy winter coat, thermal layers, hat, scarf, and gloves".to_string()
    } else if temperature <= 10 {
        "Warm coat or jacket with sweater".to_string()
    } else if temperature <= 20 {
        "Light jacket or sweater".to_string()
    } else {
        "T-shirt and shorts or light clothing".to_string()
    };

    // Generate weather-specific comment
    let comment = match condition {
        "sunny" => "It's sunny outside! Don't forget sunglasses and sunscreen".to_string(),
        "rainy" => "Rain expected! Waterproof jacket and umbrella recommended".to_string(),
        "cloudy" => "Cloudy skies today. Layers are your best bet".to_string(),
        "snowy" => "Snowfall warning! Wear insulated waterproof boots".to_string(),
        "windy" => "Windy conditions! Windbreaker and secure hat advised".to_string(),
        _ => "Interesting weather pattern detected!".to_string(),
    };

    (clothing, comment)
}

fn ask_to_continue() -> bool {
    print!("\n🔄 Get another recommendation? (y/n): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().eq_ignore_ascii_case("y")
}

