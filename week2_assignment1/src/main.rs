fn main() {

    println!("Welcome to the Weather Suggestion Assistant!");
    println!("Please enter the current temperature in Celsius:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let temp: f64 = input.trim().parse().expect("Please enter a valid number");
    println!("Temprature Comment: {}", temp);

    println!("Please enter the current weather condition (e.g., sunny, rainy, cloudy, snowy):");
    let mut weather_input = String::new();
    std::io::stdin().read_line(&mut weather_input).expect("invalid input");
    let weather = weather_input.trim().to_lowercase();

    let result = weather_suggestion_assistant(temp, &weather);
    println!("Suggestion: {}, Outfit: {}", result.0, result.1);
}

fn weather_suggestion_assistant(temp: f64, weather: &str) -> (&str, &str) {
 let mut result = ("", "");

    if temp < 0.0 {
        result.0 = "Stay indoors, it's freezing!";
    }
    else if temp < 10.0 {
        result.0 = "Wear a warm coat and stay warm.";
    }
    else if temp < 20.0 {
        result.0 = "A light jacket should be fine.";
    }
    else if temp < 30.0 {
        result.0 = "It's warm, wear comfortable clothes.";
    }
    else {
        result.0 = "It's hot, stay hydrated and wear light clothing.";
    }

    match weather {
        "sunny" | "Warm" => {
            result.1 = "T-shirt";
        },
        "rainy" => {
            result.1 = "Don't forget your umbrella and waterproof jacket.";
        },
        "cloudy" => {
            result.1 = "A light jacket should be fine.";
        },
        "snowy" => {
            result.1 = "Wear snow boots and a heavy coat.";
        },
        _ => {
            result.1 = "Weather condition not recognized, dress appropriately.";
        }
    }
 
    result
}