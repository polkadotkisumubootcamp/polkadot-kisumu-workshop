use std::io;

fn get_suggestion(temp: i32, weather: &str) -> (String, String) {
    // Clothing suggestion
    let clothing = if temp < 10 {
        "🧥 Wear a heavy jacket.".to_string()
    } else if temp < 20 {
        "🧶 Wear a sweater or light jacket.".to_string()
    } else {
        "👕 A T-shirt should be fine.".to_string()
    };

    // Weather comment
    let comment = match weather.to_lowercase().as_str() {
        "sunny" => "☀️ It's sunny outside. Don't forget sunglasses!".to_string(),
        "rainy" => "🌧️ Rainy day! Carry an umbrella.".to_string(),
        "cloudy" => "☁️ It's cloudy, but calm.".to_string(),
        "snowy" => "❄️ Snowy! Stay warm and drive safe.".to_string(),
        _ => "🤔 Unrecognized weather type. Stay prepared for anything.".to_string(),
    };

    (clothing, comment)
}

fn main() {
    println!("\x1b[36m🌤️ Weather Suggestion Assistant\x1b[0m");

    loop {
        // Get temperature
        println!("Enter the temperature (°C):");
        let mut temp_input = String::new();
        io::stdin().read_line(&mut temp_input).unwrap();
        let temp: i32 = match temp_input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("\x1b[31mInvalid number. Try again.\x1b[0m");
                continue;
            }
        };

        // Get weather condition
        println!("Enter the weather condition (e.g., sunny, rainy):");
        let mut weather_input = String::new();
        io::stdin().read_line(&mut weather_input).unwrap();
        let weather = weather_input.trim();

        // Get suggestion
        let (clothing, comment) = get_suggestion(temp, weather);

        // Output
        println!("\n\x1b[32mYour Suggestions:\x1b[0m");
        println!("{}", clothing);
        println!("{}", comment);

        // Ask if user wants another suggestion
        println!("\nWould you like another suggestion? (yes/no):");
        let mut again = String::new();
        io::stdin().read_line(&mut again).unwrap();
        if again.trim().to_lowercase() != "yes" {
            println!("\x1b[35mGoodbye! Stay safe and dress well!\x1b[0m");
            break;
        }
    }
}
