use std::io;

fn main() {
    loop {
        let temperature = get_temperature();
        let weather = get_weather_condition();
        let (clothing, comment) = get_suggestions(temperature, &weather);
        
        println!("Weather comment: {}", comment);
        println!("Clothing suggestion: {}", clothing);
        
        // Ask if user wants another suggestion
        if !ask_for_another() {
            break;
        }
    }
}

fn get_temperature() -> i32 {
    loop {
        println!("Please enter the temperature (a number):");
        
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
            
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };
    }
}

fn get_weather_condition() -> String {
    loop {
        println!("Please enter the weather condition (sunny, rainy, cloudy, snowy):");
        
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
            
        let weather = input.trim().to_lowercase();
        
        // Validate input
        match weather.as_str() {
            "sunny" | "rainy" | "cloudy" | "snowy" => return weather,
            _ => {
                println!("Unknown weather condition. Please try again.");
                continue;
            }
        }
    }
}

fn get_suggestions(temperature: i32, weather: &str) -> (String, String) {
    // Get clothing suggestion based on temperature
    let clothing = if temperature <= 0 {
        "Heavy winter coat, gloves, and hat".to_string()
    } else if temperature <= 10 {
        "Jacket and sweater".to_string()
    } else if temperature <= 20 {
        "Light jacket or long sleeves".to_string()
    } else {
        "T-shirt and shorts".to_string()
    };
    
    // Get weather comment using match
    let comment = match weather {
        "sunny" => "It's sunny outside! Don't forget sunscreen.".to_string(),
        "rainy" => "It's raining, better bring an umbrella!".to_string(),
        "cloudy" => "Cloudy skies today, but no rain expected.".to_string(),
        "snowy" => "Snow is falling! Be careful on slippery surfaces.".to_string(),
        _ => "Interesting weather we're having!".to_string(), 
    };
    
    (clothing, comment)
}

fn ask_for_another() -> bool {
    loop {
        println!("Would you like another suggestion? (yes/no)");
        
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
            
        match input.trim().to_lowercase().as_str() {
            "yes" | "y" => return true,
            "no" | "n" => return false,
            _ => {
                println!("Please answer 'yes' or 'no'");
                continue;
            }
        }
    }
}