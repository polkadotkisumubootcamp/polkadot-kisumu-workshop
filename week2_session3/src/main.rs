use std::io::{self, Write};

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    // flush to ensure print! macro displays before input
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_lowercase()
}

// Function to match weather and provide comment
fn get_weather_comment(weather: &str) -> String {
    match weather {
        "sunny" => "It's a beautiful day with clear skies!".to_string(),
        "rainy" => "Don't forget your umbrella!".to_string(),
        "cloudy" => "A bit gloomy, but no rain yet.".to_string(),
        "snowy" => "It's snowy — stay warm and drive safe.".to_string(),
        _ => format!("Hmm, I don't recognize '{}', but stay safe out there!", weather),
    }
}

// Suggest clothing based on temperature
fn get_clothing_suggestion(temp: i32) -> String {
    if temp <= 5 {
        "Wear a heavy jacket.".to_string()
    } else if temp <= 15 {
        "A light jacket should be fine.".to_string()
    } else if temp <= 25 {
        "T-shirt weather!".to_string()
    } else {
        "It's hot! Wear something light.".to_string()
    }
}

// Function that returns a tuple (clothing, comment)
fn weather_suggestion(temp: i32, weather: &str) -> (String, String) {
    let clothing = get_clothing_suggestion(temp);
    let comment = get_weather_comment(weather);
    (clothing, comment)
}

fn main() {
    loop {
        println!("\n--- Weather Suggestion Assistant ---");

        // Ask for temperature
        let temp_input = get_user_input("Enter the temperature (°C): ");
        let temp: i32 = match temp_input.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid temperature input. Please enter a number.");
                continue;
            }
        };

        // Ask for weather condition
        let weather = get_user_input("Enter the weather condition (sunny, rainy, cloudy, snowy): ");

        // Get suggestion
        let (clothing, comment) = weather_suggestion(temp, &weather);
        println!("\nClothing Suggestion: {}", clothing);
        println!("Weather Comment: {}", comment);

        // Ask to repeat
        let again = get_user_input("\nDo you want another suggestion? (yes/no): ");
        if again != "yes" {
            println!("Goodbye! Stay safe and dress well.");
            break;
        }
    }
}
