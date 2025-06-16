use std::io;

fn main() {
    println!("Weather Suggestion Assistant");
    println!("============================");
    
    // Ask user for temperature
    println!("Enter the temperature (number):");
    let mut temp_input = String::new();
    io::stdin().read_line(&mut temp_input).expect("Failed to read input");
    let temperature: i32 = temp_input.trim().parse().expect("Please enter a valid number");
    
    // Ask user for weather condition
    println!("Enter the weather condition (sunny/rainy/cloudy/snowy):");
    let mut weather_input = String::new();
    io::stdin().read_line(&mut weather_input).expect("Failed to read input");
    let weather = weather_input.trim().to_lowercase();
    
    let (clothing_suggestion, weather_comment) = get_weather_suggestion(temperature, &weather);
    
    // Print the final suggestion
    println!("\n--- Weather Suggestion ---");
    println!("Temperature: {}°C", temperature);
    println!("Weather: {}", weather);
    println!("Clothing suggestion: {}", clothing_suggestion);
    println!("Weather comment: {}", weather_comment);
}

fn get_weather_suggestion(temperature: i32, weather: &str) -> (String, String) {
 //suggest clothes based on temperature
    let clothing_suggestion = if temperature <= 10 {
        "Heavy jacket".to_string()
    } else if temperature <= 20 {
        "Light jacket".to_string()
    } else {
        "T-shirt".to_string()
    };

    let weather_comment = match weather {
        "sunny" => "Great day for outdoor activities!".to_string(),
        "rainy" => "Don't forget your umbrella!".to_string(),
        "cloudy" => "Perfect weather for a walk.".to_string(),
        "snowy" => "Stay warm and watch for slippery roads!".to_string(),
        _ => "Unknown weather condition, stay prepared!".to_string(),
    };

    (clothing_suggestion, weather_comment)
}
