use std::io;

fn main() {
    println!("Welcome to the Weather Suggestion Assistant!");

    loop {
        println!("\nWould you like a weather suggestion? (yes/no):");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim().to_lowercase().as_str() {
            "yes" | "y" => {
                let temp = get_temperature_input();
                let condition = get_weather_condition_input();
                let (clothing, comment) = get_suggestions(temp, &condition);
                
                println!("\nWeather Suggestions:");
                println!("- Condition: {}", comment);
                println!("- Recommended clothing: {}", clothing);
            },
            "no" | "n" => {
                println!("Goodbye!");
                break;
            },
            _ => println!("Please enter either 'yes' or 'no'"),
        }
    }
}

fn get_temperature_input() -> i32 {
    loop {
        println!("Please enter the current temperature (in degrees):");
        let mut temp = String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        match temp.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number (e.g., 25):"),
        }
    }
}

fn get_weather_condition_input() -> String {
    loop {
        println!("Please enter the weather condition (sunny, rainy, cloudy, snowy):");
        let mut condition = String::new();
        io::stdin()
            .read_line(&mut condition)
            .expect("Failed to read line");

        let condition = condition.trim().to_lowercase();
        if !condition.is_empty() {
            return condition;
        }
        println!("Please enter a valid weather condition");
    }
}

fn get_suggestions(temp: i32, condition: &str) -> (String, String) {
    let comment = match condition {
        "sunny" => "It's sunny outside! Perfect for outdoor activities.",
        "rainy" => "Don't forget your umbrella - it's wet outside!",
        "cloudy" => "Cloudy skies today, but no rain expected.",
        "snowy" => "Winter wonderland! Bundle up warm.",
        _ => "Interesting weather pattern today!",
    };

    let clothing = if temp < 0 {
        "Heavy winter coat, gloves, scarf, and hat"
    } else if temp <= 10 {
        "Winter coat and warm layers"
    } else if temp <= 20 {
        "Light jacket or sweater"
    } else if temp <= 30 {
        "T-shirt and light jacket"
    } else {
        "T-shirt and shorts - stay cool!"
    };

    (clothing.to_string(), comment.to_string())
}