use std::io;

fn main() {
    println!("Welcome to the Weather Suggestion System!");

    loop {
        // Get temperature input
        let temperature: f32 = loop {
            println!("Please enter the current temperature (a number):");
            
            let mut temp_input = String::new();
            io::stdin()
                .read_line(&mut temp_input)
                .expect("Failed to read line");
                
            match temp_input.trim().parse() {
                Ok(num) => break num,
                Err(_) => {
                    println!("That's not a valid number. Please try again.");
                    continue;
                }
            };
        };

        // Get weather condition input
        println!("Please enter the current weather condition (sunny, rainy, cloudy, snowy):");
        let mut weather_input = String::new();
        io::stdin()
            .read_line(&mut weather_input)
            .expect("Failed to read line");
        let weather = weather_input.trim().to_lowercase();

        // Get suggestions
        let (clothing, comment) = get_weather_suggestions(temperature, &weather);
        
        println!("\nWeather Suggestion:");
        println!("Temperature: {temperature}°C - {clothing}");
        println!("Condition: {weather} - {comment}\n");

        // Ask if user wants another suggestion
        println!("Would you like another suggestion? (yes/no)");
        let mut repeat_input = String::new();
        io::stdin()
            .read_line(&mut repeat_input)
            .expect("Failed to read line");
            
        if repeat_input.trim().to_lowercase() != "yes" {
            println!("Goodbye! Stay safe in the weather!");
            break;
        }
    }
}

fn get_weather_suggestions(temperature: f32, weather: &str) -> (String, String) {
    // Determine clothing based on temperature
    let clothing = if temperature < 0.0 {
        "Heavy winter coat, gloves, and hat".to_string()
    } else if temperature < 10.0 {
        "Jacket and sweater".to_string()
    } else if temperature < 20.0 {
        "Light jacket or sweater".to_string()
    } else {
        "T-shirt and shorts".to_string()
    };

    // Determine comment based on weather condition
    let comment = match weather {
        "sunny" => "Don't forget your sunscreen!",
        "rainy" => "Bring an umbrellan!",
        "cloudy" => "Might rain later, be prepared.",
        "snowy" => "Watch out for icy patches!",
        _ => "Unusual weather condition, be cautious!",
    }.to_string();

    (clothing, comment)
}