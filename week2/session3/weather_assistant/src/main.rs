use std::io::{self, Write};

fn main() {
    loop {
        // Read temperature
        let temp_input = read_input("Enter the temperature (°C): ");
        let temp: i32 = match temp_input.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("ERROR: Please enter a valid number for temperature.\n");
                continue;
            }
        };

        // Read weather condition
        let condition = read_input("Enter the weather condition (sunny, rainy, cloudy, snowy): ");

       
        let (clothing, comment) = get_weather_suggestion(temp, &condition);

        // Output the result
        println!("\nSuggestion:");
        println!("{}", comment);
        println!("{}", clothing);

        // Ask user to continue
        let again = read_input("\nDo you want another suggestion? (yes/no): ");
        if again.to_lowercase() != "yes" {
            println!("Goodbye!");
            break;
        }
        println!(); 
    }
}


fn get_weather_suggestion(temp: i32, condition: &str) -> (String, String) {
    // Match weather condition to generate a comment
    let weather_comment = match condition.to_lowercase().as_str() {
        "sunny" => "It's bright and sunny outside!".to_string(),
        "rainy" => "Don't forget your umbrella!".to_string(),
        "cloudy" => "It might be a gloomy day.".to_string(),
        "snowy" => "It's snowy—watch your step!".to_string(),
        _ => format!("'{}' is not a typical weather type. Stay prepared for anything!", condition),
    };

    // determine clothing
    let clothing_suggestion = if temp <= 10 {
        "Wear a heavy jacket.".to_string()
    } else if temp <= 20 {
        "A light sweater should be fine.".to_string()
    } else if temp <= 30 {
        "A T-shirt is perfect.".to_string()
    } else {
        "Stay cool with shorts and a tank top.".to_string()
    };

    (clothing_suggestion, weather_comment)
}

fn read_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    // flush to make sure prompt is shown before input
    io::stdout().flush().unwrap(); 
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}


