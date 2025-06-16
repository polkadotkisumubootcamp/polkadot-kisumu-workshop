use std::io;

fn get_suggestion(temp: i32, weather: &str) -> (&'static str, &'static str) {
    let clothing = if temp < 10 {
        "Heavy Jacket"
    } else if temp < 20 {
        "Sweater"
    } else {
        "T-Shirt"
    };

    let comment = match weather {
        "sunny" => "It's a bright day!",
        "rainy" => "Don't forget an umbrella!",
        "cloudy" => "Looks gloomy today.",
        "snowy" => "Wear boots and gloves!",
        _ => "Weather condition unknown.",
    };

    (clothing, comment)
}

fn main() {
    let mut temp_input = String::new();
    println!("Enter temperature:");
    io::stdin().read_line(&mut temp_input).expect("Failed to read input");
    let temperature: i32 = temp_input.trim().parse().expect("Please enter a number");

    let mut weather_input = String::new();
    println!("Enter weather condition (sunny, rainy, cloudy, snowy):");
    io::stdin().read_line(&mut weather_input).expect("Failed to read input");
    let weather = weather_input.trim().to_lowercase();

    let (clothing, comment) = get_suggestion(temperature, &weather);

    println!("Clothing Suggestion: {}", clothing);
    println!("Weather Comment: {}", comment);
}
