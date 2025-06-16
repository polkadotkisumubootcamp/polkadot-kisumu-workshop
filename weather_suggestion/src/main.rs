use std::io::{self, Write};

fn main() {
    loop {
        println!("Enter the current temperature in Celsius:");
        let temp = read_number();
        
        println!("Enter weather condition (sunny, rainy, cloudy, snowy):");
        let weather = read_weather();
        
        let (clothing, comment) = get_suggestion(temp, &weather);
        
        println!("\n--- Weather Suggestion ---");
        println!("Clothing Suggestion: {}", clothing);
        println!("Weather Comment: {}\n", comment);
        
        println!("Would you like another suggestion? (yes/no):");
        if read_string().to_lowercase() != "yes" {
            println!("Goodbye! Stay safe and dress well.");
            break;
        }
    }
}

fn read_number() -> i32 {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number."),
        }
    }
}

fn read_weather() -> String {
    let valid_conditions = ["sunny", "rainy", "cloudy", "snowy"];
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let weather = input.trim().to_lowercase();
        
        if valid_conditions.contains(&weather.as_str()) {
            return weather;
        } else {
            println!("Please enter a valid weather condition: sunny, rainy, cloudy, or snowy.");
        }
    }
}

fn read_string() -> String {
    print!("> ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn get_suggestion(temp: i32, weather: &str) -> (String, String) {
    let clothing = if temp <= 5 {
        "Wear a heavy jacket and scarf."
    } else if temp <= 15 {
        "Wear a light jacket."
    } else if temp <= 25 {
        "A t-shirt should be fine."
    } else {
        "It's hot! Wear light clothes."
    };

    let comment = match weather {
        "sunny" => "It's a bright and sunny day! Don't forget your sunglasses.",
        "rainy" => "It's raining. Carry an umbrella.",
        "cloudy" => "It's cloudy. Might be a gloomy day.",
        "snowy" => "Snow is falling. Stay warm and be careful.",
        _ => "Weather condition not recognized. Stay alert and dress accordingly.",
    };

    (clothing.to_string(), comment.to_string())
}
