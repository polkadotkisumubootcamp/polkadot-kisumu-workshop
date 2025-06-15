use std::io::{self, Write};

const CYAN: &str = "\x1b[36m";
const RESET: &str = "\x1b[0m";

fn main() {
    loop {
        println!("\nWelcome to weather suggestion assistant!");

        let temperature = get_user_input("Please enter temperature: ");
        let weather = get_weather_input("Enter weather condition: ");

        let (clothing, comment) = get_weather_advice(temperature, &weather);

        println!("\n{}Weather Suggestion Assistant Advice{}", CYAN, RESET);
        println!("Suggestion: {}", clothing);
        println!("Comment: {}", comment);

        if !continue_program() {
            break;
        }
    }
}

fn get_user_input(prompt: &str) -> f32 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse::<f32>() {
            Ok(value) => return value,
            Err(_) => println!("Please enter a valid number!"),
        }
    }
}

fn get_weather_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}

fn get_weather_advice(temp: f32, weather: &str) -> (String, String) {
    let base_clothing = match temp {
        t if t >= 30.0 => "Light clothing, shorts, sunhat",
        t if t >= 20.0 => "Light clothing, t-shirt",
        t if t >= 10.0 => "Light jacket, long pants",
        t if t >= 0.0 => "Warm coat, scarf",
        _ => "Heavy winter clothing, thermal wear",
    };

    let weather_addon = match weather.to_lowercase().as_str() {
        "rainy" => " + umbrella and raincoat",
        "sunny" => " + sunscreen and sunglasses",
        "cloudy" => " + light windbreaker",
        _ => "",
    };

    let comment = match (temp, weather.to_lowercase().as_str()) {
        (t, _) if t >= 35.0 => "Extreme heat warning!",
        (t, "rainy") if t < 10.0 => "Cold and wet, be extra careful!",
        (t, "sunny") if t > 25.0 => "Hot sunny day!",
        (_, "rainy") => "Carry an umbrella!",
        (t, _) if t < 0.0 => "Freezing conditions, stay warm!",
        _ => "Moderate weather conditions",
    };

    (
        format!("{}{}", base_clothing, weather_addon),
        comment.to_string(),
    )
}

fn continue_program() -> bool {
    print!("\nWould you like to continue? (y/n): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_lowercase().starts_with('y')
}
